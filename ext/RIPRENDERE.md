# Il messaggio per riprendere il lavoro

Da incollare all'inizio di ogni nuova sessione di Claude Code, aperta nella cartella del fork (vedi
[GUIDA-MAC.md](GUIDA-MAC.md)). Il primo messaggio della prima volta è invece in [PRIMO-PROMPT.md](PRIMO-PROMPT.md).

---

Ciao, sono Paolo. Riprendiamo il lavoro sul fork. Spiegami tutto in italiano e chiedimi conferma prima di cambiare
qualcosa, di fare commit o push.

1. Controlla di essere sul ramo `it/main` (se no, dimmelo e proponimi di passarci), poi leggi `ext/CLAUDE.md` e la
   sezione "Stato" di `ext/PIANO.md`.
2. Guarda su GitHub le PR aperte verso IPedrax/BIMCraft e la discussione #586 (sono elencate nello "Stato"): risposte,
   commenti, PR unite o chiuse, e i controlli della CI. Se una PR ha controlli rossi, leggi il log e dimmi perché.
3. Aggiorna `main` da upstream (solo avanzamento, `--ff-only`) e dimmi cosa è arrivato. Se upstream si è mosso, proponimi
   di unirlo in `it/main` e, per le PR aperte, se serve rimetterle in pari (attenzione ai file delle traduzioni:
   `cargo i18n update`).
4. Proponimi il prossimo passo secondo lo "Stato" (prima le voci "Da controllare" e "Da verificare"), con un piano breve,
   e aspetta il mio ok.
