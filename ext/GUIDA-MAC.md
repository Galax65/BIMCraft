# Primo avvio di Claude Code sul MacBook

Tempo: circa 30 minuti, più la prima compilazione (può superare i 20 minuti, solo la prima volta).
Fonte delle istruzioni di installazione: https://code.claude.com/docs/en/overview

## 1. Apri il Terminale

Finder → Applicazioni → Utility → **Terminale** (oppure Cmd+Spazio, scrivi "Terminale", Invio).
I comandi qui sotto si incollano uno alla volta e si confermano con Invio.

## 2. Strumenti di base (forse li hai già, avendo compilato la PR #194)

```sh
xcode-select --install     # se dice "already installed", va bene così
rustup update              # aggiorna Rust: il progetto vuole almeno la 1.95
git --version
```

Se è la prima volta che fai commit da questo Mac, dì a git chi sei:

```sh
git config --global user.name "Paolo ..."
git config --global user.email "la-tua-email-di-github"
```

## 3. Scarica il fork

```sh
mkdir -p ~/Progetti && cd ~/Progetti
git clone https://github.com/Galax65/BIMCraft.git
cd BIMCraft
```

Metti il file `0001-ext-piano-e-specifiche-del-fork.patch` (quello ricevuto in chat) nella cartella **Download**.

## 4. Installa Claude Code

```sh
curl -fsSL https://claude.ai/install.sh | bash
```

Non mostra l'avanzamento: aspetta che finisca. Poi **chiudi e riapri il Terminale** e verifica:

```sh
claude --version
```

Se dice che `claude` non esiste, vedi https://code.claude.com/docs/en/troubleshoot-install.md (di solito è il PATH).

Alternativa senza terminale: l'app desktop di Claude ha la scheda **Code**, che include Claude Code.
Per questo progetto conviene comunque il Terminale, perché i comandi di questa guida sono quelli.

## 5. Avvia Claude Code nella cartella del progetto

```sh
cd ~/Progetti/BIMCraft
claude
```

Al primo avvio ti chiede di accedere: usa lo stesso account di claude.ai.

## 6. Il primo messaggio

Apri `PRIMO-PROMPT.md` (dentro il pacchetto ricevuto) e incolla il testo tra le due righe orizzontali.

## Durante il lavoro: le cose da sapere

- **Permessi.** Claude Code chiede il permesso prima di eseguire comandi o modificare file. Leggi cosa propone:
  `cargo ...` e `git status/diff/log/commit` sono sicuri. Per `git push`, `rm` o comandi che non capisci, chiedi
  "cosa fa questo comando?" prima di accettare.
- **Interrompere:** tasto **Esc**. Lui si ferma e tu puoi correggere la direzione.
- **Ripartire pulito** tra una fase e l'altra: scrivi `/clear`. Le regole del fork le rilegge da `ext/CLAUDE.md`.
- **Aiuto:** `/help`.
- **Uscire:** `/exit` o Ctrl+C due volte. Per riprendere: `cd ~/Progetti/BIMCraft && claude`.
- **Provare l'app:** quando ti dice di provare, lui avvia `cargo run -p bimcraft --release`; chiudi la finestra
  dell'app quando hai finito e descrivigli cosa hai visto, con le parole che useresti in Archicad.

## Quando arriverà il momento di pubblicare su GitHub

Il push userà le tue credenziali GitHub. Se git le chiede, il modo più semplice è installare GitHub CLI
(`brew install gh`, richiede Homebrew) e fare `gh auth login`. Claude Code ti guiderà quando servirà.
