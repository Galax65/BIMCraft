# Bozze

Codice **mai compilato**, tenuto da parte. Non fa parte del workspace (il `Cargo.toml` è rinominato `Cargo.toml.bozza`).

- `quadra/` — prima bozza del connettore BIMCraft → Quadra (contratto `bimcraft-quadra` 0.1.0: progetto, livelli, fasi,
  elementi con GUID, misure con l'unità nel nome, parametri). Legge il modello con il comando `elements` e i campi
  pubblici di `Session::model`. Serve alla Fase 7: allora diventerà `ext/crates/quadra`, si aggiungerà `"ext/crates/*"`
  ai `members` del `Cargo.toml` radice e si scriveranno prima i test.
