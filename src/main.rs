use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

fn main() {
    let mut input = String::new();
    let mut db: HashMap<String, String> = HashMap::new();

    if let Ok(file) = File::open("db.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                ["D", key] => {
                    db.remove(*key);
                }
                ["S", key, value] => {
                    db.insert(key.to_string(), value.to_string());
                }
                _ => {}
            }
        }
    }

    loop {
        print!("tinyq > ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let words: Vec<&str> = input.split_whitespace().collect();

        if words.is_empty() {
            continue;
        }

        match words[0] {
            "set" => {
                if words.len() != 3 {
                    println!("Usage: set <key> <value>");
                    continue;
                }
                let key = words[1].to_string();
                let value = words[2].to_string();
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("db.txt")
                    .unwrap();

                writeln!(file, "S {} {}", key, value).unwrap();
                db.insert(key, value);
            }
            "get" => {
                if words.len() != 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                let key = words[1];
                match db.get(key) {
                    Some(value) => println!("{}", value),
                    None => println!("Key not found"),
                }
            }
            "del" => {
                if words.len() != 2 {
                    println!("Usage: del <key>");
                    continue;
                }
                let key = words[1].to_string();
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("db.txt")
                    .unwrap();
                writeln!(file, "D {}", key).unwrap();
                db.remove(&key);
            }

            "exit" => break,
            _ => println!("Unknown Command"),
        }
    }
}
