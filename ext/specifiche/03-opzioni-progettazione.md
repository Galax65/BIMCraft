# 03 — Opzioni di progettazione

## Come lavora un utente Archicad (comportamento atteso)

- Il modello ha un **modello principale** e uno o più **set di opzioni**; ogni set ha più **opzioni** alternative
  (es. set "Scala" con opzioni "Scala A" e "Scala B").
- Un elemento sta nel modello principale oppure in una sola opzione. Si lavora su un'opzione attivandola.
- Una **combinazione di opzioni** dice, per ogni set, quale opzione mostrare. La si sceglie per vista (dalla mappa viste
  o dal pannello impostazioni della vista), così due viste mostrano due varianti affiancate su una tavola.
- Un **gestore** (palette o finestra) elenca set e opzioni, permette di crearli, rinominarli, attivarli; quando si
  decide, l'opzione scelta si **accetta** e diventa modello principale.

## Cosa c'è in BIMCraft (motore già pronto)

`phasing.rs` e `docs/areas/phasing.md`: `Model::option_sets` (ogni set ha `options` e `primary`), il valore riservato
`Design Option` tra i `params` dell'elemento, `Session::option` (l'opzione attiva, scritta sui nuovi elementi),
`activate`, le `Graphics` della vista con `options {set: option}` (la primaria se non dice altro), rinomina che porta con
sé elementi, viste e filtri, e "accetta" che tiene gli elementi della primaria e cancella gli altri. Export IFC in
`BIMCraft_Phasing`. Nella finestra lo strumento è nascosto sotto la scala "Team" (`presets.rs`).

## Cosa manca

1. **Combinazione con un nome**, riusabile da più viste (oggi ogni vista tiene la sua scelta sciolta). Proposta: voce di
   libreria "combinazione di opzioni" {nome, scelta per set}; una vista tiene la combinazione o la scelta sciolta.
   Modifica di formato → da concordare.
2. **Accettare un'opzione qualsiasi**, non solo la primaria (verificare cosa fa oggi `accept`; se serve, parametro
   `option`).
3. **Interfaccia nel profilo Arc**: gestore delle opzioni (set, opzioni, attiva, nuova, rinomina, accetta), indicatore
   dell'opzione attiva sempre visibile, scelta della combinazione nel pannello della vista (Navigatore, [01](01-navigatore.md)).
4. **Termini italiani** nei `.po`.

## Passi

1. **O1** — Leggere `phasing.rs` e i test esistenti; scrivere qui cosa fa esattamente `accept`. Nessun codice.
2. **O2** — Gestore nel profilo Arc sui comandi esistenti. Test egui_kittest: creo un set con due opzioni, ne attivo
   una, disegno un muro, il muro è nell'opzione.
3. **O3** — Combinazioni con nome (core) + scelta per vista.
4. **O4** — Accetta un'opzione qualsiasi, se manca.

## Criteri di accettazione

- Due viste della stessa pianta su una tavola mostrano "Scala A" e "Scala B".
- Accetto "Scala B": resta solo lei, nel modello principale, e l'undo torna indietro.
