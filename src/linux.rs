use std::io::{self, BufRead};
use std::fs::File;
use ring::digest::{digest, SHA256, SHA512};
use md5::compute as md5_compute;
use hex::encode as hex_encode;
use crate::hash_util;

pub fn crack_linux_password(hash_type: String, hash: &str, wordlist_file: &str) -> String {
    let file = File::open(wordlist_file).expect("Failed to open wordlist file");
    let reader = io::BufReader::new(file);

    println!("Opened wordlist file, will start cracking...");
    let wordlist: Vec<String> = reader.lines()
        .filter_map(|line| line.ok())
        .collect();

    let mut attempt = 1;

    for password in wordlist {
        let password_hash = if hash_type == hash_util::MD5 {
            let result = md5_compute(password.as_bytes());
            hex_encode(result.as_ref())
        } else if hash_type == hash_util::SHA256 {
            let sha256_hash = digest(&SHA256, password.as_bytes());
            hex_encode(sha256_hash.as_ref())
        } else if hash_type == hash_util::SHA512 {
            let sha512_hash = digest(&SHA512, password.as_bytes());
            hex_encode(sha512_hash.as_ref())
        } else {
            println!("Unknown hash type");
            break;
        };

        if hash == password_hash {
            println!("Cracked Linux password: Attempt: {}", attempt);
            return password;
        } else {
            println!("[{}] No match...", attempt);
        }

        attempt += 1;
    }
    return String::from("");
}