fn main() {
    // Create db
    let mut db = match Database::new("kv.db") {
        Ok(db) => db,
        _ => {
            println!("Failed to read database");
            return ();
        },
    };

    // Handle request
    let mut args = std::env::args().skip(1);
    let key = args.next();
    let value = args.next();
    match key {
        None => {
            println!("Usage: kvstore [key] [value]");
            return;
        },
        Some(key) => match value {
            None => {
                // Key only -> query from database
                match db.query(&key) {
                    Some(value) => println!("{}", value),
                    None => println!("KEY NOT FOUND"),
                }
                match db.flush() {
                    _ => (),
                };
            },
            Some(value) => {
                // Key and value -> insert to database
                db.insert(&key, &value);
                match db.flush() {
                    _ => (),
                };
            },
        },
    }
}

struct Database {
    path: String,
    store: std::collections::HashMap<String, String>,
}

impl Database {
    /// Returns a database associated with the given path
    ///
    /// # Arguments
    ///
    /// * `path` - a string indicating the location of the database in the file system
    fn new(path: &str) -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string(path)?;
        let mut store = std::collections::HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split('\t').collect();
            if chunks.len() != 2 {
                todo!("Return error");
            }
            let key = chunks[0].to_string();
            let value = chunks[1].to_string();
            store.insert(key, value);
        }

        return Ok(Database {
            path: path.to_string(),
            store: store,
        })
    }

    /// Inserts a key-value pair into the database
    ///
    /// # Arguments
    ///
    /// * `key` - A string with no tab character
    /// * `value` - The value associated with key in the database
    fn insert(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_string(), value.to_string());
    }

    /// Returns the value associated with the given key in the database store
    ///
    /// # Arguments
    ///
    /// * `key` - A string with no tab character
    fn query(&self, key: &str) -> Option<String> {
        return match self.store.get(key) {
            Some(v) => Some(v.to_string()),
            None => None,
        };
    }

    /// Writes the contents of the Database to the file system
    fn flush(&self) -> std::io::Result<()> {
        let contents = self.store.iter()
            .map(|(key, value)| format!("{}\t{}", key, value))
            .collect::<Vec<String>>()
            .join("\n");
        return std::fs::write(&self.path, contents);
    }
}
