# Piano di lavoro del fork

Obiettivo: un BIM open source che un utente **Archicad** possa usare senza riimparare tutto, per progetti residenziali e
piccoli interventi in Italia, con un ottimo IFC e quantità pronte per il computo. Base tecnica unica: IPedrax/BIMCraft
(Rust). Il nome definitivo si decide più avanti.

Ogni fase si chiude solo con i tre controlli verdi (`cargo fmt --check`, `cargo clippy ... -D warnings`,
`cargo test --workspace`) e con Paolo che prova la funzione nell'app.

## Fase 0 — Avvio sul Mac (una sessione)

- Installare Claude Code, clonare il fork, aggiungere il remote `upstream`, creare `it/main` ([GUIDA-MAC.md](GUIDA-MAC.md)).
- Prima compilazione e test completi **senza modifiche**: annotare tempi e test eventualmente rossi su macOS.
- Avviare l'app (`cargo run -p bimcraft --release`) e controllare la traduzione italiana.
- Rileggere la PR #194 (tavolozza strumenti + Info Box) e la PR #242 (traduzione): capire cosa è già stato accettato.

## Fase 1 — Profilo Archicad e Navigatore ([profilo](specifiche/00-profilo-archicad.md), [navigatore](specifiche/01-navigatore.md))

Prima la issue a IPedrax e il terzo profilo d'interfaccia (accanto a BIMCraft e Rev, che già esistono), poi il Navigatore.

Mappa progetto, Mappa viste, Book impaginazione, Set di pubblicazione, al posto del Project Browser quando il profilo
d'interfaccia è "Archicad". Soprattutto interfaccia: il motore ha già viste, template, tavole e pubblicazione.

## Fase 2 — Lucidi ([specifica](specifiche/02-lucidi.md))

Lucidi nativi (visibile/nascosto, bloccato), combinazioni di lucidi salvate nelle viste, lucido predefinito per strumento,
export IFC come `IfcPresentationLayerAssignment`. Poi wireframe per lucido e gruppi di intersezione.

## Fase 3 — Opzioni di progettazione ([specifica](specifiche/03-opzioni-progettazione.md))

Il motore le ha già (`phasing.rs`): serve il gestore in stile Archicad e la combinazione di opzioni scelta per vista.

## Fase 4 — Strutture composte e priorità dei materiali ([specifica](specifiche/04-strutture-composte.md))

Materiali da costruzione con priorità di intersezione e dati fisici; intersezione degli strati per priorità tra muri,
poi tra muri e solai. È la fase più delicata (geometria del core): si apre prima una issue a IPedrax.

## Fase 5 — Metodologie italiane

- Fasi nominate come in Italia (stato di fatto, demolizioni, nuove costruzioni) e rappresentazione giallo/rosso come
  stile di vista, non come regola fissa nel codice.
- Classificazione UNI 11337 come tabella caricabile (il Classification Manager esiste già; nessuna tabella distribuita).
- Riferimenti normativi (DPR 380/2001, D.Lgs. 36/2023, CAM) come dati in `data/codes/` con la fonte, mai nel codice.
- Unità e convenzioni grafiche italiane (quote, retini, penne) come standard tecnico selezionabile (`standards.rs`).

## Fase 6 — Quantità

Aree nette dei muri (tolte le aperture), quantità per strato e per materiale, quantità per fase.

## Fase 7 — Ponte con Quadra (per ultimo)

Contratto JSON versionato `bimcraft-quadra` e importatore in Quadra. I due progetti restano indipendenti fino a qui:
nessuno dei due importa il codice dell'altro, il contratto è l'unico punto di contatto. Una prima bozza non compilata
del lato BIMCraft è in [bozze/quadra](bozze/quadra/).

## Stato

| Fase | Stato | Note |
|---|---|---|
| 0 | da fare | documenti preparati il 22/11/2026 (sessione su claude.ai, senza compilare) |
| 1–7 | da fare | |
