use crate::{Data, History, Operations};
use rusqlite::{params, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn }) // Self is for type.
    }
    pub fn create_tables(&self) -> Result<()> {
        // self is for reference.
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS history(
                id INTEGER PRIMARY KEY,
                num1 REAL NOT NULL,
                op CHAR(1) NOT NULL,
                num2 REAL NOT NULL,
                result REAL NOT NULL
                )",
            [],
        )?;
        Ok(())
    }

    pub fn save_in_db(&self, history: &History) -> Result<()> {
        self.conn.execute(
            "INSERT INTO history(num1, op, num2, result) VALUES (?1, ?2, ?3, ?4)",
            params![
                &history.data.num1,
                &history.data.op.as_char().to_string(),
                &history.data.num2,
                &history.result
            ],
        )?;
        Ok(())
    }
    pub fn read_db(&self) -> Result<Vec<History>> {
        let mut stmt = self
            .conn
            .prepare("SELECT num1, op, num2, result FROM history")?;

        let history_iter = stmt.query_map([], |row| {
            Ok(History {
                data: Data {
                    num1: row.get(0)?,
                    op: Operations::from_char(
                        row.get::<_, String>(1)?.chars().next().unwrap_or(' '),
                    )
                    .ok_or(rusqlite::Error::InvalidQuery)?,
                    num2: row.get(2)?,
                },
                result: row.get(3)?,
            })
        })?;
        let mut results = Vec::new();
        for item in history_iter {
            results.push(item?);
        }

        Ok(results)
    }

    pub fn delete_db(&self) -> Result<()> {
        self.conn.execute("DELETE FROM history", [])?;
        Ok(())
    }
}
