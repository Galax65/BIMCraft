# 01 — Navigatore

## Come lavora un utente Archicad (comportamento atteso)

Una palette con quattro "mappe", che si scelgono con dei pulsanti in alto:

1. **Mappa progetto**: la struttura del modello, sempre aggiornata. Piani (dal più alto al più basso, con la quota),
   sezioni, prospetti, prospetti interni, dettagli, viste 3D, abachi ed elenchi, indici, informazioni. Doppio clic apre.
   Non si salvano impostazioni: è il punto di vista "grezzo".
2. **Mappa viste**: le **viste salvate**. Una vista = un punto di vista della mappa progetto + le sue impostazioni:
   combinazione di lucidi, scala, set di penne, opzioni di visualizzazione del modello, filtro ristrutturazione (fasi),
   sostituzioni grafiche, quotatura, zoom. Organizzate in cartelle a piacere. È ciò che si mette nelle tavole.
3. **Book impaginazione**: le tavole (layout) e i loro modelli (master), in cartelle/sottoinsiemi con numerazione.
4. **Set di pubblicazione**: elenchi di viste e tavole da stampare o esportare (PDF, DWG, IFC…), ciascuno con formato e
   cartella di destinazione.

Sotto la mappa, un pannello mostra le **impostazioni dell'elemento scelto** (per una vista: lucidi, scala, penne, fase,
opzioni), modificabili lì. L'**Organizer** affianca due mappe per trascinare le viste nelle tavole.

## Cosa c'è in BIMCraft

| Archicad | BIMCraft (core) | Note |
|---|---|---|
| Piani | livelli (`Level`) e le loro piante | quota in `elevation` |
| Sezioni, prospetti | `Section`, `Elevation` | |
| Viste 3D, abachi | vista 3D, `schedule` | |
| Impostazioni di una vista | `Graphics` della vista e **view template** (`graphics.rs`) | fase, filtro fasi, opzioni, visibilità |
| Tavole e master | `sheet`, title block (`titleblock.rs`) | |
| Set di pubblicazione | `print_set`, `publish` (`printset.rs`, `publish.rs`) | |
| Palette | `browser.rs` (Project Browser, schede All/Views/Schedules/Sheets…) | da affiancare, non sostituire |

La differenza di fondo: in Revit (e in BIMCraft) una vista *è* l'oggetto con le sue impostazioni; in Archicad la mappa
progetto e la mappa viste sono separate. Proposta: la **Mappa progetto** mostra i livelli, le sezioni, ecc. come
"punti di vista" (aprirli apre la vista base di quel livello); la **Mappa viste** mostra le viste del modello con il loro
template e le loro impostazioni, in cartelle. "Salva vista corrente" crea una vista con le impostazioni attive.

## Dove va la logica

Per la regola "la UI è sottile", l'albero delle quattro mappe si costruisce nel **core** (una funzione pura del modello,
più un comando in sola lettura, es. `navigator {map}`), testato lì; `ui-egui` lo disegna soltanto. Le cartelle della
mappa viste sono un dato del progetto (con id e GUID come ogni voce di libreria) → modifica del formato: `FORMAT` +
passo di `Model::migrate`, da concordare con upstream.

## Passi

1. **N1 — Mappa progetto in sola lettura** (core): comando `navigator {map: "project"}` → albero con piani in ordine di
   quota decrescente, poi sezioni, prospetti, 3D, abachi. Test nel core.
2. **N2 — Palette nel profilo Arc** (UI): quattro pulsanti, la mappa progetto disegnata, doppio clic apre la vista.
   Test egui_kittest.
3. **N3 — Mappa viste**: elenco delle viste con template e impostazioni; "salva vista corrente". Cartelle: solo dopo
   l'accordo con upstream sul formato.
4. **N4 — Pannello impostazioni** sotto la mappa (scala, fase, opzioni, poi combinazione di lucidi dalla fase 2).
5. **N5 — Book impaginazione e set di pubblicazione** sulle tavole e i print set esistenti; trascinamento vista → tavola.

## Criteri di accettazione (da far provare a Paolo)

- Su un progetto con 3 piani, la mappa progetto li mostra dall'alto con la quota; doppio clic apre la pianta.
- Una vista salvata conserva fase e scala; riaprendo il progetto è ancora lì.
- Il Project Browser negli altri profili funziona come prima (nessun test di upstream cambiato).
