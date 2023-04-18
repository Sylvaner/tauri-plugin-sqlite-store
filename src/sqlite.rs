use std::{collections::HashMap};
use rusqlite::{self, Connection, Statement, types::{Value as SqlValue, ValueRef}, params_from_iter, ParamsFromIter, Row};
use serde_json::{json, Value as JsonValue};

/// Connect to the database
/// 
/// # Arguments
/// 
/// * `path` - Path to database
/// 
/// # Example
/// 
/// ```
/// let conn = connect("/path/to/my/db.sqlite").unwrap();
/// ```
/// 
pub fn connect(path: &str) -> Result<Connection, rusqlite::Error> {
    Connection::open(path)
}

/// Close database
/// 
/// # Arguments
/// 
/// * `conn` - Database connection
///
pub fn close(conn: Connection) ->Result<bool, rusqlite::Error> {
    match conn.close() {
        Ok(()) => Ok(true),
        Err((_, e)) => Err(e)
    }
}

/// Convert a json value format to sql format
/// 
/// # Arguments
/// 
/// * `value` - Value with JSON format
/// 
/// # Example
/// 
/// ```
/// let i = json_value_to_sql(json!(2));
/// ```
/// 
fn json_value_to_sql(value: &JsonValue) -> SqlValue {
    match value {
        JsonValue::Bool(b) => SqlValue::Integer(*b as i64),
        JsonValue::String(s) => SqlValue::Text(s.to_string()),
        JsonValue::Number(n) => {
            SqlValue::Integer(n.as_i64().unwrap())
        },
        JsonValue::Array(a) => SqlValue::Text(serde_json::to_string(a).unwrap()),
        JsonValue::Object(o) => SqlValue::Text(serde_json::to_string(&o).unwrap()),
        JsonValue::Null => SqlValue::Null
    }
}

/// Bind params to a statement
/// 
/// # Arguments
/// 
/// * `params` - Array with the parameters
/// 
/// # Example
/// 
/// ```
/// let conn = connect("/path/to/my/db.sqlite").unwrap();
/// let stmt = conn.prepare("SELECT * FROM book WHERE color = ?1 AND pages > ?2").unwrap();
/// stmt.bind_params_to_statement(vec!(json!("My book"), json!("red"), json!(300)));
/// ```
/// 
fn json_params_to_sql(params: &Vec<JsonValue>) -> Result<ParamsFromIter<Vec<SqlValue>>, rusqlite::Error> {
    let mut result: Vec<SqlValue> = Vec::new();
    for param in params {
        result.push(json_value_to_sql(param));
    }
    Ok(params_from_iter(result))
}

/// Execute SQL query
/// 
/// # Arguments
/// 
/// * `conn` - Database connection
/// * `sql` - Query
/// * `params` - List of params used in the query
/// 
/// # Example
/// 
/// ```
/// // Add a book in a database (My book, red color and 300 pages)
/// let conn = connect("/path/to/my/db.sqlite").unwrap();
/// execute(&conn, "INSERT INTO book VALUES (?1, ?2, ?3)", vec!(json!("My book"), json!("red"), json!(300))).unwrap();
/// ```
/// 
pub fn execute(conn: &mut Connection, sql: &str, params: Vec<JsonValue>) -> Result<bool, rusqlite::Error> {
    if params.len() > 0 && params.get(0).unwrap().is_array() {
        let transaction = conn.transaction()?;
        for p in params {
            transaction.execute(sql, json_params_to_sql(p.as_array().unwrap())?)?;                
        }
        transaction.commit()?;
    } else {
        let mut stmt = conn.prepare(sql)?;
        let sql_params = json_params_to_sql(&params)?;
        stmt.execute(sql_params)?;
    }
    Ok(true)
}

/// Execute list of queries
/// 
/// # Arguments
/// 
/// * `conn` - Database connection
/// * `queries` - List of queries
/// 
/// # Example
/// 
/// ```ts
/// // Clean table and add 3 items
/// let mut conn = create_test_db_in_memory();
/// let mut queries: Vec<(&str, Vec<JsonValue>)> = Vec::new();
/// queries.push(("DELETE FROM test", Vec::new()));
/// queries.push(("INSERT INTO test VALUES (NULL, ?1, ?2)", serde_json::from_str("[\"Riri\", 12]").unwrap()));
/// queries.push(("INSERT INTO test VALUES (NULL, ?1, ?2)", serde_json::from_str("[\"Fifi\", 12]").unwrap()));
/// queries.push(("INSERT INTO test VALUES (NULL, ?1, ?2)", serde_json::from_str("[\"Loulou\", 12]").unwrap()));
/// batch(&mut conn, queries).unwrap();
/// ```
/// 
pub fn batch(conn: &mut Connection, queries: Vec<(&str, Vec<JsonValue>)>) -> Result<bool, rusqlite::Error> {
    let transaction = conn.transaction()?;
    for (sql, params) in queries {
        transaction.execute(sql, json_params_to_sql(&params)?)?;
    }
    transaction.commit()?;
    Ok(true)
}

