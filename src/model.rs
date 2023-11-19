use rusqlite::{Connection, Result};

static DB_FILE: &str = "entries.db";

pub struct Model {
    pub connection: Connection,
}

impl Model {
    pub fn new() -> Result<Model> {
        let connection = Connection::open(DB_FILE)?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                body TEXT,
                timestamp TEXT,
                tag TEXT,
                comments TEXT
            )",
            [],
        )?;
        Ok(Model { connection })
    }

    pub fn select(&self) -> Result<Vec<(i64, String, String, String, Vec<String>)>> {
        let mut stmt = self.connection.prepare("SELECT * FROM secrets")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, serde_json::from_str::<Vec<String>>(row.get::<usize, String>(4)?.as_str()).unwrap()))
        })?;
        let mut result = Vec::new();
        for row in rows {
            match row {
                Ok((c0, c1, c2, c3, c4)) => result.push((c0, c1, c2, c3,c4)),
                Err(err) => return Err(err),
            }
        }
        Ok(result)
    }

    pub fn insert(&self, body: String, timestamp: String, tag: String, comments: Vec<String>) -> Result<()> {
        self.connection.execute(
            "INSERT INTO secrets (body, timestamp, tag, comments)
            VALUES (?, ?, ?, ?)",
            [body, timestamp, tag, serde_json::to_string(&comments).unwrap()],
        )?;
        Ok(())
    }


    pub fn add_comment(&self, id: i64, comment: String) -> Result<()> {
        self.connection.execute("UPDATE secrets SET comments = JSON_ARRAY_APPEND(comments, '$', ?) WHERE id = ?", [comment, id.to_string()])?;
        Ok(())
    }

    pub fn select_by_id(&self, id: i64) -> Result<(i64, String, String, String, Vec<String>)>  {
        let mut stmt = self.connection.prepare("SELECT * FROM secrets WHERE id = ?")?;
        let result = stmt.query_row([id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                serde_json::from_str::<Vec<String>>(row.get::<usize, String>(4)?.as_str()).unwrap(),
            ))
        });
        Ok(result.unwrap())
    }

    
}