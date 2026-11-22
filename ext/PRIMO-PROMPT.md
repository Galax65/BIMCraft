# Il primo messaggio per Claude Code

Da incollare così com'è nella prima sessione, aperta nella cartella del fork (vedi [GUIDA-MAC.md](GUIDA-MAC.md)).
Sostituisci il percorso del file patch se l'hai salvato altrove.

---

Ciao, sono Paolo, architetto e utente Archicad, non programmatore. Questa è la mia copia del fork Galax65/BIMCraft
(base: IPedrax/BIMCraft, Rust). È la prima volta che uso Claude Code: spiegami ogni passo in italiano e chiedimi
conferma prima di comandi che cambiano qualcosa.

Fase 0 del piano, in quest'ordine:

1. Controlla gli strumenti: `git --version`, `rustc --version` (il workspace chiede almeno Rust 1.95; se serve proponi
   `rustup update`), `cargo --version`.
2. Configura i remote: `origin` deve essere Galax65/BIMCraft; aggiungi `upstream` = https://github.com/IPedrax/BIMCraft.git
   e fai `git fetch upstream`. Dimmi di quanti commit `main` del fork è indietro rispetto a `upstream/main`; se si può
   avanzare senza conflitti (`--ff-only`) proponimelo.
3. Crea il branch `it/main` da `main` e applica con `git am` il patch che ho scaricato:
   `~/Downloads/0001-ext-piano-e-specifiche-del-fork.patch`. Aggiunge solo la cartella `ext/` (nessun file di upstream).
4. Leggi `ext/CLAUDE.md`, `ext/README.md` e `ext/PIANO.md`: da qui in poi valgono come regole del fork insieme al
   `CLAUDE.md` radice.
5. Prima compilazione e test, **senza modificare niente**:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
   Avvisami che la prima volta ci vuole parecchio. Riassumimi il risultato e gli eventuali test rossi su macOS.
6. Avvia l'app (`cargo run -p bimcraft --release`) e dimmi cosa devo controllare (profili d'interfaccia, lingua italiana).
7. Aggiorna la sezione "Stato" di `ext/PIANO.md`, fai un commit e **non** fare push: lo decidiamo insieme.

Poi proponimi il primo passo della Fase 1 (`ext/specifiche/00-profilo-archicad.md`) con un piano breve, e aspetta il mio ok.
