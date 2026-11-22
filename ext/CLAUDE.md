# Regole per Claude Code sul fork Galax65/BIMCraft

Valgono **in aggiunta** al `CLAUDE.md` radice di upstream, che resta la legge del codice (mai crash, tutto è un comando,
id e GUID, test prima, il core senza interfaccia, niente valori normativi nel codice, clean-room). Se una regola qui
contraddice quella di upstream, vince upstream e me lo segnali.

## Chi è l'utente

Paolo, architetto in Friuli Venezia Giulia, utente esperto di **Archicad**, non programmatore. Parlagli in italiano,
spiega ogni passo in parole semplici, digli sempre cosa stai per fare e cosa deve verificare lui (per esempio aprire
l'app e provare una funzione). Le sue osservazioni su come si comporta Archicad hanno la precedenza sulle specifiche.

## Come si lavora

- **Piccoli passi verificabili.** Un passo = un test che fallisce, il codice che lo fa passare, i tre controlli verdi:
  ```sh
  cargo fmt --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  ```
  Durante il lavoro basta il crate toccato (`cargo test -p bimcraft-core <nome>`); il giro completo prima di ogni commit.
- **Piano prima del codice.** Prima di toccare `crates/core` o di iniziare una specifica, scrivi un piano breve (file
  toccati, comandi nuovi, test) e aspetta il mio ok.
- **Dove va il codice:** leggi [README.md](README.md). Contributi per upstream su `up/<funzione>` da `main`; lavori del
  fork in `ext/crates/` su `it/<funzione>`.
- **Archicad è un riferimento di comportamento, non una fonte da copiare.** Niente testi d'aiuto, icone, nomi di file o
  contenuti Graphisoft. Le parole dell'interfaccia sono termini tecnici comuni in italiano, passati da `lang::t` e dai `.po`.
- **Stato del lavoro:** a fine sessione aggiorna la sezione "Stato" di [PIANO.md](PIANO.md) (cosa è fatto, cosa manca).
- **Commit** in italiano, uno per passo verde. Push e PR solo quando te lo chiedo.

## Dove guardare

- Piano: [PIANO.md](PIANO.md). Specifiche: [specifiche/](specifiche/).
- Mappa dei moduli di upstream: il `CLAUDE.md` radice e `crates/ui-egui/CLAUDE.md`.
- Bozze non compilate: [bozze/](bozze/) (non fanno parte del workspace).
