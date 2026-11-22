# Modelli di progetto del fork

## `italiano.bimt`

Un progetto nuovo già impostato all'italiana. In BIMCraft: **File > Nuovo > Progetto > Sfoglia…** e scegli
`ext/modelli/italiano.bimt`; da quel momento compare nell'elenco dei modelli di Nuovo progetto. Il modello non viene mai
sovrascritto: il primo Salva chiede dove mettere il progetto.

Cosa contiene:

- **Lingua del progetto italiana**: i disegni dicono SU sulle scale e SCALA nei cartigli, gli abachi e le viste nuove nascono con nomi italiani.
- **Unità**: metri con due decimali (il modello resta in millimetri; si cambia in Gestisci > Unità di progetto).
- **Piani**: Piano interrato (−3,00), Piano terra (0,00), Piano primo (+3,00), Sottotetto (+6,00).
- **Fasi**: Stato di fatto e Stato di progetto. In BIMCraft, come in Revit, un elemento **nasce nella fase della vista in
  cui lo disegni** e **Demolisci lo demolisce nella fase della vista in cui clicchi**. Quindi:
  1. il **rilievo** si disegna nella pianta *Piano terra - Stato di fatto*;
  2. **Demolisci** si usa nella pianta *Piano terra* (progetto) o nel *Comparativo*, mai nello stato di fatto;
  3. il **nuovo** si disegna nella pianta *Piano terra*.
  Un elemento che nasce e si demolisce nella stessa fase è "temporaneo": il comparativo lo mostra in giallo, così l'errore
  si vede; Cmd+Z lo riporta com'era. Per lavorare come in Archicad (uno stato Esistente / Da demolire / Nuovo scelto in
  una palette, qualunque sia la vista) vedi la specifica [07](../specifiche/07-ristrutturazione.md).
- **Gialli e rossi**: nella tavola comparativa le demolizioni sono gialle, le nuove costruzioni rosse, l'esistente
  conservato in mezzatinta grigia.
- **Tre piante del piano terra**: *Piano terra* (stato di progetto), *Piano terra - Stato di fatto* e
  *Piano terra - Comparativo*. Per gli altri piani: Duplica vista, poi fase e filtro come queste.

## Come si rigenera

Il `.bimt` si ricava dallo script, che è la fonte da correggere:

```sh
cargo run -p bimcraft-cli -- run --script ext/modelli/italiano.script --save ext/modelli/italiano.bimt
```

Verificato il 22/11/2026 disegnando un rettangolo e un tramezzo nello stato di fatto, il tramezzo demolito e un muro
nuovo nello stato di progetto, ed esportando le tre piante in PNG (`export {format: png, view}`): stato di fatto con il
tramezzo, comparativo con l'esistente grigio, il tramezzo giallo e il muro nuovo rosso, progetto senza il tramezzo.
