# 05 — Flusso di lavoro e selezione stile Archicad

Base del profilo Arc ([00](00-profilo-archicad.md)): come lavora un utente Archicad, e cosa BIMCraft fa già. Le
osservazioni di Paolo nell'app hanno la precedenza su quanto scritto qui; ciò che è da verificare è segnato **(da
verificare)**.

## Fonti

Solo comportamento, mai testi, icone o contenuti Graphisoft.

- Aiuto pubblico di Graphisoft: [impostazioni strumento](https://help.graphisoft.com/AC/30/INT/Topics/150_UserInterfaceToolSettings/150_UserInterfaceToolSettings-2.html),
  [selezione](https://help.graphisoft.com/AC/24/INT/_AC24_Help/030_Interaction/030_Interaction-26.htm), e gli articoli
  della community sullo [strumento Freccia](https://community.graphisoft.com/t5/Getting-started/How-to-Select-Elements-The-Arrow-Tool/ta-p/304007)
  e sul [Riquadro](https://community.graphisoft.com/t5/Getting-started/How-to-Select-Elements-The-Marquee-Tool/ta-p/304006).
- [ArchiCraft](https://github.com/ArchiCraftApp/ArchiCraft) (Rust ed egui, CC BY 4.0), letto il 22/11/2026 al commit
  `301d572`: la sua specifica `docs/SRS.md` ricostruisce il flusso di Archicad dalla Knowledge Base pubblica (sezioni 4.3
  interfaccia, 4.7 selezione, 4.8 modifica, 4.28 lucidi, appendice B: i termini di Archicad e i nomi neutri) e
  `docs/keymap.md` propone le scorciatoie. Ne prendiamo le idee di comportamento, non il codice.

## Il flusso, passo per passo

1. **Scegliere uno strumento** nella palette strumenti (o con la sua scorciatoia). L'**Info Box** in alto mostra subito i
   valori principali dello strumento: tipo o struttura, spessore, altezza, lucido, linea di riferimento, metodo di
   costruzione.
2. **Regolare** i valori nell'Info Box, oppure aprire le **impostazioni strumento** (Cmd-T, doppio clic sullo strumento,
   clic sull'icona nell'Info Box). La finestra dice nel titolo se modifica i **predefiniti** (per i prossimi elementi) o
   gli **N elementi selezionati**. In alto a sinistra i **preferiti** dello strumento.
3. **Disegnare**: lo strumento resta attivo dopo ogni elemento; Esc annulla l'input in corso.
4. **Selezionare** senza cambiare strumento (sotto), modificare con l'Info Box, Cmd-T o la **Pet Palette**.
5. **Trasferire parametri**: contagocce (Alt-clic su un elemento: lo strumento ne prende i valori) e siringa
   (Cmd-Alt-clic: li inietta) **(da verificare i tasti su Mac)**.

## Selezione

| Gesto | Archicad | BIMCraft oggi |
|---|---|---|
| Strumento di selezione | la **Freccia**, uno strumento della palette | la modalità Modifica, come Revit |
| Clic | seleziona; con la **selezione rapida** (calamita, attiva di base) anche cliccando sulla superficie, non solo su bordi e nodi | linee e campiture |
| Aggiungere / togliere | **Shift+clic** aggiunge, e su un elemento già selezionato lo toglie | **Ctrl** aggiunge, **Shift** toglie (`Canvas::pick_up`) |
| Selezionare con uno strumento attivo | **Shift tenuto** fa da Freccia senza lasciare lo strumento | no: si torna a Modifica |
| Riquadro | il **Riquadro** (Marquee) resta disegnato, su un piano o su tutti, e limita "Seleziona tutto" e i comandi; si sposta e si toglie | riquadro temporaneo: da sinistra gli elementi interi, da destra anche quelli toccati (`outline_walls_in_box`) |
| Riquadro della Freccia | elementi interi o anche toccati, scelto nell'Info Box **(da verificare se dipende dalla direzione)** | dalla direzione |
| Seleziona tutto (Cmd-A) | tutti gli elementi del tipo dello strumento attivo (tutti, con la Freccia), nel Riquadro se c'è | Seleziona tutte le istanze (nella vista o nel progetto) |
| Trova e seleziona | per criteri combinati (tipo, piano, lucido, materiale, ID…), set di criteri salvati | Filtro sulla selezione, per categoria o parametro (`selection.rs`) |
| Elementi sovrapposti | **(da verificare)** | Tab cambia il candidato |
| Annullare la selezione | Esc o clic nel vuoto | Esc |
| Lucidi | elementi su lucidi nascosti non selezionabili; su lucidi bloccati selezionabili ma non modificabili | i lucidi non ci sono (Fase 2) |

## Modifica di una selezione

- **Pet Palette**: un clic su un nodo o un lato di un elemento selezionato apre i comandi di quella parte (sposta nodo,
  inserisci nodo, offset del lato, curva il lato, tutti i lati) oltre a sposta, ruota, specchia, sposta in verticale.
  BIMCraft ha una piccola palette per le linee di schizzo (`BimApp::edge_palette`).
- **Trasformazioni** con la variante copia e copia multipla, scelta durante il comando con un tasto.
- **Selezione multipla nell'Info Box**: i campi con valori diversi sono "misti"; un valore scritto vale per tutti gli
  elementi modificabili.

## Note di Paolo dopo le prime prove (22/11/2026)

1. **Selezionare con Shift dentro un comando attivo**: con uno strumento di disegno in mano, Shift tenuto seleziona
   senza lasciarlo (vedi la tabella sopra). Va nello schema di selezione.
2. **Clic sull'ultimo punto della catena**: un clic sull'ultimo punto già messo termina la catena (muri, linee,
   polilinee), come un doppio clic. Oggi la catena finisce con Invio, Esc o un doppio clic; cliccare di nuovo il
   primo punto la chiude. (Confermato da Paolo: termina, i muri restano.)
3. **Comandi di modifica in una barra orizzontale in alto**: sposta, ruota, specchia, offset, taglia… in fila
   orizzontale in alto, accanto o sopra la barra delle impostazioni, invece che nella colonna della palette.
4. **Niente terzo profilo**: queste cose si propongono come modifiche al profilo BIMCraft (vedi [00](00-profilo-archicad.md)).
5. **File di Archicad**: vedi [06](06-da-archicad.md).

## Cosa cambia per upstream

Upstream vuole profili che differiscano **solo nell'aspetto**, con le stesse scorciatoie (`docs/areas/desktop.md`).
L'Info Box, le impostazioni strumento, il Navigatore sono aspetto. La selezione di Archicad (Shift+clic, Shift con uno
strumento attivo, Freccia al posto di Modifica, Cmd-T) cambia invece il comportamento del mouse e della tastiera: va
proposta a IPedrax come **schema di selezione e di scorciatoie** scelto nelle impostazioni, indipendente dal profilo,
nella stessa issue del profilo Arc. Il Riquadro persistente e il contagocce nascono nel core (comandi nuovi) e valgono
per tutti i profili.

## Ordine proposto (ogni passo con il suo piano e il suo ok)

1. Il profilo Arc (vedi [00](00-profilo-archicad.md)).
2. L'aggancio in alto delle palette e l'Info Box.
3. Cmd-T: le impostazioni dello strumento o della selezione.
4. Lo schema di selezione Archicad (dopo la risposta di IPedrax).
5. Riquadro persistente, contagocce e siringa (core).
