# 07 — Ristrutturazione all'uso di Archicad

Nota di Paolo (22/11/2026): le fasi all'uso di Revit (un elemento nasce nella fase della vista, Demolisci demolisce nella
fase della vista) sono scomode; meglio il sistema di Archicad, con una palette per decidere con che stato si disegna e
per assegnarlo anche dopo. Archicad è il riferimento di comportamento; in BIMCraft le cose si chiamano per ciò che fanno.

## Cosa c'è già in BIMCraft

- **Fasi** (`phasing.rs`, [phasing](../../docs/areas/phasing.md)): ogni elemento ha `Phase Created` e, se demolito,
  `Phase Demolished`; i filtri delle fasi dicono come una vista mostra ogni stato (nuovo, esistente, demolito,
  temporaneo).
- **Il menu Ristrutturazione** nella barra di controllo di ogni vista (`BimApp::renovation_menu`): Tutto, Esistente,
  Demolizioni, Nuovo. È il filtro di ristrutturazione di Archicad.
- **Demolisci** (DW): demolisce nella fase della vista. Un elemento nato nella stessa fase diventa temporaneo.
- Un elemento nasce nella fase della vista in cui si disegna (`BimApp::made_in`), non in una scelta dell'utente.

## Cosa manca: la palette Ristrutturazione

Una palette (in ogni profilo) con:

1. **Lo stato con cui si disegna**: Esistente, Da demolire, Nuovo. Vale per ogni elemento creato da quel momento,
   in qualunque vista, finché non si cambia. Nessuna sorpresa dalla vista in cui si è.
2. **Assegna alla selezione**: lo stato scelto dato agli elementi selezionati, in un passo di annullamento.
3. **Il filtro della vista**: il menu Ristrutturazione di oggi, anche qui.

Sotto restano le fasi (servono all'IFC, agli abachi, ai filtri). Ogni stato è una coppia di fasi, in un progetto con
almeno due fasi (la prima lo stato di fatto, l'ultima lo stato di progetto):

| Stato | `Phase Created` | `Phase Demolished` |
|---|---|---|
| Esistente | la prima fase | nessuna |
| Da demolire | la prima fase | l'ultima fase |
| Nuovo | l'ultima fase | nessuna |

Con più di due fasi (cantiere a lotti) la palette lavora sulla fase della vista come "progetto" e sulla precedente come
"stato di fatto": da decidere con IPedrax.

## Nel motore

Oggi una sessione dà a ciò che crea solo la fase di creazione (`Session::phase`, `Session::stamp`). Per "Da demolire"
serve anche quella di demolizione: o un secondo campo della sessione, o un comando `renovation {status, ids?}` che
imposta le due fasi (con `ids` le assegna, senza le sceglie per ciò che si crea). Da proporre a IPedrax prima del codice:
le fasi sono un suo progetto.

## Comportamento confermato da Paolo (22/11/2026)

- Un elemento **Da demolire**: normale nello stato di fatto, giallo nel comparativo, assente nel progetto.
- Un elemento **Nuovo**: assente nello stato di fatto, rosso nel comparativo, normale nel progetto.
- **Porte e finestre hanno uno stato proprio**, indipendente dal muro: una finestra nuova in un muro esistente, una porta
  da demolire in un muro che resta. In BIMCraft una porta o finestra senza fase segue il muro; la palette le dà sempre
  la sua.
- **Lo stato per strato** (uno strato di un muro composto da demolire, uno nuovo, come un intonaco rifatto o un
  cappotto aggiunto): in Archicad non si può, ed è una delle richieste più frequenti degli utenti. In BIMCraft gli strati
  sono già parti con il loro id e GUID (`HasPart`, `library.rs`), quindi potrebbero portare uno stato: proposta per dopo,
  da segnalare a IPedrax come punto in cui BIMCraft può fare meglio.

Proposta a IPedrax: commento alla [#586](https://github.com/IPedrax/BIMCraft/discussions/586), il 22/11/2026.

## Nel frattempo

Il [modello italiano](../modelli/README.md) spiega il procedimento con le tre piante, e il comparativo mostra in giallo
anche gli elementi "temporanei", così un errore si vede.

## Piano di lavoro (approvato da Paolo il 22/11/2026)

Decisioni: si parte da **"Segue la vista"** (il comportamento di oggi; la palette cambia qualcosa solo quando si sceglie
uno stato); con **più di due fasi** "Nuovo" è la fase della vista ed "Esistente" quella prima (con due fasi: la prima e
l'ultima); il lavoro va su **`up/palette-ristrutturazione`** da `main`, PR solo quando Paolo lo chiede. Lo stato per
strato resta per dopo.

1. **Motore (`crates/core`).** La sessione ricorda anche la fase in cui viene demolito ciò che si crea (un campo accanto
   a `Session::phase`, applicato da `Session::stamp` in `engine.rs`; Co-Work lo porta in `cowork.rs` come `phase` e
   `option`). Comando nuovo `renovation`: `{status}` fissa lo stato con cui si disegna (`existing`, `demolish`, `new`,
   o null per seguire la vista), `{status, ids}` lo dà a quegli elementi in un passo di annullamento (porte e finestre
   con una fase propria), senza parametri dice lo stato attuale e quello degli elementi; rifiutato con un messaggio
   chiaro in un progetto di una fase sola. Test prima: muro disegnato "Da demolire" con le due fasi; Nuovo dato a muro,
   porta e finestra e annullato in un passo; rifiuto con una fase sola; payload nel giro dei parametri ostili
   (`hostile_params_never_panic_or_break_the_model`). Un paragrafo in `docs/areas/phasing.md`.
2. **App (`crates/ui-egui`).** `Palette::Renovation` in `dock.rs`, il contenuto in `phasing.rs` dell'app: tre pulsanti
   per lo stato con cui si disegna (più "Segue la vista"), tre per assegnarlo alla selezione con lo stato della
   selezione (o "misto"), il menu Ristrutturazione della vista. `BimApp::made_in` usa lo stato della palette quando c'è.
   Test egui_kittest: Da demolire e un muro disegnato; selezione e Nuovo; il filtro della vista. Parole in `it.po`,
   `cargo i18n update`.
3. **Guardare il risultato:** screenshot della palette e le tre piante (stato di fatto, comparativo, progetto) esportate.
4. **Paolo prova nell'app** con il modello italiano il caso del demolito del 22/11.
