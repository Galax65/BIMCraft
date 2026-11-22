# 02 — Lucidi e combinazioni di lucidi

## Come lavora un utente Archicad (comportamento atteso)

- Ogni elemento sta in **un solo lucido**. Ogni strumento ha un lucido predefinito nelle sue impostazioni (es. muri
  esterni → "Muri – Esterni"), che si può cambiare dall'Info Box.
- Un lucido ha: **visibile/nascosto**, **bloccato/sbloccato** (gli elementi si vedono ma non si selezionano né si
  modificano), **pieno/wireframe** nel 3D, e un **gruppo di intersezione** (numero): gli elementi si intersecano tra loro
  (muri con muri, muri con solai…) solo se i loro lucidi hanno lo stesso numero.
- Una **combinazione di lucidi** memorizza lo stato di tutti i lucidi con un nome (es. "01 Pianta architettonica",
  "02 Strutture", "03 Stato di fatto"). Le viste salvate ricordano la loro combinazione.
- Esiste un lucido di sistema sempre visibile, che non si può nascondere né eliminare.
- Nella finestra Impostazioni lucidi: elenco dei lucidi a sinistra/in alto, combinazioni accanto, con le icone dello stato.

## Cosa c'è in BIMCraft

Niente di equivalente: Revit, e quindi BIMCraft, lavora per **categorie**, **visibilità/grafica**, **filtri di vista** e
**template** (`graphics.rs`). Sono strumenti più rigidi: la categoria dipende dal tipo di oggetto, il lucido lo sceglie
l'utente.

## Proposta (nel core, valida per tutti i profili)

- **Lucido** = voce di libreria (`library.rs`, con id e GUID): `name`, `hidden`, `locked`, `wireframe`, `intersection`
  (intero). Il lucido di sistema esiste sempre.
- **Elemento → lucido**: un valore riservato tra i `params` dell'elemento, come fanno le fasi (`Phase Created`): così
  `modify`, copie, abachi e filtri lo raggiungono senza codice nuovo. Nome riservato, es. `Layer`, che nessun parametro di
  progetto o di famiglia può prendere (stessa regola di `phasing::CREATED`).
- **Combinazione** = voce di libreria: nome + stato di ogni lucido (per GUID).
- **Vista**: le sue `Graphics` tengono la combinazione (o gli stati sciolti). `Look::of` nasconde ciò che sta in un
  lucido nascosto, **prima** dei filtri e delle sostituzioni della vista.
- **Bloccato**: `Model::check` (o il punto unico dove si validano i comandi) rifiuta una modifica a un elemento in un
  lucido bloccato, con un errore che dice quale lucido sbloccare.
- **Strumenti**: ogni strumento ha un lucido predefinito tra le impostazioni della finestra; `Session::stamp` lo
  scrive sui nuovi elementi come fa con la fase.
- **IFC**: export come `IfcPresentationLayerAssignment` (IFC4, ISO 16739); import dalla stessa entità.
- **File**: campo nuovo nel modello → `FORMAT` + passo di `Model::migrate` (un progetto vecchio: tutti nel lucido di
  sistema). Va concordato con upstream prima.

## Passi

1. **L1** — Voce di libreria lucido + comando `layer {name, hidden?, locked?, ...}` + parametro riservato. Test: creo
   un lucido, metto un muro dentro, `elements` lo mostra.
2. **L2** — Nascosto: una pianta non disegna gli elementi di un lucido nascosto (test sui pixel del PNG, come upstream).
3. **L3** — Bloccato: `modify` su un elemento bloccato è rifiutato e il modello resta com'era.
4. **L4** — Combinazioni + vista che le ricorda; comando `layer_combination`.
5. **L5** — Lucido predefinito per strumento; UI: Info Box e finestra Impostazioni lucidi nel profilo Arc.
6. **L6** — IFC export/import.
7. **L7** — Wireframe nel 3D e gruppi di intersezione (legati alla [04](04-strutture-composte.md)).

## Criteri di accettazione

- Nascondo "Arredi" e la pianta non li mostra; cambio combinazione e ricompaiono.
- Blocco "Strutture": non riesco a spostare un pilastro, ma lo vedo.
- Un IFC esportato, aperto in un altro programma, mostra i lucidi come layer.
