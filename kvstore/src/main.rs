use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("the key is '{}', and the value is '{}'", key, value);
    let contents = format!("{},{}\n", key, value);

    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase().clone(), value.clone());
    database.insert(key.clone(), value.clone());
    //another way of doing the same thing
    //Database::insert(database, key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        //allocate the memory for the map.
        let mut map = HashMap::new();

        //read the database file
        let contents = std::fs::read_to_string("kv.db")?;
        // the ? at the end, expands to what is below, hands the error back instead of the value;
        /*
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Err(error);
            }
        };
        */

        //read each line of the file given, as each line is an entry
        for line in contents.lines() {
            let (key, value) = line
                .split_once(',')
                .expect("Database format invalid, crashing out");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push(',');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }
}
