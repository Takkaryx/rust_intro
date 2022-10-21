use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("the key is '{}', and the value is '{}'", key, value);
    let contents = format!("{},{}\n", key, value);
    std::fs::write("kv.db", contents);

    let database = Database::new().expect("Database::new() crashed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
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

        let mut map = HashMap::new();
        //read each line of the file given, as each line is an entry
        for line in contents.lines() {
            let (key, value) = line
                .split_once(',')
                .expect("Database format invalid, crashing out");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map: map })
    }
}
