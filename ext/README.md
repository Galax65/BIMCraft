# ext/ — le aggiunte del fork Galax65/BIMCraft

Questa cartella contiene **tutto ciò che il fork aggiunge** a [IPedrax/BIMCraft](https://github.com/IPedrax/BIMCraft)
(il progetto "upstream"). Il nome definitivo del programma si decide più avanti: per ora è "il fork".

```
ext/
├── README.md          ← questo file: cos'è ext/ e come si convive con upstream
├── CLAUDE.md          ← regole per Claude Code quando lavora sul fork
├── PIANO.md           ← il piano di lavoro, fase per fase
├── GUIDA-MAC.md       ← primo avvio di Claude Code sul MacBook
├── PRIMO-PROMPT.md    ← il messaggio da incollare nella prima sessione
├── specifiche/        ← una specifica per ogni funzione stile Archicad
└── bozze/             ← codice NON compilato, tenuto da parte (es. il ponte con Quadra)
```

## Due tipi di lavoro

1. **Contributi a upstream** (interfaccia stile Archicad, lucidi, priorità dei materiali…). Toccano per forza
   `crates/core` e `crates/ui-egui`, che sono di upstream. Si sviluppano su un branch nato da `upstream/main` e si
   propongono a IPedrax come pull request, una funzione alla volta. Se upstream li accetta, il fork non deve più
   mantenerli: è il modo più sicuro per non accumulare conflitti.
2. **Aggiunte solo del fork** (dati e norme italiane, prezzari, ponte con Quadra). Vivono qui in `ext/`, in crate
   propri sotto `ext/crates/`. L'unica riga da toccare in un file di upstream è `members` nel `Cargo.toml` radice
   (aggiungere `"ext/crates/*"`), e la si aggiunge solo quando nasce il primo crate.

> Perché non in `crates/`? Il test `crates/core/tests/agent_docs.rs` di upstream esige che ogni cartella di `crates/`
> abbia un `CLAUDE.md` citato nel `CLAUDE.md` radice: un crate lì obbligherebbe a modificare quel file a ogni aggiornamento.

## Branch

| Branch | A cosa serve |
|---|---|
| `main` | copia fedele di `upstream/main`. Non ci si lavora mai sopra. |
| `it/main` | integrazione del fork: `main` + `ext/` + i contributi non ancora accettati upstream. È quello che si usa. |
| `up/<funzione>` | un contributo per upstream, nato da `main` (es. `up/navigatore`). Diventa una PR verso IPedrax. |
| `it/<funzione>` | un lavoro solo del fork, nato da `it/main`. |

## Aggiornare da upstream (una volta a settimana, o prima di iniziare una funzione)

```sh
git fetch upstream
git switch main && git merge --ff-only upstream/main && git push origin main
git switch it/main && git merge main          # i conflitti, se ci sono, stanno quasi solo fuori da ext/
cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
git push origin it/main
```

Se un test di upstream fallisce dopo l'aggiornamento, prima si capisce perché (spesso upstream ha cambiato una regola),
poi si corregge **dalla nostra parte**: non si modificano i test di upstream per farli passare.
