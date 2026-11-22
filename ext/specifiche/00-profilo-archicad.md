# 00 — Il profilo d'interfaccia "Archicad"

Base comune delle specifiche 01–04. Da proporre a IPedrax come issue prima di scrivere codice.

## Cosa c'è già

`crates/ui-egui/src/style.rs` ha `enum Profile { BimCraft, Rev }`: due profili che **differiscono solo nell'aspetto**
(ogni funzione esiste in entrambi, mostrata a modo suo), scelti nelle impostazioni della finestra (`settings.json`, mai
nel progetto). Le scorciatoie sono le stesse nei due profili. Vedi `docs/areas/desktop.md` (paragrafo sui profili) e i
test `tests/profile.rs`, `tests/look.rs`.

## Cosa si aggiunge

Un terzo profilo, `Profile::Arc` (nome di lavoro), con:

- **Palette strumenti** a sinistra (la tavolozza della PR #194) e **Info Box** in alto con le impostazioni dello
  strumento attivo o della selezione.
- **Impostazioni strumento** (Ctrl+T in Archicad): la finestra dei valori predefiniti dello strumento attivo, che
  valgono per il prossimo elemento disegnato. Con una selezione, modifica gli elementi selezionati.
- **Navigatore** al posto del Project Browser ([01](01-navigatore.md)).
- **Lucidi** nell'Info Box e nelle impostazioni strumento ([02](02-lucidi.md)).
- Scorciatoie: restano quelle comuni; le differenze Archicad si aggiungono come **schema di scorciatoie** selezionabile,
  non cambiando quelle esistenti.

## Principi

1. **Stesso motore, stessi comandi.** Il profilo cambia come si presentano le cose, non cosa fanno. Ogni azione resta un
   comando di `Session::execute`; se al profilo Archicad serve un comportamento nuovo (es. i lucidi), quello nasce nel core
   e vale per tutti i profili.
2. **Termini.** Nell'interfaccia italiana si usano i termini tecnici comuni (piano, lucido, combinazione di lucidi,
   struttura composta, materiale da costruzione, opzione di progettazione), definiti nei `.po`. Niente testi Graphisoft.
3. **Test.** Ogni pezzo del profilo ha un test egui_kittest che lo apre e lo usa (clic, tasti), come quelli di Rev.

## Primo passo (piccolo)

1. Test: `Profile::ALL` contiene il nuovo profilo, la finestra si apre con esso e mostra la palette strumenti a sinistra.
2. Codice: il nuovo valore dell'enum, il suo nome, il layout di partenza (`Docking`), e la voce nelle impostazioni.
3. Verifica di Paolo: scegliere il profilo nelle impostazioni, riavviare, vedere la disposizione.

## Testo della issue per IPedrax (bozza, in inglese)

> **An Archicad-like interface profile.** Following #190 and #194: a third `Profile` beside BIMCraft and Rev, looks only,
> same commands. It would host the Tools palette with an Info Box, tool default settings, a Navigator (project map, view
> map, layout book, publisher sets) in place of the Project Browser. Where it needs behaviour the core lacks (layers and
> layer combinations, building-material priorities) we would propose it as separate core PRs usable by every profile.
> Would you accept it in this shape? I can test every PR on macOS.
