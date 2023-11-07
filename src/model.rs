use rusqlite::{Connection, Result};

static DB_FILE: &str = "entries.db";

pub struct Model {
    pub connection: Connection,
}

impl Model {
    pub fn new() -> Result<Model> {
        let connection = Connection::open(DB_FILE)?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS secrets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                body TEXT,
                timestamp TEXT,
                tag TEXT
            )",
                [],
            )
            ?;
        Ok(Model {connection,})
    }

    /*
    pub fn parse_datetime_from_string(datetime_str: &str) -> Result<DateTime<Utc>, ParseError> {
        let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")?;
        let utc_datetime = Utc.from_utc_datetime(&naive_datetime);
        Ok(utc_datetime)
    }

    pub fn format_datetime_for_sqlite(datetime: &DateTime<Utc>) -> String {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    */


    pub fn select(&self) -> Result<Vec<(i64, String, String, String)>> {
        let mut stmt = self.connection.prepare("SELECT * FROM secrets")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        let mut result = Vec::new();
        for row in rows {
            match row {
                Ok((c0, c1, c2, c3)) => result.push((c0, c1, c2, c3)),
                Err(err) => return Err(err),
            }
        }
        Ok(result)
    }

/* 
    pub fn insert(&self, body: String, timestamp: String, tag: String) -> Result<()> {
        self.connection.execute(
            "INSERT INTO secrets (body, timestamp, tag)
            VALUES (?, ?, ?)",
            [body, timestamp, tag],
        )?;
        Ok(())
    }
*/

    /*
        pub fn delete(&self, id: i64) -> Result<()> {
        self.connection.execute("DELETE FROM secrets WHERE id = ?", [id])?;
        Ok(())
    }*/
}

/*
fn main() -> Result<()> {
    let model = Model::new()?;

    // Use the model to interact with the database
    // For example:
    let secrets = model.select()?;
    for (id, body, timestamp, tag) in secrets {
        println!(
            "ID: {}, Body: {}, Timestamp: {}, Tag: {}",
            id, body, timestamp, tag
        );
    }

    Ok(())
}
*/
