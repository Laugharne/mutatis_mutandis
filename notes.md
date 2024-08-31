
1. préparer cli processing
    - [X] working directory
    - [X] préparer mécanisme d'options du cli
    - [X] `version` & `versions`
    - [X] préparer moyen/manière de scanner un projet anchor (alias bash)...
    - [X] `init`
      - [X] check if anchor project, creation dirs & files
      - [X] update `.gitignore`
      - [ ] interactive mode -> `mutatis.toml`
    - [ ] `clear`
    - [ ] `reset`
    - [ ] `remove`
    - [ ] `help`
2. fichier toml
3. mutations & tests

Librairies:
- [X] lib utils.rs
- [X] lib cli.rs

--------

in `.gitignore`file:
- add `/.mutatis`
- add `/test-ledger`

**Tree:**
- .mutatis
  - /backup
  - /logs
  - /mutations
    - /m0001
    - /m0002
    - /m....
  - /tmp
  - `mutatis.toml`

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


--------

Pour gérer les sections dans un fichier TOML en Rust, vous pouvez utiliser des structures imbriquées pour représenter les sections. En TOML, les sections sont représentées par des tables, et vous pouvez créer des structures Rust correspondantes qui reflètent cette organisation.

Voici comment gérer les sections dans un fichier TOML en utilisant Rust, `serde` pour la sérialisation, et `toml` pour la conversion des structures Rust en chaîne de texte au format TOML.

### 1. Ajouter les Dépendances à `Cargo.toml`

Assurez-vous que votre fichier `Cargo.toml` contient les dépendances suivantes :

```toml
[dependencies]
toml = "0.7"
serde = { version = "1.0", features = ["derive"] }
```

### 2. Exemple de Code Rust

Voici un exemple de code pour créer une structure avec des sections, sérialiser ces données en TOML, puis les écrire dans un fichier :

```rust
use serde::Serialize;
use std::fs::File;
use std::io::Write;

// Définir des structures représentant les sections de votre fichier TOML

#[derive(Serialize)]
struct Config {
    general: GeneralConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
}

// Section générale du fichier TOML
#[derive(Serialize)]
struct GeneralConfig {
    titre: String,
    version: u32,
    debug: bool,
}

// Section de configuration de la base de données
#[derive(Serialize)]
struct DatabaseConfig {
    hote: String,
    port: u16,
    utilisateur: String,
    mot_de_passe: String,
}

// Section de configuration de la journalisation
#[derive(Serialize)]
struct LoggingConfig {
    niveau: String,
    chemin_du_fichier: String,
}

fn main() -> std::io::Result<()> {
    // Créer une instance de la configuration
    let config = Config {
        general: GeneralConfig {
            titre: String::from("Mon application"),
            version: 1,
            debug: true,
        },
        database: DatabaseConfig {
            hote: String::from("localhost"),
            port: 5432,
            utilisateur: String::from("admin"),
            mot_de_passe: String::from("secret"),
        },
        logging: LoggingConfig {
            niveau: String::from("info"),
            chemin_du_fichier: String::from("/var/log/app.log"),
        },
    };

    // Sérialiser la structure en une chaîne de caractères TOML
    let toml_str = toml::to_string(&config).expect("Erreur de sérialisation TOML");

    // Ouvrir ou créer le fichier de configuration TOML
    let mut fichier = File::create("config.toml")?;

    // Écrire la chaîne TOML dans le fichier
    fichier.write_all(toml_str.as_bytes())?;

    println!("Fichier TOML écrit avec succès.");
    Ok(())
}
```

### Explications

1. **Définir les Structures pour les Sections :**
   - Chaque section du fichier TOML est représentée par une structure distincte :
     - `GeneralConfig` pour la section générale.
     - `DatabaseConfig` pour la section base de données.
     - `LoggingConfig` pour la section de journalisation.
   - La structure `Config` principale regroupe ces sous-structures.

2. **Créer une Instance de la Configuration :**
   - Une instance de `Config` est créée avec des valeurs spécifiques pour chaque section.

3. **Sérialiser en Chaîne TOML :**
   - `toml::to_string(&config)` est utilisé pour convertir la structure `Config` en une chaîne TOML.

4. **Écrire dans le Fichier :**
   - Le contenu sérialisé est écrit dans un fichier `config.toml`.

### Résultat du Fichier TOML

Le fichier `config.toml` généré pourrait ressembler à ceci :

```toml
[general]
titre = "Mon application"
version = 1
debug = true

[database]
hote = "localhost"
port = 5432
utilisateur = "admin"
mot_de_passe = "secret"

[logging]
niveau = "info"
chemin_du_fichier = "/var/log/app.log"
```

### Compilation et Exécution

Pour compiler et exécuter le code :

1. Enregistrez le code dans un fichier, par exemple `main.rs`.
2. Compilez et exécutez le programme avec :
   ```bash
   cargo run
   ```

Ce programme créera un fichier `config.toml` contenant des sections clairement définies. En utilisant des structures imbriquées, vous pouvez facilement modéliser des configurations complexes avec plusieurs sections en Rust.

