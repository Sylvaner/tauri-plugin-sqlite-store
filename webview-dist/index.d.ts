import { TableGenerator, SQLiteCreate } from 'sqlite-tablegenerator';
/**
 * Open database options
 */
export interface OpenOptions {
    /** Disable foreign keys validation */
    disable_foreign_keys?: boolean;
}
/**
 * List of queries to run in batch
 */
export type BatchQueries = [string, unknown[]][];
/**
 * API Tauri
 */
export declare class SqliteStore {
    /**
     * Database path
     */
    private path;
    /**
     * Constructor
     *
     * @param path - Database path
     */
    private constructor();
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
    private connect;
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
    static load(options?: OpenOptions): Promise<SqliteStore>;
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
    static open(path: string, options?: OpenOptions): Promise<SqliteStore>;
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
    getTableGenerator(name: string): TableGenerator;
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
    create(data: SQLiteCreate): Promise<boolean>;
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
    setPragma(key: string, value: any): Promise<boolean>;
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
    select<T>(query: string, params?: unknown[]): Promise<Array<T>>;
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
    selectFirst<T>(query: string, params?: unknown[]): Promise<T>;
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
    execute(query: string, params?: unknown[]): Promise<boolean>;
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
    batch(queries: BatchQueries): Promise<boolean>;
    /**
     * Close the database
     *
     * @returns True on success
     */
    close(): Promise<boolean>;
}
