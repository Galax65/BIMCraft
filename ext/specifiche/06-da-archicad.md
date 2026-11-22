# 06 — Portare un progetto da Archicad

Nota di Paolo (22/11/2026): aprire i file di Archicad; se non è possibile, un profilo di esportazione da Archicad
(modello, viste, layout).

## Perché non si apre un `.pln`

Il formato di Archicad non ha una specifica pubblica. Il `CLAUDE.md` di upstream ammette la lettura di un formato
senza specifica solo per Revit (`.rvt`, `.rfa`), nel recinto di `crates/rvt` e di [docs/rvt-provenance.md](../../docs/rvt-provenance.md),
perché porta codice già pubblicato (rvt-rs, Apache-2.0). Per Archicad non c'è niente di simile: leggerlo vorrebbe dire
ricostruire un formato chiuso, fuori dalle regole clean-room. Quindi no.

## La strada: esportare da Archicad ciò che BIMCraft sa leggere

Già proposta da Paolo nella #219 (punto "Coming from Archicad: export, not native files"); IPedrax, chiudendola, ha
lasciato aperto proprio questo: *"an Archicad-style IFC sidecar for views and sheets"*.

1. **Il modello in IFC4**, con un traduttore IFC di Archicad preparato per BIMCraft (quali classi, proprietà, quantità,
   strati, lucidi come `IfcPresentationLayerAssignment`). Paolo fornisce file di prova reali esportati da Archicad, per
   controllare l'import IFC di BIMCraft (`import.rs`, [IFC import](../../docs/areas/ifc-import.md)).
2. **Viste e layout in un file JSON a parte** ("sidecar"), scritto da uno script che gira **dentro Archicad** con la
   sua interfaccia di programmazione pubblica (JSON/Python API di Graphisoft): per ogni vista il tipo, il piano, la
   scala, la combinazione di lucidi, il ritaglio; per ogni layout il formato, i disegni posati e la loro posizione.
   Lo script è nostro e lo esegue l'utente sul suo progetto: niente contenuti Graphisoft nel repository.
3. **In BIMCraft un comando** (`import_sidecar` o un parametro di `import`) che, dopo l'IFC, crea le viste e i layout
   del sidecar sugli elementi importati, legati per GUID (`ids.rs`). Il formato del sidecar è versionato e
   documentato, come ogni file che BIMCraft legge (tollerante: `lenient.rs`).

## Da decidere

- Se il sidecar diventa un formato di BIMCraft (utile anche da altri programmi) o resta un ponte solo per Archicad:
  da chiedere a IPedrax, che l'ha lasciato aperto nella #219.
- Dove sta lo script per Archicad: nel fork (`ext/`) o upstream.
