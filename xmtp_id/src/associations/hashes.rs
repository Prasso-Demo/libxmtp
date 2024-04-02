use sha2::{Digest, Sha256};

pub fn sha256_string(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn generate_xid(account_address: &String, nonce: &u32) -> String {
    sha256_string(format!("{}{}", account_address, nonce))
}
