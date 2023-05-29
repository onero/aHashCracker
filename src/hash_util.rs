use ring::digest::{digest, SHA256 as SHA256Algorithm, SHA512 as SHA512Algorithm};
use md5::compute as md5_compute;
use hex::encode as hex_encode;
use ntlm_hash::*;

pub const MD5: &str = "MD5";
pub const MD5_IDENTIFIER: &str = "$1$";
pub const SHA256: &str = "SHA256";
pub const SHA256_IDENTIFIER: &str = "$5$";
pub const SHA512: &str = "SHA512";
pub const SHA512_IDENTIFIER: &str = "$6$";
pub const NTLM: &str = "NTLM";

pub fn identify_hash(hash: &str) -> (String, String) {
    // Check if the hash matches MD5 hash
    println!("Checking if hash is MD5...");
    if hash.starts_with(MD5_IDENTIFIER) {
        println!("MD5 hash found!");
        return (MD5.to_owned(), hash[MD5_IDENTIFIER.len()..].to_owned());
    }

    // Check if the hash matches SHA256 hash
    println!("Checking if hash is SHA256...");
    if hash.starts_with(SHA256_IDENTIFIER) {
        println!("SHA256 hash found!");
        return (SHA256.to_owned(), hash[SHA256_IDENTIFIER.len()..].to_owned());
    }

    // Check if the hash matches SHA512 hash
    println!("Checking if hash is SHA512...");
    if hash.starts_with(SHA512_IDENTIFIER) {
        println!("SHA512 hash found!");
        return (SHA512.to_owned(), hash[SHA512_IDENTIFIER.len()..].to_owned());
    }

    // Return an empty string if the hash does not match any supported hash
    println!("Unknown hash!");
    ("".to_owned(), "".to_owned())
}

pub fn hash_password(hash_type: &str, password: &str) -> String {
    match hash_type {
        MD5 => {
            let result = md5_compute(password.as_bytes());
            hex_encode(result.as_ref())
        },
        SHA256 => {
            let sha256_hash = digest(&SHA256Algorithm, password.as_bytes());
            hex_encode(sha256_hash.as_ref())
        },
        SHA512 => {
            let sha512_hash = digest(&SHA512Algorithm, password.as_bytes());
            hex_encode(sha512_hash.as_ref())
        },
        NTLM => {
            ntlm_hash(&password)
        },
        _ => {
            println!("Unknown hash type");
            String::new()
        },
    }
}
