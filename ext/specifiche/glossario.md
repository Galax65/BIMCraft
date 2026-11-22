# Glossario dell'interfaccia italiana

Le parole chiave dell'interfaccia, decise una volta e usate ovunque in `translations/ui/it.po`. Da approvare da Paolo
prima di riempire le voci vuote. Sono termini tecnici comuni dell'edilizia e del CAD in italiano, scelti perché un
utente Archicad li riconosca; non si copiano frasi o testi d'aiuto di Graphisoft.

Legenda: ✅ resta com'è · ✏️ da cambiare · ➕ oggi vuota o assente · **(?)** Paolo decide.

## Regole

1. **Comandi all'imperativo**, come oggi: Sposta, Ruota, Duplica, Modifica tipo.
2. **Maiuscola solo all'inizio**: "Proprietà del tipo", non "Proprietà del Tipo".
3. **Nomi propri non tradotti**: BIMCraft, Rev, Co-Work, IFC, DWG.
4. **Una parola inglese, una italiana**, sempre la stessa. Dove l'inglese ha due significati il `.po` usa un contesto
   (`msgctxt`): per esempio *layer* è uno **strato** di un muro composto, un **layer** di un DWG importato, e il
   **lucido** del fork (Fase 2).
5. **Le traduzioni valgono per tutti i profili.** Le parole proprie del flusso Archicad (Info Box, Navigatore con le
   sue mappe, Freccia) stanno in voci nuove del profilo Arc, in fondo, non al posto delle comuni.

## Termini comuni

### Interfaccia

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Project Browser | Navigatore di progetto | ✏️ Browser di progetto | libera "Navigatore" per il profilo Arc e per il Navigator del profilo BIMCraft |
| Navigator | — | ➕ Navigatore | il nome che il profilo BIMCraft dà al Project Browser |
| Inspector | — | ➕ Ispettore | il nome che il profilo BIMCraft dà alle Proprietà |
| Properties | Proprietà | ✅ | |
| Tools | Strumenti | ✅ | |
| Ribbon | Barra multifunzione | ✅ | |
| Options Bar | — | ➕ Barra delle opzioni | |
| Modify | Modifica | ✅ | |
| Settings | Impostazioni | ✅ | |
| Snap | — | ➕ Aggancio **(?)** | |

### Struttura del progetto ed elementi

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Level | Piano | ✅ | come Archicad |
| Grid | Griglia | ✅ | |
| Reference Plane | Piano di riferimento | ✅ | |
| Work Plane | Piano di lavoro | ✅ | |
| Wall | Muro | ✅ | |
| Curtain Wall | — | ➕ Facciata continua | |
| Curtain Grid | — | ➕ Griglia della facciata | |
| Mullion | — | ➕ Montante | |
| Floor | Solaio | ✅ | |
| Ceiling | Controsoffitto | ✅ | |
| Roof | Copertura | ✅ | |
| Roof by Footprint | — | ➕ Copertura da perimetro | |
| Column | Pilastro | ✅ | |
| Beam | Trave | ✅ | |
| Brace | — | ➕ Controvento | |
| Foundation | Fondazioni | ✏️ Fondazione | singolare |
| Wall Foundation | — | ➕ Fondazione continua | |
| Isolated Foundation | — | ➕ Plinto | |
| Stair / Ramp / Railing | Scala / Rampa / Parapetto | ✅ | |
| Door / Window | Porta / Finestra | ✅ | |
| Opening | Aperture | ✏️ Apertura | singolare |
| Shaft | Cavedio | ✅ | |
| Room | Zona | ✅ | come Archicad |
| Room Separator | Separatore di zona | ✅ | |
| Area | Superficie | ✅ **(?)** | l'elemento "area" di Revit; "Superficie" rischia di confondersi con la misura |
| Toposolid | — | ➕ Terreno | |
| Property Line | — | ➕ Confine di proprietà | |
| Point Cloud | Nuvola di punti | ✅ | |

### Tipi, famiglie, materiali

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Family | Famiglia | ✅ | |
| Component | Componente | ✅ | |
| Type / Instance | Tipo / Istanza | ✅ | |
| Type Properties | Proprietà del tipo | ✅ | |
| Edit Type | Modifica tipo | ✅ | |
| Material | Materiale | ✅ | |
| Compound Structure | — | ➕ Struttura composta | come Archicad |
| Layer (di un muro composto) | — | ➕ Strato | `msgctxt` |
| Layer (di un DWG) | — | ➕ Layer | `msgctxt "cad"`, termine CAD |
| Core | — | ➕ Nucleo | il nucleo di una struttura composta |
| Finish | Finitura | ✅ | |
| Model Group / Detail Group | — / Gruppo di dettaglio | ➕ Gruppo di modello | |
| Shared Parameters | — | ➕ Parametri condivisi | |
| Project Parameters | Parametri di progetto | ✅ | |

