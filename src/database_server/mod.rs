use std::collections::HashMap;
use std::fmt;
use std::fs::read_dir;
use std::path::Path;

use crate::Tables;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub(crate) enum AppError {
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
    Csv(csv::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::Sqlite(ref err) => write!(f, "SQLite error: {}", err),
            AppError::Io(ref err) => write!(f, "IO error: {}", err),
            AppError::Csv(ref err) => write!(f, "CSV error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> AppError {
        AppError::Sqlite(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> AppError {
        AppError::Csv(err)
    }
}

pub(crate) struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Database, AppError> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE integers (id INTEGER PRIMARY KEY, value INTEGER);",
            [],
        )?;
        conn.execute(
            "CREATE TABLE booleans (id INTEGER PRIMARY KEY, value BOOLEAN);",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn load_from_directory(path: &Path) -> Result<Database, AppError> {
        let db = Database::new()?;
        println!("Database initialized in memory.");

        let entries = read_dir(path).map_err(AppError::Io)?;
        for entry in entries {
            let entry = entry.map_err(AppError::Io)?;
            let file_path = entry.path();
            println!("Considering file: {}", file_path.display());

            if file_path.is_file() && file_path.extension().unwrap_or_default() == "csv" {
                println!("Loading CSV file: \"{}\"", file_path.display());
                db.load_table_from_csv(&file_path)?;
            }
        }

        Ok(db)
    }

    // Load a table from a CSV file in the directory.
    pub fn load_table_from_csv(&self, file_path: &Path) -> Result<(), AppError> {
        let mut reader = csv::Reader::from_path(file_path)?;
        let headers = reader
            .headers()?
            .iter()
            .map(|h| h.split(':').next().unwrap().to_string())
            .collect::<Vec<String>>();

        let table_name = file_path.file_stem().unwrap().to_str().unwrap();
        println!("Processing CSV for table: {}", table_name);

        self.ensure_table(&headers, file_path)?;

        for record in reader.records() {
            let record = record?;
            let values: Vec<String> = record
                .iter()
                .map(|value| format!("'{}'", value.replace("'", "''")))
                .collect();
            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({});",
                table_name,
                headers.join(","),
                values.join(",")
            );
            println!("Executing SQL: {}", sql);
            self.conn.execute(&sql, []).map_err(AppError::Sqlite)?;
        }

        Ok(())
    }

    // Ensure a table exists with the appropriate columns and types based on CSV headers
    pub fn ensure_table(&self, headers: &[String], file_path: &Path) -> Result<(), AppError> {
        let table_name = file_path.file_stem().unwrap().to_str().unwrap();
        println!("Ensuring table structure for: {}", table_name);

        let sql_check_table_exists =
            format!("SELECT name FROM sqlite_master WHERE type='table' AND name=?;");
        let table_exists: bool = self
            .conn
            .query_row(&sql_check_table_exists, &[table_name], |_| Ok(()))
            .is_ok();

        if !table_exists {
            println!("Table does not exist. Creating new table: {}", table_name);

            let column_definitions: Vec<String> = headers
                .iter()
                .map(|header| {
                    let parts: Vec<&str> = header.split(':').collect();
                    format!(
                        "{} {}",
                        parts[0],
                        match parts.get(1) {
                            Some(&"uint32") | Some(&"int32") | Some(&"uint16") | Some(&"int16")
                            | Some(&"uint8") | Some(&"int8") => "INTEGER",
                            Some(&"bool") => "BOOLEAN",
                            Some(&"string") => "TEXT",
                            _ => "TEXT",
                        }
                    )
                })
                .collect();

            let columns_sql = column_definitions.join(", ");
            let sql_create_table = format!("CREATE TABLE {} ({});", table_name, columns_sql);
            self.conn
                .execute(&sql_create_table, [])
                .map_err(AppError::Sqlite)?;
            println!("Table created successfully: {}", table_name);
        } else {
            println!("Table {} already exists. Skipping creation.", table_name);
        }

        Ok(())
    }

    // Retrieves data from a named table and converts each row into a HashMap.
    pub fn retrieve_table_data(
        &self,
        table_name: &str,
    ) -> Result<Vec<HashMap<String, String>>, AppError> {
        let mut stmt = self
            .conn
            .prepare(&format!("SELECT * FROM {}", table_name))
            .map_err(AppError::Sqlite)?;

        let rows = stmt
            .query_map([], |row| {
                let mut result = HashMap::new();
                // Assume columns are known or dynamically determined.
                result.insert("id".to_string(), row.get::<_, String>("id").unwrap());
                Ok(result)
            })
            .map_err(AppError::Sqlite)?;

        // Map each row result from rusqlite::Error to AppError and collect
        rows.map(|row_result| row_result.map_err(AppError::Sqlite))
            .collect()
    }

    // Additional method to convert the database content to the Tables structure
    pub fn to_tables(&self) -> Result<Tables, AppError> {
        let mut tables = Tables::new();

        // First, get all table names from the database
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table';")?;
        let table_names = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?;

        // Iterate over each table name and retrieve its data
        for table_name in table_names {
            let table_name = table_name?;
            let data = self.retrieve_table_data(&table_name)?;
            tables.tables.insert(table_name, data);
        }

        Ok(tables)
    }
}
