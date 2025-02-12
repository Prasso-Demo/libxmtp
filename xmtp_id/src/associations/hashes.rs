use sha2::{Digest, Sha256};

use super::{state::PublicIdentifier, AssociationError};

/// Helper function to generate a SHA256 hash as a hex string.
fn sha256_string(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Validates that the account address is exactly 42 characters, starts with "0x",
/// and contains only valid hex digits.
fn is_valid_address(account_address: &PublicIdentifier) -> bool {
    match account_address {
        PublicIdentifier::Ethereum(addr) => {
            addr.len() == 42
                && addr.starts_with("0x")
                && addr[2..].chars().all(|c| c.is_ascii_hexdigit())
        }
        _ => true,
    }
}

/// Generates an inbox ID if the account address is valid.
pub fn generate_inbox_id(
    identifier: &PublicIdentifier,
    nonce: &u64,
) -> Result<String, AssociationError> {
    if !is_valid_address(identifier) {
        return Err(AssociationError::InvalidAccountAddress);
    }
    Ok(sha256_string(format!("{identifier}{nonce}")))
}
