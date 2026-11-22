//! Connettore BIMCraft → Quadra.
//!
//! Esporta il modello aperto in una [`Esportazione`]: un contratto dati versionato ([`CONTRATTO`], [`VERSIONE`]) che Quadra
//! legge per creare o aggiornare un computo metrico. Il connettore **legge e basta**: passa da `Session::execute` con il
//! comando `elements` (sola lettura) e dai campi pubblici del modello, non tocca geometria, viste né rendering.
//!
//! Regole del contratto (dettagli in `ext/quadra/README.md`):
//! - ogni elemento porta `guid` (stabile: è la chiave con cui Quadra aggancia le voci) e `id` (leggibile, può cambiare);
//! - le unità sono quelle dichiarate in [`Unita`]: lunghezze in mm, aree in m², volumi in m³, pesi in kg;
//! - le misure sono quelle che BIMCraft calcola già (nessuna regola di computo qui dentro: quelle stanno in Quadra);
//! - un campo sconosciuto in lettura va ignorato; un cambio incompatibile alza la versione maggiore.

use std::collections::{BTreeMap, HashMap};

use bimcraft_core::engine::Session;
use serde::Serialize;
use serde_json::{Map, Value, json};

/// Il nome del contratto, scritto in ogni esportazione.
pub const CONTRATTO: &str = "bimcraft-quadra";
/// La versione del contratto (semver): la maggiore cambia solo con modifiche incompatibili.
pub const VERSIONE: &str = "0.1.0";
/// Quanti elementi chiedere al comando `elements` per pagina.
const PAGINA: u64 = 500;
/// Il massimo di elementi esportati: oltre, l'esportazione si rifiuta e lo dice, invece di tagliare in silenzio.
pub const MAX_ELEMENTI: usize = 1_000_000;

/// I nomi con cui BIMCraft tiene le fasi tra i parametri di un elemento (`phasing::CREATED`, `DEMOLISHED`).
const FASE_CREATA: &str = "Phase Created";
const FASE_DEMOLITA: &str = "Phase Demolished";

/// Categorie che non sono opere da computare (annotazioni, viste, riferimenti), anche quando dicono un livello.
const NON_COMPUTABILI: &[&str] = &[
    "level",
    "view",
    "sheet",
    "section",
    "elevation",
    "schedule",
    "dimension",
    "text",
    "tag",
    "detail_line",
    "filled_region",
    "grid",
    "reference_plane",
    "cad_import",
    "camera",
    "project",
];

/// Le misure che BIMCraft dà di un elemento e il nome che prendono nel contratto, unità comprese.
const MISURE: &[(&str, &str)] = &[
    ("length", "lunghezza_mm"),
    ("height", "altezza_mm"),
    ("thickness", "spessore_mm"),
    ("width", "larghezza_mm"),
    ("perimeter_m", "perimetro_m"),
    ("area_m2", "area_m2"),
    ("volume_m3", "volume_m3"),
    ("formwork_m2", "casseforme_m2"),
    ("rebar_kg", "acciaio_kg"),
];

/// Un'esportazione completa del modello, ciò che Quadra importa.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Esportazione {
    pub contratto: &'static str,
    pub versione: &'static str,
    pub generatore: Generatore,
    pub progetto: Progetto,
    pub livelli: Vec<Livello>,
    /// Le fasi del progetto, in ordine (es. stato di fatto, poi progetto).
    pub fasi: Vec<String>,
    pub elementi: Vec<Elemento>,
}

/// Chi ha scritto il file.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Generatore {
    pub nome: &'static str,
    pub versione: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Progetto {
    pub nome: String,
    pub unita: Unita,
}

/// Le unità di misura del contratto: fisse, così Quadra non deve indovinarle.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Unita {
    pub lunghezze: &'static str,
    pub aree: &'static str,
    pub volumi: &'static str,
    pub pesi: &'static str,
}

