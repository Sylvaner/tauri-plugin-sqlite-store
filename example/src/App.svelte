<script lang="ts">
import { CLASSIC_COLUMN_ID } from 'sqlite-tablegenerator';
import { SqliteStore } from 'tauri-plugin-sqlite-store';

let lastMessage = '';
let db: SqliteStore | null = null;

function showError(e) {
  lastMessage = e.toString();
}

async function createDb() {
  SqliteStore.load().then(result_db => {
    db = result_db;
      db.create({
        name: 'home',
        columns: [
          CLASSIC_COLUMN_ID,
          {
            name: 'address',
            type: 'TEXT'
          }
        ]
      }).then(() => {
        const personGenerator = db.getTableGenerator('person');
        personGenerator.addClassicIdColumn();
        personGenerator.addColumn('name', 'TEXT', {unique: true});
        personGenerator.addColumn('home', 'TEXT');
        personGenerator.addForeignKey({
          column: 'home',
          targetTable: 'home',
          targetColumn: 'id',
          onDelete: 'CASCADE'
        });
        db.execute(personGenerator.getCreateTableQuery())
          .then(() => { lastMessage = 'Database created'})
          .catch(showError);
      }).catch(showError);
  }).catch(e => {
      console.log(e);
      lastMessage = e.toString();
  });
}

async function populate() {
  if (db !== null) {
    await db.execute('INSERT INTO home VALUES (NULL, ?1)', ['Paris']);
    await db.execute('INSERT INTO home VALUES (NULL, ?1)', ['New-York']);
    await db.execute('INSERT INTO person VALUES (NULL, ?1, ?2)', ['Pascal', 1]);
    await db.execute('INSERT INTO person VALUES (NULL, ?1, ?2)', ['John', 2]);
    await db.execute('INSERT INTO person VALUES (NULL, ?1, ?2)', ['Mary', 2]);
    lastMessage = 'Database populated';
  }
}
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  <div class="row">
    { db !== null ? db['path'] : '' }
  </div>
  <div class="row">
    { lastMessage }
  </div>
  <div class="row">
    <p>
      <button on:click={createDb}>Create database</button>
    </p>
  </div>
  <div class="row">
    <p>
      <button on:click={populate}>Populate</button>
    </p>
  </div>


</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>