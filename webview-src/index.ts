import { invoke } from '@tauri-apps/api/tauri';
import { TableGenerator, SQLiteCreate } from 'sqlite-tablegenerator';

/**
 * Open database options
 */
export interface OpenOptions {
  /** Disable foreign keys validation */
  disable_foreign_keys?: boolean
}

/**
 * List of queries to run in batch
 */
export type BatchQueries = [string, unknown[]][];

/**
 * API Tauri 
 */
export class SqliteStore {
  /**
   * Database path
   */
  private path: string = '';

  /**
   * Constructor
   * 
   * @param path - Database path
   */
  private constructor(path: string) {
    this.path = path;
  }

  /**
   * Connect to the database
   * 
   * @param options - Opening options
   * 
   * @returns True on success
   * 
   * @example
   * ```ts
   * this.connect({
   *  disable_foreign_keys: false
   * });
   * ```
   */
  private connect(options?: OpenOptions): Promise<boolean> {
    if (options === undefined) {
      options = {};
    }
    return invoke('plugin:sqlite-store|open', {dbPath: this.path, options});
  }
  
  /**
   * Open store
   * 
   * @param options  - Opening options
   * 
   * @returns Api object
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.load();
   * ```
   */
  public static load(options?: OpenOptions): Promise<SqliteStore> {
    return new Promise<SqliteStore>((resolve, reject) => {
      let instance = new SqliteStore('');
      if (options === undefined) {
        options = {};
      }
      invoke('plugin:sqlite-store|load', {options}).then(storePath => {
        instance.path = storePath as string;
        resolve(instance);
      }).catch(reject);
    });
  }

  /**
   * Open database file
   * 
   * @param path - Path to the database file 
   * @param options  - Opening options
   * 
   * @returns Api object
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.open('/path/to/the/db', {
   *   disable_foreign_keys: true
   * });
   * ```
   */
  public static open(path: string, options?: OpenOptions): Promise<SqliteStore> {
    return new Promise<SqliteStore>((resolve, reject) => {
      let instance = new SqliteStore(path);
      instance.connect(options).then(success => {
        if (success) {
          resolve(instance);
        } else {
          reject(`Unable to open database ${path}.`);
        }
      }).catch(reject);      
    });
  }

  /**
   * Get generator helper for table creation
   * @link https://github.com/Sylvaner/SQLite-TableGenerator
   * 
   * @param name Table name
   * 
   * @returns Generator for the table
   * 
   * ```ts
   * const db = await SqliteStore.load();
   * const personGenerator = db.getTableGenerator('person');
   * personGenerator.setTableOptions({ifNotExists: true});
   * personGenerator.addClassicIdColumn();
   * personGenerator.addColumn('name', 'TEXT', {notNull: true, default: 'John'});
   * personGenerator.addColumn('home', 'INTEGER', {notNull: true});
   * personGenerator.addForeignKey({
   *   key: 'home',
   *   targetTable: 'home',
   *   targetKey: 'id',
   *   onDelete: 'CASCADE'
   * });
   * db.execute(personGenerator.getCreateTableQuery(true));
   * ```
   * ```
   */
  public getTableGenerator(name: string): TableGenerator {
    return new TableGenerator(name);
  }

  /**
   * Create a table from a definition at TableGenerator format
   * @link https://github.com/Sylvaner/SQLite-TableGenerator
   * 
   * @param data - Table informations
   * 
   * @returns True on success
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.load();
   * db.create({
   *   name: 'test_table',
   *   columns: [
   *     {
   *       name: 'id',
   *     }
   *   ]
   * });
   * ```
   */
  public create(data: SQLiteCreate): Promise<boolean> {
    const query = TableGenerator.fromData(data).getCreateTableQuery();
    return this.execute(query, []);
  }

  /**
   * Set pragma value
   * 
   * @param key - Pragma key
   * @param value - Value to set
   * 
   * @returns True on success
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.open('/path/to/the/db');
   * db.setPragma('foreign_keys', 0);
   * ```
   */
  public setPragma(key: string, value: any): Promise<boolean> {
    return invoke('plugin:sqlite-store|set_pragma', {dbPath: this.path, key, value});
  }

  /**
   * Select in database
   * 
   * @param query - Select query
   * @param params - Query params
   * 
   * @returns Array with selected rows
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.open('/path/to/the/db');
   * const rows = await db.select<{id: number, name: string}>('SELECT id, name FROM person WHERE age >= ?1 and sex = ?2', [18, 'M']);
   * for (r of rows) {
   *   console.log(`${person.name} has id ${person.id}`);
   * }
   * ```
   */
  public select<T>(query: string, params?: unknown[]): Promise<Array<T>> {
    if (params === undefined) {
      params = [];
    }
    return invoke('plugin:sqlite-store|select', {dbPath: this.path, query, params});
  }

  /**
   * Select first row in selected rows
   * 
   * @param query - Select query
   * @param params - Query params
   * 
   * @returns Array with selected rows
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.open('/path/to/the/db');
   * db.selectFirst<{id: number, name: string}>('SELECT id, name FROM person WHERE age >= ?1 and sex = ?2', [18, 'M']).then(perso => {
   *   console.log(`${person.name} has id ${person.id}`);
   * }).catch((e) => {
   *   console.error('No adults');
   * });
   * ```
   */  
  public selectFirst<T>(query: string, params?: unknown[]): Promise<T> {
    return new Promise<T>((resolve, reject) => {
      this.select<T>(query, params).then((results) => {
        if (results.length > 0) {
          resolve(results[0]);
        } else {
          reject(new Error('No results'));
        }
      }).catch(reject);
    });
  }

  /**
   * Execute a query
   * 
   * @param query - SQL query
   * @param params - Query params
   * 
   * @returns True on success
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.open('/path/to/the/db');
   * db.execute('DELETE FROM person WHERE age < ?1 and sex = ?2', [18, 'F']);
   * ```
   */
  public execute(query: string, params?: unknown[]): Promise<boolean> {
    if (params === undefined) {
      params = [];
    }
    return invoke('plugin:sqlite-store|execute', {dbPath: this.path, query, params});
  }

  /**
   * Execute a list of queries with on transaction (Rollback on error)
   * 
   * @param queries - List of queries
   * 
   * @returns True on success
   * 
   * @example
   * 
   * ```ts
   * const db = await SqliteStore.open('/path/to/the/db');
   * db.batch([
   *  ['DELETE FROM person WHERE age < ?1', [18]],
   *  ['INSERT INTO person VALUES (NULL, ?1, ?2, ?3)', ['John', 16, 'F']]
   * ]);
   * ```
   */
  public batch(queries: BatchQueries): Promise<boolean> {
    return invoke('plugin:sqlite-store|batch', {dbPath: this.path, queries});
  }

  /**
   * Close the database
   * 
   * @returns True on success
   */
  public close(): Promise<boolean> {
    return invoke('plugin:sqlite-store|close', {dbPath: this.path});
  }
}
