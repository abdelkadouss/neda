use rusqlite::{Connection, Result};

pub mod prayers_times_db;

struct DB {
    pub conn: Connection,
    pub db_path: String,
    pub tables: Vec<Table>,
}

struct Table {
    pub name: String,
    pub schema: String,
}

impl Table {
    pub fn new(name: String, schema: String) -> Self {
        Self { name, schema }
    }
}

impl DB {
    pub fn new(db_path: String, tables: Vec<Table>) -> Result<Self> {
        let conn = Connection::open(db_path.clone())?;
        let mut db = DB {
            conn,
            db_path,
            tables,
        };
        db.make_schema()?;
        Ok(db)
    }

    fn make_table(&mut self, table: &Table) -> Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table.name, table.schema
        );
        self.conn.execute(&sql, [])?;
        Ok(())
    }

    fn make_schema(&mut self) -> Result<()> {
        for table in &self.tables {
            let sql = format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                table.name, table.schema
            );
            self.conn.execute(&sql, [])?;
        }
        Ok(())
    }

    pub fn push(&mut self, table_name: String, data: &[(&str, &dyn rusqlite::ToSql)]) -> Result<()> {
        let columns: Vec<&str> = data.iter().map(|(col, _)| *col).collect();
        let placeholders: Vec<String> = (0..data.len()).map(|i| format!("?{}", i + 1)).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        );

        let values: Vec<&dyn rusqlite::ToSql> = data.iter().map(|(_, val)| *val).collect();
        self.conn.execute(&sql, &values[..])?;

        Ok(())
    }
}