/// Get list of column names
/// 
/// # Arguments
/// 
/// * `stmt` - Statement of the query
/// 
fn get_columns_names_from_statement(stmt: &Statement) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for name in stmt.column_names() {
        names.push(name.to_string());
    }
    names
}

/// Parse column data from result
/// 
/// # Arguments
/// 
/// * `value_to_parse` - Value in SQL format
/// 
fn parse_result_column(value_to_parse: ValueRef) -> JsonValue {
    match value_to_parse {
        ValueRef::Null => JsonValue::Null,
        ValueRef::Integer(i) => json!(i),
        ValueRef::Real(f) => json!(f),
        ValueRef::Text(t) => json!(std::str::from_utf8(t).unwrap()),
        ValueRef::Blob(b) => json!(b)
    }
}

/// Parse row from result
/// 
/// # Arguments
/// 
/// * `names` - List of column names
/// * `row` - Row to parse
///
fn parse_result_row(names: &Vec<String>, row: &Row) -> Result<HashMap<String, JsonValue>, rusqlite::Error> {
    let mut parsed_row: HashMap<String, JsonValue> = HashMap::new();
    for name in names.iter() {
        let column_ref = row.get_ref(name.as_str())?;
        parsed_row.insert(name.to_owned(), parse_result_column(column_ref));
    }
    Ok(parsed_row)
}

