# Tauri Plugin SqliteStore

[:fr: This page in French](README-fr.md)

Add SQLite persistence to your Tauri applications and access any SQLite database on the computer.

# Install

Tauri plugins with an API require 2 steps

## Install plugin

In the file `src-tauri/Cargo.toml`, add the dependency to the list
```toml
[dependencies]
tauri-plugin-sqlite-store = { git = "https://github.com/Sylvaner/tauri-plugin-sqlite-store", branch = "main" }
```
It will be compiled automatically when compiling the application.

## Install API

Access to features is with an API that makes it easier to use. Depending on your package manager:

```sh
pnpm add https://github.com/Sylvaner/tauri-plugin-sqlite-store
# ou
npm add https://github.com/Sylvaner/tauri-plugin-sqlite-store
# ou
yarn add https://github.com/Sylvaner/tauri-plugin-sqlite-store
```

# Configuration

## Register the plugin in your application

In the main function in file `src-tauri/src/main.rs`, register the plugin : 

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sqlite_store::init()) // Add this line
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Add file access permissions

### Simple usage

The plugin creates a file `store.sqlite` in the APP_DATA directory. To allow this file to be written, you must modify the file `tauri.conf.json` to add permissions to this directory:
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
Access to a database in a specific location

### Accès à une base de données dans un emplacement spécifique

In this case, you must add the access permissions to the directory or to all the files on the computer (not recommended). In the file `tauri.conf.json`:
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

# Usage

Import the class `SQLite` into your code so you can use it directly

## Utilisation simple

* Import SqliteStore
* Load store
* Execute queries

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

## Access to a database in a specific location

The method of opening a database is the only thing that changes:

```ts
const db = await SQliteStore.open('/path/to/the/db');
```

# Features

All features are accessible from `SQLiteStore`:

* `load` - Load local store
* `open` - Open a specific database
* `select` - Read a dataset
* `selectFirst` - Read the first result of a dataset
* `execute` - Run a query
* `batch` - Run a list of queries
* `setPragma` - Run a PRAGMA query

## Tables creation

Help is provided for creating tables with a [générateur de requêtes](https://github.com/Sylvaner/SQLite-TableGenerator).
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

The API documentation is accessible directly from GitHub: [Documentation](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/main/api-docs/classes/SqliteStore.md)

# Exemple

Dans le répertoire [example](https://github.com/Sylvaner/tauri-plugin-sqlite-store/tree/main/example) se trouve une application fonctionnelle avec le plugin installé. Pour le lancer : 

```sh
cd example
yarn install
yarn tauri dev
```

The important files are:
* `src/App.svelte` : API Usage
* `src-tauri/src/main.rs` : API registration
* `src-tauri/tauri.conf.json` : Configuration permissions

# Build the package

```sh
yarn install
yarn build
```

Files are generated in the folder `webview-dist`.

# Thanks

* lzdyes : Tauri Sqlite plugin developer https://github.com/lzdyes/tauri-plugin-sqlite

# License

MIT or MIT/Apache 2.0 where applicable.