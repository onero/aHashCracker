use std::io::{self, BufRead};
use std::fs::File;
use crate::hash_util;

pub fn crack_password(hash_type: String, hash: &str, wordlist_file: &str) -> String {
    let file = File::open(wordlist_file).expect("Failed to open wordlist file");
    let reader = io::BufReader::new(file);

    println!("Opened wordlist file, will start cracking...");
    let wordlist: Vec<String> = reader.lines()
        .filter_map(|line| line.ok())
        .collect();

    let mut attempt = 1;

    for password in wordlist {
        let password_hash = hash_util::hash_password(&hash_type, &password);

        if hash.to_lowercase() == password_hash.to_lowercase() {
            println!("Cracked password: Attempt: {}", attempt);
            return password;
        } else {
            println!("[{}] No match...", attempt);
        }

        attempt += 1;
    }
    return String::from("");
}