### Disegno e grafica

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Dimension | Quota | ✅ | |
| Spot Elevation | Quota altimetrica | ✅ | |
| Tag | Etichetta | ✅ | |
| Text | Testo | ✅ | |
| Detail Line / Model Line | Linea di dettaglio / — | ➕ Linea di modello | |
| Filled Region | Retino | ✅ | come Archicad |
| Fill Pattern | — | ➕ Motivo di retino **(?)** | |
| Masking Region | Area di mascheramento | ✅ | |
| Line Style / Line Weights | Stile di linea / Spessori linea | ✅ | |
| Object Styles | Stili oggetto | ✅ | |
| Visibility/Graphics | Visibilità/Grafica | ✅ | |

### Viste e tavole

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Floor Plan | Pianta | ✅ | |
| Ceiling Plan | Pianta controsoffitti | ✏️ Pianta dei controsoffitti | |
| Elevation / Section | Prospetto / Sezione | ✅ | |
| 3D View | Vista 3D | ✅ | |
| Camera | Fotocamera | ✏️ Camera **(?)** | |
| Drafting View | — | ➕ Vista di disegno **(?)** | |
| Legend | — | ➕ Legenda | |
| Schedule | Abaco | ✅ | |
| Sheet | Layout | ✅ | come Archicad |
| Title Block | — | ➕ Cartiglio | |
| Viewport | Riquadro vista | ✅ **(?)** | il disegno messo su un layout |
| View Template | Modello di vista | ✅ | |
| View Range | Intervallo di vista | ✅ | |
| Detail Level | Livello di dettaglio | ✅ | |
| Visual Style | Stile di visualizzazione | ✅ | |
| Crop Region | — | ➕ Area di ritaglio | |
| Section Box | Box di sezione | ✅ | |
| Revision | Revisione | ✅ | |
| Publish / Print / Export | — / — / Esporta | ➕ Pubblica / Stampa | |

### Fasi e opzioni

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Phase / Phases | Fase / — | ➕ Fasi | |
| Existing / Demolished / New Construction | Esistente / Demolito / Nuova costruzione | ✅ | i nomi italiani delle fasi (stato di fatto…) sono la Fase 5 |
| Phase Filter | Filtro delle fasi | ✅ | |
| Design Option(s) | Opzione/i di progetto | ✏️ Opzione/i di progettazione | come la specifica 03 |
| Option Set | — | ➕ Gruppo di opzioni | |
| Primary | — | ➕ Principale | l'opzione principale |

### Modifica

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Move / Copy / Rotate | Sposta / Copia / Ruota | ✅ | |
| Mirror | — | ➕ Specchia | |
| Array | — | ➕ Serie **(?)** | Archicad lo chiama "Moltiplica" |
| Align | Allinea | ✅ | |
| Offset | Scostamento | ✏️ Offset | termine CAD comune in italiano |
| Trim/Extend | — | ➕ Taglia/Estendi | |
| Split | Dividi | ✅ | |
| Pin / Unpin | — | ➕ Blocca / Sblocca | |
| Hide / Isolate | — | ➕ Nascondi / Isola | |
| Delete / Undo / Redo | Elimina / Annulla / Ripeti | ✅ | |
| Select All Instances | Seleziona tutte le istanze | ✅ | |
| Filter | Filtro | ✅ | |

### Discipline

| Inglese | Oggi | Proposta | Note |
|---|---|---|---|
| Building Codes | — | ➕ Normative | |
| Quantities | — | ➕ Quantità | |
| Reinforcement / Rebar | Armature / — | ➕ Barra d'armatura | |
| Pipe / Duct / Conduit / Cable Tray | Tubazione / — / Cavidotto / Passerella portacavi | ➕ Canale | il canale dell'aria |

## Solo nel profilo Arc (voci nuove, non traduzioni)

Nascono con il codice del profilo Arc, in inglese neutro nel sorgente (`msg!`), con questa traduzione italiana:

| Concetto | Italiano | Note |
|---|---|---|
| la barra in alto con i valori dello strumento | Info Box **(?)** | nome comune tra gli utenti Archicad; in alternativa "Barra informazioni" |
| la finestra dei valori dello strumento o della selezione | Impostazioni strumento | |
| i valori salvati di uno strumento | Preferiti | |
| lo strumento di selezione | Freccia | |
| il riquadro di selezione persistente | Riquadro | |
| la selezione cliccando sulla superficie | Selezione rapida | |
| la palette su un nodo o un lato | Palette contestuale | "Pet Palette" è un nome di Graphisoft |
| prendere / iniettare i parametri | Contagocce / Siringa | |
| la selezione per criteri | Trova e seleziona | |
| il Navigatore e le sue parti | Navigatore: Mappa progetto, Mappa viste, Book impaginazione, Set di pubblicazione **(?)** | |
| visibilità per categoria del fork (Fase 2) | Lucido, Combinazione di lucidi | |
| materiale con priorità (Fase 4) | Materiale da costruzione, Priorità d'intersezione | |
| solido a forma libera | Forma libera | "Morph" è un nome di Graphisoft |
