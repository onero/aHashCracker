pub const MD5: &str = "MD5";
pub const MD5_IDENTIFIER: &str = "$1$";
pub const SHA256: &str = "SHA256";
pub const SHA256_IDENTIFIER: &str = "$5$";
pub const SHA512: &str = "SHA512";
pub const SHA512_IDENTIFIER: &str = "$6$";

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