/// Query the database
/// 
/// # Arguments
/// 
/// * `connection` - Database connection
/// * `sql` - Query
/// * `params` - List of params used in the query
/// 
/// # Example
/// 
/// ```
/// // Add a book in a database (My book, red color and 300 pages)
/// let conn = connect("/path/to/my/db.sqlite").unwrap();
/// select(&conn, "SELECT * FROM book WHERE id < ?1", vec!(json!(10))).unwrap();
/// ```
/// 
pub fn select(conn: &Connection, sql: &str, params: Vec<JsonValue>) -> Result<Vec<HashMap<String, JsonValue>>, rusqlite::Error> {
    let mut stmt = conn.prepare(sql)?;
    let names = get_columns_names_from_statement(&stmt);
    let mut result: Vec<HashMap<String, JsonValue>> = Vec::new();
    let sql_params = json_params_to_sql(&params)?;
    if let Ok(mut rows) = stmt.query(sql_params) {
        loop {
            let row = match rows.next() {
                Ok(row_opt) => match row_opt {
                    Some(row) => row,
                    None => break
                },
                Err(e) => {
                    return Err(e);
                }
            };
            result.push(parse_result_row(&names, row)?);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db_in_memory() -> Connection {
        let mut conn = connect(":memory:").unwrap();
        execute(&mut conn, "CREATE TABLE test (id INTEGER PRIMARY KEY AUTOINCREMENT, name VARCHAR(20), age INTEGER)", Vec::new()).unwrap();
        execute(&mut conn, "INSERT INTO test VALUES (NULL, \"John\", 20), (NULL, \"Bill\", 31)", Vec::new()).unwrap();
        conn
    }

    #[test]
    fn memory_database() {
        let conn = connect(":memory:");
        assert!(conn.is_ok());
    }

    #[test]
    fn bad_database_path() {
        let conn = connect("\\/badpath\\ ");
        assert!(conn.is_err());
    }

    #[test]
    fn convert_json_value_to_sql() {
        assert_eq!(SqlValue::Integer(42), json_value_to_sql(&json!(42)));
        assert_eq!(SqlValue::Integer(-42), json_value_to_sql(&json!(-42)));
        assert_eq!(SqlValue::Integer(42), json_value_to_sql(&json!(42 as u64)));
        assert_eq!(SqlValue::Text("Test string".to_string()), json_value_to_sql(&json!("Test string")));
        assert_eq!(SqlValue::Text("[\"Field 1\",\"Field 2\"]".to_string()), json_value_to_sql(&json!(vec!("Field 1", "Field 2"))));
        assert_eq!(SqlValue::Text("{\"id\":1,\"name\":\"John\"}".to_string()), json_value_to_sql(&json!({
            "id": 1,
            "name": "John"
        })));
        assert_eq!(SqlValue::Integer(1), json_value_to_sql(&json!(true)));
        assert_eq!(SqlValue::Integer(0), json_value_to_sql(&json!(false)));
        assert_eq!(SqlValue::Null, json_value_to_sql(&JsonValue::Null));
    }

    #[test]
    fn test_parse_result_column() {
        assert_eq!(JsonValue::Null, parse_result_column(ValueRef::Null));
        assert_eq!(JsonValue::from(0.42), parse_result_column(ValueRef::Real(0.42)));
        assert_eq!(JsonValue::from(42), parse_result_column(ValueRef::Integer(42)));
        assert_eq!(JsonValue::from("H2G2"), parse_result_column(ValueRef::Text("H2G2".as_bytes())));
    }

    #[test]
    fn test_execute_and_select() {
        let mut conn = create_test_db_in_memory();
        execute(&mut conn, "INSERT INTO test VALUES (NULL, ?1, ?2)", vec!(json!("Bob"), json!(42))).unwrap();
        let result = select(&conn, "SELECT * FROM test WHERE id > ?1", vec!(json!(1))).unwrap();
        assert_eq!(2, result.len());
        assert_eq!(json!(2), result[0].get("id").unwrap().to_owned());
        assert_eq!(json!("Bill"), result[0].get("name").unwrap().to_owned());
        assert_eq!(json!(3), result[1].get("id").unwrap().to_owned());
        assert_eq!(json!(42), result[1].get("age").unwrap().to_owned());
    }

    #[test]
    fn test_insert_multiples() {
        let mut conn = create_test_db_in_memory();
        let exec_result = execute(&mut conn, "INSERT INTO test VALUES (NULL, ?1, ?2)", 
            vec!(
                json!(vec!(json!("Bob"), json!(30))),
                json!(vec!(json!("Dylan"), json!(31))),
                json!(vec!(json!("Mike"), json!(32))),
                json!(vec!(json!("Billy"), json!(33)))
            )
        );
        assert!(exec_result.is_ok());
        let result = select(&conn, "SELECT * FROM test", Vec::new()).unwrap();
        assert_eq!(6, result.len());
    }

    #[test]
    fn test_not_enough_parameter_execute() {
        let mut conn = create_test_db_in_memory();
        let result = execute(&mut conn, "INSERT INTO test VALUES (NULL, ?1, ?2)", vec!(json!(42)));
        assert!(result.is_err());
    }

    #[test]
    fn test_not_enough_parameter_in_array_execute() {
        let mut conn = create_test_db_in_memory();
        let result = execute(&mut conn, "INSERT INTO test VALUES (NULL, ?1, ?2)", 
            vec!(
                json!(vec!(json!("Bob"), json!(30))),
                json!(vec!(json!(42))),
                json!(vec!(json!("Dylan"), json!(31)))
            )
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_too_much_parameter_execute() {
        let mut conn = create_test_db_in_memory();
        let result = execute(&mut conn, "INSERT INTO test VALUES (NULL, ?1, ?2)", vec!(json!(42), json!(42), json!(42)));
        assert!(result.is_err());
    }

    #[test]
    fn test_batch() {
        let mut conn = create_test_db_in_memory();
        let mut queries: Vec<(&str, Vec<JsonValue>)> = Vec::new();
        queries.push(("DELETE FROM test", Vec::new()));
        queries.push(("INSERT INTO test VALUES (NULL, ?1, ?2)", serde_json::from_str("[\"Riri\", 12]").unwrap()));
        queries.push(("INSERT INTO test VALUES (NULL, ?1, ?2)", serde_json::from_str("[\"Fifi\", 12]").unwrap()));
        queries.push(("INSERT INTO test VALUES (NULL, ?1, ?2)", serde_json::from_str("[\"Loulou\", 12]").unwrap()));
        batch(&mut conn, queries).unwrap();
        let result = select(&conn, "SELECT * FROM test", Vec::new()).unwrap();
        assert_eq!(3, result.len());
        assert_eq!("Riri", result[0].get("name").unwrap());
        assert_eq!("Fifi", result[1].get("name").unwrap());
        assert_eq!("Loulou", result[2].get("name").unwrap());
    }
}