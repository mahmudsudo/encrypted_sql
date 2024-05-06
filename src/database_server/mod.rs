use rusqlite::{Connection, Result};
use std::path::Path;
use std::fs::{File, read_dir};

pub(crate) struct Database {
    conn: Connection,
}

impl Database {
   pub fn new() -> Result<Database> {
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
        let mut db = Database::new()?;

        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let table_name = path.file_stem().unwrap().to_str().unwrap();
                let table_type = path.extension().unwrap().to_str().unwrap();

                if table_type == "csv" {
                    db.load_table_from_csv(&table_name, &path)?;
                }
            }
        }

        Ok(db)
    }

    // Load a table from CSV file in the directory.
    fn load_table_from_csv(&self, table_name: &str, file_path: &Path) -> Result<()> {
        let mut reader = csv::Reader::from_path(file_path)?;

        // Get the column names from the CSV file
        let headers = reader.headers().unwrap();

        // Find the index of the "value" column
        let value_column_index = headers.iter().position(|h| h == "value").unwrap();

        // Determine the table to insert into based on the column names.
        match table_name {
            "integers" => self.load_integers_from_csv(&mut reader, value_column_index)?,
            "booleans" => self.load_booleans_from_csv(&mut reader, value_column_index)?,
            _ => return Err(format!("Unknown table name: {}", table_name).into()),
        }

        Ok(())
    }

    fn load_integers_from_csv(&self, reader: &mut csv::Reader<File>, value_column_index: usize) -> Result<()> {
        for record in reader.records() {
            let record = record?;
            let value = record.get(value_column_index).unwrap().parse::<i32>()?;
            self.insert_integer(value)?;
        }

        Ok(())
    }

    fn load_booleans_from_csv(&self, reader: &mut csv::Reader<File>, value_column_index: usize) -> Result<()> {
        for record in reader.records() {
            let record = record?;
            let value = record.get(value_column_index).unwrap().parse::<bool>()?;
            self.insert_boolean(value)?;
        }

        Ok(())
    }

}