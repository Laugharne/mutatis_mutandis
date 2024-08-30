in `.gitignore`file:
- add `/.mutatis`
- add `.mutatis.toml`
- add `/test-ledger`

**Tree:**
- .mutatis
  - backup
  - mutations
    - m0001
    - m0002
    - m....
  - tmp

--------

```rust
use std::fs::File;
use std::env;
use text_colorizer::*;
```

--------

`mutatis init`
- check if working dir is an anchor project
- ask for `test-ledger` path current if no response or SOL_VALIDATOR_DIRECTORY (added `.bashrc`)

`mutatis analyze`

`mutatis run` : check if a commit was submit before ?

`mutatis reset` : rétabli les sources originelles

`mutatis clear`

Clear:
- results
- mutations

`mutatis remove` :

Remove all:
- mutations
- `.mutatis.toml`
- `/test-ledger`

--------

`mutatis help`

`mutatis version`

`mutatis versions`

--------

**Later:**

- Jest
- Bankrun
- Parallelization
- Ignoring some project directories or rust files

**Anchor:**

- anchor test
- paramétrages: commande pour lancer les tests (par défaut)
  - "_solana-test-validator --reset_"
  - "_anchor test --skip-local-validator_"
  - path pour "_test-ledger_"
- Besoin d'autres spécificités d'anchor ?? (_accounts ??, string relative à des PDA ?_)