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

## Da confermare con Paolo (comportamento di Archicad)

- Un elemento Da demolire: nello stato di fatto normale, nel comparativo giallo, nel progetto assente.
- Un elemento Nuovo: nello stato di fatto assente, nel comparativo rosso, nel progetto normale.
- Porte e finestre: prendono lo stato del muro o il proprio? (In BIMCraft oggi seguono il muro se non hanno una fase.)
- Si può dare uno stato a un elemento di un gruppo, o a parti di un muro?

## Nel frattempo

Il [modello italiano](../modelli/README.md) spiega il procedimento con le tre piante, e il comparativo mostra in giallo
anche gli elementi "temporanei", così un errore si vede.
