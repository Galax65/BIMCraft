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
| 0 | fatta il 22/11/2026 | macOS, Rust 1.99.1. `main` = `upstream/main` (81ebe7f2), remote `upstream` aggiunto, branch `it/main`. Controlli senza modifiche: fmt 6 s, clippy 1 min 19 s, test 24 min 52 s: 5423 passati, 0 falliti, 24 ignorati. PR #242 (italiano) accettata; la PR #194 è stata ripresa da upstream nel profilo BIMCraft (tavolozza strumenti, Inspector e Navigator: 1a605a6d, 29673574). `translations/ui/it.po`: 2226 voci su circa 4700 senza traduzione; "Project Browser" è tradotto "Navigatore di progetto", da rivedere quando nasce il Navigatore stile Archicad. |
| 1 | proposta a IPedrax ([#586](https://github.com/IPedrax/BIMCraft/discussions/586)), si aspetta la risposta; Paolo preferisce modifiche al profilo BIMCraft invece di un terzo profilo; sue note nella [05](specifiche/05-flusso-e-selezione.md) e portare i progetti da Archicad nella [06](specifiche/06-da-archicad.md) | Prima prova di Paolo nell'app: l'Inspector deve stare in alto (Info Box); Cmd-T deve aprire le impostazioni dello strumento; i lucidi mancano (Fase 2). Le palette oggi si agganciano solo a sinistra, a destra e in basso (`dock.rs`). Flusso di lavoro e selezione Archicad a confronto con BIMCraft: [05](specifiche/05-flusso-e-selezione.md); la selezione di Archicad (Shift+clic, Shift con uno strumento attivo) cambia il comportamento del mouse, non solo l'aspetto: va chiesta a IPedrax come schema di selezione. Issue per IPedrax non ancora pubblicata. |
| Traduzioni | proposte a IPedrax il 22/11/2026: [#584](https://github.com/IPedrax/BIMCraft/pull/584) (traduzione) e [#585](https://github.com/IPedrax/BIMCraft/pull/585) (94 nomi di comandi e strumenti resi traducibili, sopra la #584) | Glossario approvato ([glossario](specifiche/glossario.md)). Su `up/traduzione-it` (nato da `main`, unito in `it/main`): interfaccia 4708/4708, disegni e abachi 240/243 (restano bath, bwc, wc: parole chiave dei nomi delle zone, non testi). Nei disegni stampati il layout è la "Tavola"; *amendment* è "integrazione". 5423 test verdi. Le parole proprie di Archicad (Info Box, Navigatore) vanno in voci del profilo Arc, non nelle traduzioni comuni ai profili. **Prossimo:** rimettersi in pari con upstream (`cargo i18n update`, tradurre le voci nuove), push e PR verso IPedrax come seguito della #242; poi la issue del profilo Arc. Installare `gh` (manca anche Homebrew: pacchetto da cli.github.com). |
| Fase 5, primo passo | fatto il 22/11/2026 | Modello di progetto italiano in [modelli/](modelli/README.md): lingua, metri, piani, stato di fatto e di progetto, gialli e rossi, tre piante; verificato esportando le piante. Nomi italiani delle stanze per `load_points` proposti a IPedrax ([#588](https://github.com/IPedrax/BIMCraft/pull/588)), uniti in `it/main`. |
| Note per la Fase 5 | | `loads.rs` (`kind_of`) riconosce il tipo di zona (bagno, cucina, servizio…) solo da nomi portoghesi e inglesi: "Bagno" o "Cucina" non sono riconosciuti. Da rendere dati della normativa, con i nomi italiani. |
| 2–7 | da fare | |
