use rusqlite::{Connection, Result};

static DB_FILE: &str = "entries.db";

// a row in the database
type Row = (i64, String, String, String, Vec<String>);

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
                comments JSON
            )",
            [],
        )?;
        Ok(Model { connection })
    }

    pub fn select(&self) -> Result<Vec<Row>> {
        let mut stmt = self.connection.prepare("SELECT * FROM secrets")?;
        let rows = stmt.query_map([], |row| {
            let json_text: String = row.get(4)?;
            let json_value = serde_json::from_str(&json_text).unwrap();
            let json_array: Vec<String> = serde_json::from_value(json_value).unwrap();
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                json_array,
            ))
        })?;
        let mut result = Vec::new();
        for row in rows {
            match row {
                Ok((c0, c1, c2, c3, c4)) => result.push((c0, c1, c2, c3, c4)),
                Err(err) => return Err(err),
            }
        }
        Ok(result)
    }

    pub fn insert(
        &self,
        body: String,
        timestamp: String,
        tag: String,
        comments: Vec<String>,
    ) -> Result<()> {
        let comments_json = serde_json::to_string(&comments).unwrap();
        self.connection.execute(
            "INSERT INTO secrets (body, timestamp, tag, comments)
            VALUES (?, ?, ?, ?)",
            //https://stackoverflow.com/questions/13781552/insert-a-vector-to-a-database
            [body, timestamp, tag, comments_json],
        )?;
        Ok(())
    }

    pub fn add_comment(&self, id: i64, comment: String) -> Result<()> {
        //https://www.sqlite.org/json1.html#jins
        self.connection.execute(
            "UPDATE secrets SET comments = JSON_INSERT(comments, '$[#]', ?) WHERE id = ?",
            [comment, id.to_string()],
        )?;
        Ok(())
    }

    pub fn select_by_id(&self, id: i64) -> Result<Row> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM secrets WHERE id = ?")?;
        let result = stmt.query_row([id], |row| {
            let json_text: String = row.get(4)?;
            let json_value = serde_json::from_str(&json_text).unwrap();
            let json_array: Vec<String> = serde_json::from_value(json_value).unwrap();
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                json_array,
            ))
        });
        Ok(result.unwrap())
    }
}
