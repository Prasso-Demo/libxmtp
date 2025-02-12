use super::{member::Passkey, AssociationError, MemberIdentifier, MemberKind};
use sha2::{Digest, Sha256};
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub enum PublicIdentifier {
    Ethereum(String),
    Passkey([u8; Passkey::KEY_SIZE]),
    Installation(Vec<u8>),
}
impl PublicIdentifier {
    pub fn to_lowercase(self) -> Self {
        match self {
            Self::Ethereum(addr) => Self::Ethereum(addr.to_lowercase()),
            ident => ident,
        }
    }

    /// Get the generated inbox_id for this public identifier.
    /// The same public identifier will always give the same inbox_id.
    pub fn get_inbox_id(&self, nonce: u64) -> Result<String, AssociationError> {
        if !self.is_valid_address() {
            return Err(AssociationError::InvalidAccountAddress);
        }
        Ok(sha256_string(format!("{self}{nonce}")))
    }

    /// Validates that the account address is exactly 42 characters, starts with "0x",
    /// and contains only valid hex digits.
    fn is_valid_address(&self) -> bool {
        match self {
            PublicIdentifier::Ethereum(addr) => {
                addr.len() == 42
                    && addr.starts_with("0x")
                    && addr[2..].chars().all(|c| c.is_ascii_hexdigit())
            }
            _ => true,
        }
    }

    pub fn kind(&self) -> MemberKind {
        match self {
            Self::Ethereum(_) => MemberKind::Ethereum,
            Self::Installation(_) => MemberKind::Installation,
            Self::Passkey(_) => MemberKind::Passkey,
        }
    }
}

impl Display for PublicIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let addr;
        let output = match self {
            Self::Ethereum(addr) => addr,
            Self::Passkey(key) => {
                addr = hex::encode(key);
                &addr
            }
            Self::Installation(key) => {
                addr = hex::encode(key);
                &addr
            }
        };
        write!(f, "{output}")
    }
}
impl Debug for PublicIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Self::Ethereum(_) => "Ethereum",
            Self::Passkey(_) => "Passkey",
            Self::Installation(_) => "Installation",
        };
        write!(f, "{kind}: {self}")
    }
}

impl From<MemberIdentifier> for PublicIdentifier {
    fn from(ident: MemberIdentifier) -> Self {
        match ident {
            MemberIdentifier::Ethereum(addr) => Self::Ethereum(addr),
            MemberIdentifier::Installation(key) => Self::Installation(key),
            MemberIdentifier::Passkey(Passkey { public_key, .. }) => Self::Passkey(public_key),
        }
    }
}

impl PartialEq<MemberIdentifier> for PublicIdentifier {
    fn eq(&self, other: &MemberIdentifier) -> bool {
        match self {
            Self::Ethereum(addr) => match other {
                MemberIdentifier::Ethereum(other_addr) => addr == other_addr,
                _ => false,
            },
            Self::Passkey(key) => match other {
                MemberIdentifier::Passkey(Passkey {
                    public_key: other_key,
                    ..
                }) => key == other_key,
                _ => false,
            },
            Self::Installation(key) => match other {
                MemberIdentifier::Installation(other_key) => key == other_key,
                _ => false,
            },
        }
    }
}
impl PartialEq<PublicIdentifier> for MemberIdentifier {
    fn eq(&self, other: &PublicIdentifier) -> bool {
        other == self
    }
}

/// Helper function to generate a SHA256 hash as a hex string.
fn sha256_string(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}
