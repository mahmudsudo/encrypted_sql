use std::fmt;
use std::fs::read_dir;
use std::path::Path;

use rusqlite::{Connection, Result};

#[derive(Debug)]
enum AppError {
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
        conn.execute("CREATE TABLE integers (id INTEGER PRIMARY KEY, value INTEGER);", [])?;
        conn.execute("CREATE TABLE booleans (id INTEGER PRIMARY KEY, value BOOLEAN);", [])?;
        Ok(Database { conn })
    }

    pub fn insert_integer(&self, value: i32) -> Result<()> {
        self.conn.execute("INSERT INTO integers (value) VALUES (?);", (value,))?;
        Ok(())
    }

   pub  fn insert_boolean(&self, value: bool) -> Result<()> {
        self.conn.execute("INSERT INTO booleans (value) VALUES (?);", (value,))?;
        Ok(())
    }

    pub fn get_integer(&self, id: i32) -> Result<Option<i32>> {
        let mut stmt = self.conn.prepare("SELECT value FROM integers WHERE id = ?;")?;
        let row = stmt.query_row((id,), |row| row.get(0))?;
        Ok(row)
    }

   pub fn get_boolean(&self, id: i32) -> Result<Option<bool>> {
        let mut stmt = self.conn.prepare("SELECT value FROM booleans WHERE id = ?;")?;
        let row = stmt.query_row((id,), |row| row.get(0))?;
        Ok(row)
    }

    pub fn load_from_directory(path: &Path) -> Result<Database, AppError> {
        let db = Database::new()?;

        for entry in read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_file() && file_path.extension().unwrap_or_default() == "csv" {
                db.load_table_from_csv(&file_path)?;
            }
        }

        Ok(db)
    }

    // Load a table from a CSV file in the directory.
    pub fn load_table_from_csv(&self, file_path: &Path) -> Result<(), AppError> {
        let mut reader = csv::Reader::from_path(file_path)?;
        let headers = reader.headers()?.iter().map(String::from).collect::<Vec<String>>();

        self.ensure_table(&headers, file_path)?;

        for record in reader.records() {
            let record = record?;
            let values: Vec<String> = record.iter().map(|value| format!("'{}'", value.replace("'", "''"))).collect();
            let sql = format!("INSERT INTO my_table ({}) VALUES ({});", headers.join(","), values.join(","));
            self.conn.execute(&sql, [])?;
        }

        Ok(())
    }

    // Ensure a table exists with the appropriate columns and types based on CSV headers
    pub fn ensure_table(&self, headers: &[String], file_path: &Path) -> Result<(), AppError> {
        let table_name = file_path.file_stem().unwrap().to_str().unwrap();

        let exists = self.conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name=?;",
            &[table_name],
            |_| Ok(())
        ).is_ok();

        if !exists {
            let columns_sql = headers.iter().map(|header| {
                let parts: Vec<&str> = header.split(':').collect();
                format!("{} {}", parts[0], match parts.get(1) {
                    Some(&"uint32") => "INTEGER",
                    Some(&"bool") => "BOOLEAN",
                    Some(&"string") => "TEXT",
                    _ => "TEXT",
                })
            }).collect::<Vec<String>>().join(", ");
            let sql = format!("CREATE TABLE {} (id INTEGER PRIMARY KEY AUTOINCREMENT, {});", table_name, columns_sql);
            self.conn.execute(&sql, [])?;
        }

        Ok(())
    }
}