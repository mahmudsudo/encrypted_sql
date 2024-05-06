use rusqlite::{Connection, Result};

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
}