# 04 — Strutture composte e priorità dei materiali

## Come lavora un utente Archicad (comportamento atteso)

- Un **materiale da costruzione** ha: retino di sezione e colori, superficie (aspetto nel 3D), **priorità di
  intersezione** (un numero: più alto = vince), proprietà fisiche (densità, conducibilità termica, calore specifico),
  e dati descrittivi (ID, produttore, descrizione).
- Una **struttura composta** è un elenco ordinato di strati: ogni strato ha un materiale da costruzione, uno spessore e
  una funzione (**nucleo**, **finitura**, altro). Le linee di separazione tra strati e di contorno hanno penna e tipo.
- Dove due elementi si incontrano (muro con muro a L, T, X; muro con solaio, trave, pilastro), **ogni strato continua o
  si interrompe in base alla priorità del suo materiale**: lo strato con priorità più alta attraversa quello con priorità
  più bassa; a priorità uguale i materiali uguali si fondono (nessuna linea tra loro). Così l'isolante si raccorda
  all'isolante e la muratura passa sotto l'intonaco senza ritocchi a mano.
- Gli elementi si intersecano solo se i loro lucidi hanno lo stesso gruppo di intersezione ([02](02-lucidi.md)).
- Le quantità (volumi, superfici) si leggono per strato e per materiale, già "pulite" dalle intersezioni.

## Cosa c'è in BIMCraft

- `model.rs`: `Layer { function, material, thickness, variable }` per muri (dall'esterno verso l'interno), solai, tetti,
  controsoffitti (dall'alto verso il basso). `function` è quella di Revit (struttura, supporto, isolamento, finiture,
  membrana).
- `Material { color, cut, surface, cut_pattern, surface_pattern, texture, transparency }`: **nessuna priorità, nessuna
  proprietà fisica**.
- `walls/compound.rs`: il raccordo degli strati risolto solo per **un angolo retto tra muri finitura/struttura/finitura**
  (quale nucleo passa); per il resto, giunzioni "alla Revit".
- `quantities.rs`: volumi, casseforme e acciaio delle strutture in calcestruzzo, non per strato.

## Proposta

1. **M1 — Dati.** `Material` riceve `priority` (intero, es. 0–999, default a metà scala) e proprietà fisiche opzionali
   (`density_kg_m3`, `conductivity_w_mk`, `heat_capacity_j_kgk`). `FORMAT` + `Model::migrate` (i progetti vecchi
   prendono la priorità di default). Valori tipici: **nessuno nel codice**; materiali di partenza con i loro valori in un
   file di dati con fonte, come `data/codes/`.
2. **M2 — Algoritmo in pianta, tra muri.** Per ogni giunzione (L, T, X) si calcolano i poligoni di ogni strato di
   ciascun muro e si ritagliano: uno strato perde l'area occupata dagli strati dell'altro muro con priorità maggiore;
   a priorità uguale e stesso materiale i poligoni si uniscono. Funzione pura nel core, che restituisce i poligoni per
   strato; `render` li disegna (retini e linee di separazione), `quantities` li misura.
3. **M3 — In sezione: muri con solai, travi, pilastri**, stessa regola sul piano verticale.
4. **M4 — Quantità per strato e per materiale** (area e volume netti), comando in sola lettura; servirà al ponte con
   Quadra.
5. **M5 — UI profilo Arc**: finestra Materiali da costruzione (con la priorità) e finestra Strutture composte con
   anteprima dello spessore.

## Test (prima del codice, casi fatti a mano)

- Due muri uguali a L (intonaco 15 / laterizio 250 / intonaco 15): il laterizio è continuo nell'angolo, l'intonaco
  interno si interrompe contro il laterizio, quello esterno gira l'angolo.
- Muro a cappotto contro muro senza cappotto a T: l'isolante si ferma contro il nucleo del muro che lo riceve.
- Due strati di materiale uguale e priorità uguale: nessuna linea di separazione tra loro.
- L'area di ogni strato dopo il ritaglio = calcolata a mano; la somma dei volumi = il volume dei muri senza
  sovrapposizioni.
- Disegno: pixel del PNG della pianta (come i test di upstream), per controllare le linee.

## Rischi

È geometria del core usata da tutto (disegno, 3D, IFC, quantità). Va proposta a IPedrax **prima** come issue con questa
specifica e i casi di test; i parametri di curvatura (`arc`) e gli strati variabili si escludono dalla prima versione,
dichiarandolo con un commento `shortcut:` come fa upstream.
