use std::io::{self};
mod cracking;
mod hash_util;

fn main() {
    let mut input_line = String::new();
    println!("Enter the line from /etc/shadow or SAM DB: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    let parts: Vec<&str> = input_line.split(':').collect();
    if parts.len() < 2 {
        println!("Invalid input");
        return;
    }

    let username = parts[0].trim();
    let trimmed_linux_hash = parts[1].trim();
    let trimmed_windows_hash = parts[3].trim(); // NTLM hash!

    let wordlist_file = "../wordlist.txt";

    // Check if the hash belongs to Linux or Windows
    if trimmed_linux_hash.starts_with("$") {
        println!("Identified hash as Linux password");
        let (hash_type, hash_without_prefix) = hash_util::identify_hash(trimmed_linux_hash);
        let cracked_password = cracking::crack_password(hash_type, &hash_without_prefix, wordlist_file);
        println!("Result: {}:{}", username, cracked_password);
    } else if trimmed_windows_hash.len() == 32 {  // NTLM hashes are 32 characters long
        println!("Identified hash as Windows NTLM hash");
        let hash_type = hash_util::NTLM.to_owned();
        let cracked_password = cracking::crack_password(hash_type, trimmed_windows_hash, wordlist_file);
        println!("Result: {}:{}", username, cracked_password);
    } else {
        println!("Unable to identify the hash type");
    }
}


