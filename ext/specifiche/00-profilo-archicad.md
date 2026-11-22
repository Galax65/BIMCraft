# 00 — Il profilo d'interfaccia "Archicad"

Base comune delle specifiche 01–04. Da proporre a IPedrax come issue prima di scrivere codice.

> **Cambio di rotta (22/11/2026, decisione di Paolo):** niente terzo profilo `Arc`. Le stesse cose (barra delle
> impostazioni in alto, comandi di modifica in una barra orizzontale in alto, Cmd-T, Navigatore diviso per scopo) si
> propongono come **modifiche al profilo BIMCraft**, che ha già la palette strumenti, l'Ispettore e il Navigatore.
> Il resto di questa specifica vale ancora, letto come "profilo BIMCraft" dove dice "profilo Arc".

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

## Cosa è già stato detto upstream (PR #194, novembre 2026)

- L'Info Box in alto l'ha già proposto Paolo il 02/11/2026, con uno screenshot di Archicad e i gruppi da sinistra a
  destra (Principale, Lucido, Metodo geometria, Linea di riferimento, Struttura, Pianta e sezione, Piani collegati,
  offset e spessore). La #194 è stata chiusa senza unirla; le sue idee sono nel profilo BIMCraft (1a605a6d).
- **Obiezione di dansity:** un elemento può avere fino a 100 parametri (COBie, classificazioni), che una barra non
  contiene. **Risposta:** come in Archicad, l'Info Box mostra solo i parametri principali dello strumento; tutti gli
  altri stanno nelle Impostazioni strumento (Cmd-T) e nelle Proprietà, che restano. L'Info Box non sostituisce le
  Proprietà, le affianca.
- **Dove proporlo:** i manutentori hanno chiesto di discutere i layout dell'interfaccia nelle
  [Discussions](https://github.com/IPedrax/BIMCraft/discussions), non nelle PR. La proposta del profilo Arc si apre lì;
  il codice arriva dopo, una PR per passo.

## Regola dei nomi (da IDEAS.md di upstream)

Archicad si cita solo come fonte dell'idea ("visto in"); in BIMCraft una funzione si chiama per ciò che fa, mai con
un nome di Graphisoft: niente "Info Box", "Pet Palette", "Bacchetta magica", "Mappa progetto", né un profilo chiamato
"Archicad". Vale per il codice, per i testi inglesi e per le proposte; le parole italiane dell'interfaccia restano
termini tecnici comuni (glossario). "Arc" è solo il nome di lavoro di queste specifiche.

## Proposta pubblicata

[Discussione #586](https://github.com/IPedrax/BIMCraft/discussions/586), il 22/11/2026, nella categoria Ideas: seguito
della #219 (chiusa da IPedrax con "Built"), il terzo profilo con la barra delle impostazioni in alto, Cmd+T e il
Navigatore diviso per scopo; selezione, lucidi (collegati alla #427 di IPedrax) e priorità dei materiali come proposte
separate. Si aspetta la risposta di IPedrax prima di scrivere codice.

## Prima bozza della proposta (superata dalla #586)

> **An Archicad-like interface profile.** Following #190 and #194: a third `Profile` beside BIMCraft and Rev, looks only,
> same commands. It would host the Tools palette with an Info Box along the top, tool default settings (Cmd+T), and a
> Navigator (project map, view map, layout book, publisher sets) in place of the Project Browser.
>
> On dansity's point in #194 about elements with a hundred parameters: the Info Box would show only the main settings
> of the tool in hand or of the selection (as Archicad's does); every other parameter stays in Properties and in the
> tool settings dialog, so nothing is lost and Properties is unchanged.
>
> Archicad's selection (Shift+click to add and remove, Shift held with a tool to select without leaving it) changes
> what the mouse does, not only looks, so it would be a separate, opt-in selection scheme in Settings rather than part
> of the profile. Where the profile needs behaviour the core lacks (layers and layer combinations, building-material
> priorities) we would propose it as separate core PRs usable by every profile.
>
> Would you accept it in this shape? I can test every PR on macOS.