--------

Pour gérer le contenu (champs et sections) d'un fichier TOML en lecture avec Rust, vous pouvez utiliser `serde` pour désérialiser les données TOML dans des structures Rust. Cela vous permet de lire un fichier TOML existant et de mapper son contenu à des structures Rust définies.

### Étapes pour Lire et Gérer le Contenu d'un Fichier TOML

1. **Ajouter les Dépendances dans `Cargo.toml`**
2. **Définir des Structures Représentant les Sections du Fichier TOML**
3. **Lire et Désérialiser le Fichier TOML**
4. **Manipuler et Afficher le Contenu Désérialisé**

### 1. Ajouter les Dépendances à `Cargo.toml`

Assurez-vous d'avoir les dépendances nécessaires dans votre fichier `Cargo.toml` :

```toml
[dependencies]
toml = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_derive = "1.0"
```

### 2. Exemple de Code Rust pour Lire et Gérer le Contenu

Voici un exemple de code pour lire et désérialiser un fichier TOML en Rust :

```rust
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

// Définir des structures représentant les sections de votre fichier TOML
#[derive(Deserialize, Debug)]
struct Config {
    general: GeneralConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
}

// Section générale du fichier TOML
#[derive(Deserialize, Debug)]
struct GeneralConfig {
    titre: String,
    version: u32,
    debug: bool,
}

// Section de configuration de la base de données
#[derive(Deserialize, Debug)]
struct DatabaseConfig {
    hote: String,
    port: u16,
    utilisateur: String,
    mot_de_passe: String,
}

// Section de configuration de la journalisation
#[derive(Deserialize, Debug)]
struct LoggingConfig {
    niveau: String,
    chemin_du_fichier: String,
}

fn main() -> std::io::Result<()> {
    // Lire le contenu du fichier TOML
    let mut fichier = File::open("config.toml")?;
    let mut contenu = String::new();
    fichier.read_to_string(&mut contenu)?;

    // Désérialiser le contenu du fichier en structure Rust
    let config: Config = toml::from_str(&contenu).expect("Erreur de désérialisation TOML");

    // Utiliser et afficher le contenu désérialisé
    println!("Titre: {}", config.general.titre);
    println!("Version: {}", config.general.version);
    println!("Mode debug: {}", config.general.debug);
    println!("Hôte de la base de données: {}", config.database.hote);
    println!("Port de la base de données: {}", config.database.port);
    println!("Utilisateur de la base de données: {}", config.database.utilisateur);
    println!("Mot de passe de la base de données: {}", config.database.mot_de_passe);
    println!("Niveau de journalisation: {}", config.logging.niveau);
    println!("Chemin du fichier de journalisation: {}", config.logging.chemin_du_fichier);

    Ok(())
}
```

### Explications

1. **Définir les Structures pour les Sections :**
   - Chaque section du fichier TOML est représentée par une structure distincte.
   - Utilisez `#[derive(Deserialize, Debug)]` pour activer la désérialisation avec `serde` et ajouter un format de débogage (`Debug`).

2. **Lire le Contenu du Fichier TOML :**
   ```rust
   let mut fichier = File::open("config.toml")?;
   let mut contenu = String::new();
   fichier.read_to_string(&mut contenu)?;
   ```
   Ouvrez le fichier TOML et lisez son contenu dans une variable de type `String`.

3. **Désérialiser le Contenu en Structures Rust :**
   ```rust
   let config: Config = toml::from_str(&contenu).expect("Erreur de désérialisation TOML");
   ```
   Utilisez `toml::from_str` pour convertir le contenu du fichier TOML en une instance de la structure `Config`.

4. **Utiliser et Afficher le Contenu Désérialisé :**
   Affichez ou manipulez les données désérialisées comme vous le souhaitez.

### Résultat de la Lecture

Si vous avez un fichier `config.toml` comme suit :

```toml
[general]
titre = "Mon application"
version = 1
debug = true

[database]
hote = "localhost"
port = 5432
utilisateur = "admin"
mot_de_passe = "secret"

[logging]
niveau = "info"
chemin_du_fichier = "/var/log/app.log"
```

Le programme affichera :

```
Titre: Mon application
Version: 1
Mode debug: true
Hôte de la base de données: localhost
Port de la base de données: 5432
Utilisateur de la base de données: admin
Mot de passe de la base de données: secret
Niveau de journalisation: info
Chemin du fichier de journalisation: /var/log/app.log
```

### Compilation et Exécution

Pour compiler et exécuter le code :

1. Enregistrez le code dans un fichier, par exemple `main.rs`.
2. Compilez et exécutez le programme avec :
   ```bash
   cargo run
   ```

### Résumé

Ce code montre comment lire et gérer le contenu d'un fichier TOML en utilisant Rust. Vous pouvez utiliser les structures pour représenter des sections et des champs du fichier, et `serde` pour effectuer la désérialisation automatiquement.
