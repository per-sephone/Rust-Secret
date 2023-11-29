// Used ChatGPT for help generating rust doc comments
use rusqlite::{Connection, Result};

/// The file path for the SQLite database.
static DB_FILE: &str = "entries.db";

/// Represents a row in the database, consisting of an ID, body, timestamp, tag, and comments.
type Row = (i64, String, String, String, Vec<String>);

/// A struct representing the database model.
pub struct Model {
    /// The SQLite database connection.
    pub connection: Connection,
}

impl Model {
    /// Creates a new instance of `Model` with a connection to the SQLite database.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection to the database cannot be established or if the
    /// creation of the 'secrets' table fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::Model;
    ///
    /// match Model::new() {
    ///     Ok(model) => println!("Model created successfully"),
    ///     Err(err) => eprintln!("Error creating model: {:?}", err),
    /// }
    /// ```
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

    /// Retrieves all rows from the 'secrets' table in the database.
    ///
    /// # Errors
    ///
    /// Returns an error if the SQL query or result processing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::Model;
    ///
    /// let model = Model::new().expect("Failed to create model");
    ///
    /// match model.select() {
    ///     Ok(rows) => println!("Rows retrieved successfully: {:?}", rows),
    ///     Err(err) => eprintln!("Error retrieving rows: {:?}", err),
    /// }
    /// ```

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

    /// Inserts a new row into the 'secrets' table in the database.
    ///
    /// # Arguments
    ///
    /// * `body` - The body of the secret.
    /// * `timestamp` - The timestamp of the secret.
    /// * `tag` - The tag associated with the secret.
    /// * `comments` - A vector of comments associated with the secret.
    ///
    /// # Errors
    ///
    /// Returns an error if the SQL insert operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::Model;
    ///
    /// let model = Model::new().expect("Failed to create model");
    ///
    /// match model.insert("Body".to_string(), "Timestamp".to_string(), "Tag".to_string(), vec!["Comment".to_string()]) {
    ///     Ok(_) => println!("Row inserted successfully"),
    ///     Err(err) => eprintln!("Error inserting row: {:?}", err),
    /// }
    /// ```
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

    /// Adds a new comment to the 'comments' field of the row with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the row to which the comment should be added.
    /// * `comment` - The comment to be added to the row.
    ///
    /// # Errors
    ///
    /// Returns an error if the SQL update operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::Model;
    ///
    /// let model = Model::new().expect("Failed to create model");
    ///
    /// match model.add_comment(1, "New comment".to_string()) {
    ///     Ok(_) => println!("Comment added successfully"),
    ///     Err(err) => eprintln!("Error adding comment: {:?}", err),
    /// }
    /// ```
    pub fn add_comment(&self, id: i64, comment: String) -> Result<()> {
        //https://www.sqlite.org/json1.html#jins
        self.connection.execute(
            "UPDATE secrets SET comments = JSON_INSERT(comments, '$[#]', ?) WHERE id = ?",
            [comment, id.to_string()],
        )?;
        Ok(())
    }

    /// Retrieves a single row from the 'secrets' table in the database based on the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the secret to retrieve.
    ///
    /// # Errors
    ///
    /// Returns an error if the SQL query or result processing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use model::Model;
    ///
    /// let model = Model::new().expect("Failed to create model");
    ///
    /// match model.select_by_id(1) {
    ///     Ok(row) => println!("Row retrieved successfully: {:?}", row),
    ///     Err(err) => eprintln!("Error retrieving row: {:?}", err),
    /// }
    /// ```
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
