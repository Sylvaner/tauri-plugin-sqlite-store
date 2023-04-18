# Tauri Plugin SqliteStore

Ajouter de la persistance SQLite à vos applications Tauri et accéder à n'importe quelle base de données SQLite se trouvant sur l'ordinateur.

# Installation

Les plugins Tauri avec une API nécessitent 2 étapes

## Installer le plugin

Dans le fichier `src-tauri/Cargo.toml`, ajouter la dépendance à la liste
```toml
[dependencies]
tauri-plugin-sqlite-store = { git = "https://github.com/Sylvaner/tauri-plugin-sqlite-store", branch = "main" }
```
Il sera compilé automatiquement à la compilation de l'application.

## Installer l'API

L'accès aux fonctionnalités se fait au travers d'une API qui facilite l'usage. En fonction de votre gestionnaire de paquet : 

```sh
pnpm add https://github.com/Sylvaner/tauri-plugin-sqlite-store
# ou
npm add https://github.com/Sylvaner/tauri-plugin-sqlite-store
# ou
yarn add https://github.com/Sylvaner/tauri-plugin-sqlite-store
```

# Configuration

## Enregistrer le plugin dans votre application

Dans la fonction main se trouvant dans le fichier `src-tauri/src/main.rs`, ajouter le plugin : 

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sqlite_store::init()) // Ajouter cette ligne
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Ajouter les autorisations d'accès au fichier

### Utilisation simple

Le plugin créé un fichier `store.sqlite` dans le répertoire APP_DATA. Pour permettre l'écriture de ce fichier, il faut modifier le fichier `tauri.conf.json` pour ajouter les permissions sur ce répertoire : 
```json
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {                   // Début 
        "readFile": true,       //
        "writeFile": true,      //
        "createDir": true,      //
        "scope": ["$APPDATA/*"] //
      },                        // Fin
      "shell": {
        "all": false,
        "open": true
      }
    },
```
Le répertoire et le fichier seront créés automatiquement à la première ouverture.

### Accès à une base de données dans un emplacement spécifique

Dans ce cas, il faut ajouter les permissions d'accès au répertoire ou à l'ensemble des fichiers de l'ordinateur (déconseillé). Dans le fichier `tauri.conf.json` : 
```json
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {              // Début 
        "readFile": true,  //
        "writeFile": true, //
        "createDir": true, //
        "scope": ["**"]    //
      },                   // Fin
      "shell": {
        "all": false,
        "open": true
      }
    },
```

# Utilisation

Importer le classe `SQLite` dans votre code pour pouvoir l'utiliser directement

## Utilisation simple

* Importer SqliteStore
* Charger le store
* Exécuter les requêtes

```ts
import { SqliteStore } from 'tauri-plugin-sqlite-store';

async function initDb() {
    const db = await SQliteStore.load();
    await db.execute(`CREATE DATABASE test (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
    )`);
    await db.execute(`INSERT INTO test VALUES (NULL, ?1)`, ['John']);
    await db.execute(`INSERT INTO test VALUES (NULL, ?1)`, ['Bill']);
    const rows = await db.select<{id, name}>('SELECT * FROM test');
    for (row of rows) {
        console.log(`${row.name} - Id: ${row.id}`);
    }
}
```

## Accès à une base de données dans un emplacement spécifique

La méthode d'ouverture d'une base de données est la seule chose qui change : 

```ts
const db = await SQliteStore.open('/path/to/the/db');
```

# Fonctionnalités

L'ensemble des fonctionnalités sont accessibles depuis `SQLiteStore` : 

* `load` - Charger le store local
* `open` - Ouvrir une base de données spécifique
* `select` - Lire un ensemble de données
* `selectFirst` - Lire le premier résultat d'un ensemble de données
* `execute` - Exécuter une requête
* `batch` - Exécuter une liste de requêtes
* `setPragma` - Exécuter une requête PRAGMA

## Création des tables

Une aide est fournie pour la création des tables avec un [générateur de requêtes](https://github.com/Sylvaner/SQLite-TableGenerator).
```ts
import { SqliteStore } from 'tauri-plugin-sqlite-store';

async function initDb() {
    const db = await SQliteStore.load();
    // `CREATE DATABASE test (
    //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     name TEXT NOT NULL
    //)`
    const gen = db.getTableGenerator('test');
    gen.addColumn('id', 'INTEGER', {primaryKey: true, autoIncrement: true});
    gen.addColumn('name', 'TEXT', {notNull: true});
    await db.execute(gen.getCreateTableQuery());

    // `CREATE DATABASE same_test (
    //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     name TEXT NOT NULL
    //)`
    await db.create({
      name: 'same_test',
      columns: [
        {
          name: 'id',
          type: 'INTEGER',
          options: {
            primaryKey: true,
            autoIncrement: true
          }
        },
        {
          name: 'name',
          type: 'TEXT',
          options: {
            notNull: true
          }
        }
      ]
    });
}
```

# Documentation

La documentation de l'API est accessible directement depuis GitHub : [Documentation](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/main/api-docs/classes/SqliteStore.md)

# Exemple

Dans le répertoire [example](https://github.com/Sylvaner/tauri-plugin-sqlite-store/tree/main/example) se trouve une application fonctionnelle avec le plugin installé. Pour le lancer : 

```sh
cd example
yarn install
yarn tauri dev
```

Les fichiers importants sont : 
* `src/App.svelte` : Utilisation de l'API
* `src-tauri/src/main.rs` : Enregistrement de l'API
* `src-tauri/tauri.conf.json` : Configuration des permissions

# Construire le paquet

```sh
yarn install
yarn build
```

Les fichiers du répertoire `webview-dist` seront générés.

# Remerciements

* lzdyes : Développeur du plugin Tauri Sqlite https://github.com/lzdyes/tauri-plugin-sqlite

# License

MIT or MIT/Apache 2.0 where applicable.