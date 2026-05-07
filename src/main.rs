use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum LogEntry {
    Set { key: String, value: String },
    Del { key: String },
}

fn main() {
    let mut input = String::new();
    let mut db: HashMap<String, String> = HashMap::new();

    // Clean up tmp file
    let _ = std::fs::remove_file("db.txt.tmp");

    // Replay log
    if let Ok(file) = File::open("db.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<LogEntry>(&line) {
                Ok(LogEntry::Set { key, value }) => {
                    db.insert(key, value);
                }
                Ok(LogEntry::Del { key }) => {
                    db.remove(&key);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse line: {} -> {}", line, e);
                }
            }
        }
    }

    loop {
        print!("tinyq > ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let words: Vec<&str> = input.trim().split_whitespace().collect();
        if words.is_empty() {
            continue;
        }

        match words[0] {
            "set" => {
                if words.len() < 3 {
                    println!("Usage: set <key> <value...>");
                    continue;
                }
                let key = words[1].to_string();
                let value = words[2..].join(" ");

                let entry = LogEntry::Set {
                    key: key.clone(),
                    value: value.clone(),
                };

                append_to_log(&entry);
                db.insert(key, value);
                println!("OK");
            }

            "get" => {
                if words.len() != 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                match db.get(words[1]) {
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

                let entry = LogEntry::Del { key: key.clone() };
                append_to_log(&entry);
                db.remove(&key);
                println!("OK");
            }

            "keys" | "ls" => {
                if db.is_empty() {
                    println!("(empty)");
                } else {
                    let mut keys: Vec<_> = db.keys().collect();
                    keys.sort();
                    for key in keys {
                        println!("{}", key);
                    }
                    println!("Total: {} keys", db.len());
                }
            }

            "compact" => {
                let tmp_path = "db.txt.tmp";
                let mut tmp_file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(tmp_path)
                    .unwrap();

                for (key, value) in &db {
                    let entry = LogEntry::Set {
                        key: key.clone(),
                        value: value.clone(),
                    };
                    writeln!(tmp_file, "{}", serde_json::to_string(&entry).unwrap()).unwrap();
                }

                tmp_file.sync_all().unwrap();
                std::fs::rename(tmp_path, "db.txt").unwrap();
                println!("Compacted: {} keys", db.len());
            }

            "exit" => break,
            _ => println!("Unknown command. Available: set, get, del, keys, compact, exit"),
        }
    }
}

fn append_to_log(entry: &LogEntry) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("db.txt")
        .unwrap();

    writeln!(file, "{}", serde_json::to_string(entry).unwrap()).unwrap();
    file.sync_all().unwrap();
}
