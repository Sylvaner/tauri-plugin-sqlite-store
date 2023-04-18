[tauri-plugin-sqlite-store-api](../README.md) / [Exports](../modules.md) / SqliteStore

# Class: SqliteStore

API Tauri

## Table of contents

### Constructors

- [constructor](SqliteStore.md#constructor)

### Properties

- [path](SqliteStore.md#path)

### Methods

- [batch](SqliteStore.md#batch)
- [connect](SqliteStore.md#connect)
- [execute](SqliteStore.md#execute)
- [select](SqliteStore.md#select)
- [selectFirst](SqliteStore.md#selectfirst)
- [setPragma](SqliteStore.md#setpragma)
- [load](SqliteStore.md#load)
- [open](SqliteStore.md#open)

## Constructors

### constructor

• `Private` **new SqliteStore**(`path`)

Constructor

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `path` | `string` | Database path |

#### Defined in

[index.ts:30](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L30)

## Properties

### path

• `Private` **path**: `string` = `''`

Database path

#### Defined in

[index.ts:23](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L23)

## Methods

### batch

▸ **batch**(`queries`): `Promise`<`boolean`\>

Execute a list of queries with on transaction (Rollback on error)

**`Example`**

```ts
const db = await SqliteStore.open('/path/to/the/db');
db.batch([
 ['DELETE FROM person WHERE age < ?1', [18]],
 ['INSERT INTO person VALUES (NULL, ?1, ?2, ?3)', ['John', 16, 'F']]
]);
```

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `queries` | [`BatchQueries`](../modules.md#batchqueries) | List of queries |

#### Returns

`Promise`<`boolean`\>

True on success

#### Defined in

[index.ts:215](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L215)

___

### connect

▸ `Private` **connect**(`options?`): `Promise`<`boolean`\>

Connect to the database

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `options?` | [`OpenOptions`](../interfaces/OpenOptions.md) | Opening options |

#### Returns

`Promise`<`boolean`\>

True on success

#### Defined in

[index.ts:41](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L41)

___

### execute

▸ **execute**(`query`, `params?`): `Promise`<`boolean`\>

Execute a query

**`Example`**

```ts
const db = await SqliteStore.open('/path/to/the/db');
db.execute('DELETE FROM person WHERE age < ?1 and sex = ?2', [18, 'F']);
```

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `query` | `string` | SQL query |
| `params?` | `unknown`[] | Query params |

#### Returns

`Promise`<`boolean`\>

True on success

#### Defined in

[index.ts:191](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L191)

___

### select

▸ **select**<`T`\>(`query`, `params?`): `Promise`<`T`[]\>

Select in database

**`Example`**

```ts
const db = await SqliteStore.open('/path/to/the/db');
const rows = await db.select<{id: number, name: string}>('SELECT id, name FROM person WHERE age >= ?1 and sex = ?2', [18, 'M']);
for (r of rows) {
  console.log(`${person.name} has id ${person.id}`);
}
```

#### Type parameters

| Name |
| :------ |
| `T` |

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `query` | `string` | Select query |
| `params?` | `unknown`[] | Query params |

#### Returns

`Promise`<`T`[]\>

Array with selected rows

#### Defined in

[index.ts:138](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L138)

___

### selectFirst

▸ **selectFirst**<`T`\>(`query`, `params?`): `Promise`<`T`\>

Select first row in selected rows

**`Example`**

```ts
const db = await SqliteStore.open('/path/to/the/db');
db.selectFirst<{id: number, name: string}>('SELECT id, name FROM person WHERE age >= ?1 and sex = ?2', [18, 'M']).then(perso => {
  console.log(`${person.name} has id ${person.id}`);
}).catch((e) => {
  console.error('No adults');
});
```

#### Type parameters

| Name |
| :------ |
| `T` |

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `query` | `string` | Select query |
| `params?` | `unknown`[] | Query params |

#### Returns

`Promise`<`T`\>

Array with selected rows

#### Defined in

[index.ts:164](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L164)

___

### setPragma

▸ **setPragma**(`key`, `value`): `Promise`<`boolean`\>

Set pragma value

**`Example`**

```ts
const db = await SqliteStore.open('/path/to/the/db');
db.setPragma('foreign_keys', 0);
```

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `key` | `string` | Pragma key |
| `value` | `any` | Value to set |

#### Returns

`Promise`<`boolean`\>

True on success

#### Defined in

[index.ts:116](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L116)

___

### load

▸ `Static` **load**(`options?`): `Promise`<[`SqliteStore`](SqliteStore.md)\>

Open store

**`Example`**

```ts
const db = await SqliteStore.load();
```

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `options?` | [`OpenOptions`](../interfaces/OpenOptions.md) | Opening options |

#### Returns

`Promise`<[`SqliteStore`](SqliteStore.md)\>

Api object

#### Defined in

[index.ts:61](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L61)

___

### open

▸ `Static` **open**(`path`, `options?`): `Promise`<[`SqliteStore`](SqliteStore.md)\>

Open database file

**`Example`**

```ts
const db = await SqliteStore.open('/path/to/the/db');
```

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `path` | `string` | Path to the database file |
| `options?` | [`OpenOptions`](../interfaces/OpenOptions.md) | Opening options |

#### Returns

`Promise`<[`SqliteStore`](SqliteStore.md)\>

Api object

#### Defined in

[index.ts:88](https://github.com/Sylvaner/tauri-plugin-sqlite-store/blob/409bca9/webview-src/index.ts#L88)
