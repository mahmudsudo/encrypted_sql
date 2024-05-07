use rusqlite::{Connection, Result, params};
use std::path::Path;
use std::fs::{File, read_dir};
use csv::StringRecord;

pub(crate) struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Database, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        conn.execute("CREATE TABLE integers (id INTEGER PRIMARY KEY, value INTEGER);",[])?;
        conn.execute("CREATE TABLE booleans (id INTEGER PRIMARY KEY, value BOOLEAN);",[])?;
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

    // Load a Database from directory.
    pub fn load_from_directory(path: &Path) -> Result<Database> {
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

    // Load a table from CSV file in the directory.
    pub fn load_table_from_csv(&self, file_path: &Path) -> Result<()> {
        let mut reader = csv::Reader::from_path(file_path)?;
        let mut headers = reader.headers()?.clone();

        // Assume a function that ensures a table with appropriate columns exists or is created.
        self.ensure_table(&headers)?;

        for record in reader.records() {
            let record = record?;
            let values: Vec<String> = record.iter().map(|v| format!("'{}'", v)).collect();
            let sql = format!("INSERT INTO my_table ({}) VALUES ({});", headers.join(","), values.join(","));
            self.conn.execute(&sql, [])?;
        }
        Ok(())
    }

    // Ensure a table exists with the appropriate columns and types.
    pub fn ensure_table(&self, headers: &StringRecord) -> Result<()> {
        let table_name = "dynamic_table"; // TODO: derive this from the CSV file name or another source.

        // 1. Start by checking if the table exists and get existing columns.
        let exists = self.conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name=?;",
            params![table_name],
            |_| Ok(()),
        ).is_ok();

        if !exists {
            // 2. Create table if it does not exist.
            let columns = headers.iter().map(|header| {
                let parts: Vec<&str> = header.split(':').collect();
                if parts.len() != 2 {
                    return Err(rusqlite::Error::ExecuteReturnedResults); // Improvised error handling.
                }
                match parts[1] {
                    "uint32" => Ok(format!("{} INTEGER", parts[0])),
                    "bool" => Ok(format!("{} BOOLEAN", parts[0])),
                    "string" => Ok(format!("{} TEXT", parts[0])),
                    _ => Err(rusqlite::Error::ExecuteReturnedResults), // Handle unknown types.
                }
            }).collect::<Result<Vec<String>, _>>()?;

            let columns_sql = columns.join(", ");
            let sql = format!("CREATE TABLE {} (id INTEGER PRIMARY KEY AUTOINCREMENT, {});", table_name, columns_sql);
            self.conn.execute(&sql, params![])?;
        } else {
            // 3. If the table exists, how to handle schema changes?
        }

        Ok(())
    }
}