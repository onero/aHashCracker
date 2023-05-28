use std::io::{self};
mod linux;
mod hash_util;
use linux::crack_linux_password;
use hash_util::identify_hash;

fn main() {
    let mut input_line = String::new();
    println!("Enter the line from /etc/shadow: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    let parts: Vec<&str> = input_line.split(':').collect();
    if parts.len() < 2 {
        println!("Invalid input");
        return;
    }

    let username = parts[0].trim();
    let trimmed_hash = parts[1].trim();

    // Check if the hash belongs to Linux or Windows
    if trimmed_hash.starts_with("$") {
        println!("Identified hash as Linux password");
        let (hash_type, hash_without_prefix) = identify_hash(trimmed_hash);
        let wordlist_file = "../wordlist.txt";
        let cracked_password = crack_linux_password(hash_type, &hash_without_prefix, wordlist_file);
        println!("Result: {}:{}", username, cracked_password);
    } else {
        println!("Unable to identify the hash type");
    }
}