const UNITA: Unita = Unita { lunghezze: "mm", aree: "m2", volumi: "m3", pesi: "kg" };

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Livello {
    pub guid: String,
    pub id: u64,
    pub nome: String,
    pub quota_mm: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Elemento {
    /// La chiave stabile: non cambia mai, anche se l'elemento passa da un file a un altro.
    pub guid: String,
    /// Il numero leggibile (sei cifre): utile alle persone, può cambiare.
    pub id: u64,
    pub categoria: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livello_guid: Option<String>,
    /// La fase in cui è fatto: sempre presente (la prima fase quando l'elemento non ne dice una, quella dell'ospite per porte e
    /// finestre).
    pub fase_creazione: String,
    /// La fase in cui è demolito, se lo è.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fase_demolizione: Option<String>,
    /// Le misure calcolate da BIMCraft, con l'unità nel nome (`lunghezza_mm`, `area_m2`…).
    pub misure: BTreeMap<String, f64>,
    /// I parametri dell'elemento così come BIMCraft li tiene (parametri di progetto compresi), senza le fasi.
    pub parametri: Map<String, Value>,
    /// L'elemento che lo ospita (il muro di una porta), per GUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ospite_guid: Option<String>,
}

fn testo(v: &Value, chiave: &str) -> Option<String> {
    v.get(chiave).and_then(Value::as_str).map(str::to_string)
}

/// Tutti gli elementi del modello, letti a pagine dal comando `elements`.
fn righe(s: &mut Session) -> Result<Vec<Value>, String> {
    let mut tutte = Vec::new();
    let mut offset: u64 = 0;
    loop {
        let r = s.execute("elements", &json!({"limit": PAGINA, "offset": offset}))?;
        let pagina = r.get("elements").and_then(Value::as_array).cloned().unwrap_or_default();
        if pagina.is_empty() {
            break;
        }
        let lette = pagina.len() as u64;
        tutte.extend(pagina);
        if tutte.len() > MAX_ELEMENTI {
            return Err(format!("quadra: il modello ha più di {MAX_ELEMENTI} elementi, il massimo che il contratto esporta"));
        }
        offset += lette;
        if r.get("count").and_then(Value::as_u64).is_some_and(|n| offset >= n) {
            break;
        }
    }
    Ok(tutte)
}

fn livello(r: &Value) -> Option<Livello> {
    Some(Livello {
        guid: testo(r, "guid")?,
        id: r.get("id").and_then(Value::as_u64)?,
        nome: testo(r, "name").unwrap_or_default(),
        quota_mm: r.get("elevation").and_then(Value::as_f64).filter(|z| z.is_finite())?,
    })
}

/// Un elemento da computare, o nessuno se è un'annotazione, una vista o un elemento senza livello.
fn elemento(r: &Value) -> Option<Elemento> {
    let categoria = testo(r, "category")?;
    if NON_COMPUTABILI.contains(&categoria.as_str()) {
        return None;
    }
    let livello_guid = testo(r, "level");
    let ospite_guid = testo(r, "host");
    livello_guid.as_ref().or(ospite_guid.as_ref())?;
    let mut parametri = r.get("params").and_then(Value::as_object).cloned().unwrap_or_default();
    let fase = |p: &mut Map<String, Value>, k: &str| p.remove(k).and_then(|v| v.as_str().map(str::to_string));
    let fase_creazione = fase(&mut parametri, FASE_CREATA).unwrap_or_default();
    let fase_demolizione = fase(&mut parametri, FASE_DEMOLITA);
    let misure = MISURE
        .iter()
        .filter_map(|(da, a)| r.get(*da).and_then(Value::as_f64).filter(|x| x.is_finite()).map(|x| ((*a).to_string(), x)))
        .collect();
    Some(Elemento {
        guid: testo(r, "guid")?,
        id: r.get("id").and_then(Value::as_u64)?,
        categoria,
        tipo: testo(r, "type"),
        tipo_guid: testo(r, "type_guid"),
        livello_guid,
        fase_creazione,
        fase_demolizione,
        misure,
        parametri,
        ospite_guid,
    })
}

/// Le fasi che mancano: un elemento senza fase segue il suo ospite, poi la prima fase del progetto.
fn completa_fasi(elementi: &mut [Elemento], fasi: &[String]) {
    let prima = fasi.first().cloned().unwrap_or_default();
    let note: HashMap<String, String> = elementi
        .iter()
        .filter(|e| !e.fase_creazione.is_empty())
        .map(|e| (e.guid.clone(), e.fase_creazione.clone()))
        .collect();
    for e in elementi.iter_mut().filter(|e| e.fase_creazione.is_empty()) {
        e.fase_creazione = e.ospite_guid.as_ref().and_then(|g| note.get(g)).cloned().unwrap_or_else(|| prima.clone());
    }
}

/// Esporta il modello della sessione nel contratto. Legge e basta: il modello resta com'era.
pub fn esporta(s: &mut Session) -> Result<Esportazione, String> {
    let righe = righe(s)?;
    let fasi = s.model.phases.clone();
    let mut livelli: Vec<Livello> =
        righe.iter().filter(|r| r.get("category").and_then(Value::as_str) == Some("level")).filter_map(livello).collect();
    livelli.sort_by(|a, b| a.quota_mm.total_cmp(&b.quota_mm).then(a.id.cmp(&b.id)));
    let mut elementi: Vec<Elemento> = righe.iter().filter_map(elemento).collect();
    elementi.sort_by_key(|e| e.id);
    completa_fasi(&mut elementi, &fasi);
    Ok(Esportazione {
        contratto: CONTRATTO,
        versione: VERSIONE,
        generatore: Generatore { nome: "BIMCraft", versione: env!("CARGO_PKG_VERSION") },
        progetto: Progetto { nome: s.model.name.clone(), unita: UNITA },
        livelli,
        fasi,
        elementi,
    })
}

/// L'esportazione come JSON leggibile, il file che Quadra apre.
pub fn in_json(e: &Esportazione) -> Result<String, String> {
    serde_json::to_string_pretty(e).map_err(|err| format!("quadra: JSON non scritto: {err}"))
}
