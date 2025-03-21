use std::sync::Arc;

#[cfg(feature = "bench")]
pub mod bench;
#[cfg(any(test, feature = "test-utils"))]
pub mod test;

#[cfg(any(test, feature = "test-utils"))]
pub use self::test::*;

pub mod hash {
    pub use xmtp_cryptography::hash::sha256_bytes as sha256;
}

pub mod time {
    const SECS_IN_30_DAYS: i64 = 60 * 60 * 24 * 30;

    /// Current hmac epoch. HMAC keys change every 30 days
    pub fn hmac_epoch() -> i64 {
        xmtp_common::time::now_secs() / SECS_IN_30_DAYS
    }
}

pub mod id {
    /// Relies on a client-created idempotency_key (which could be a timestamp)
    pub fn calculate_message_id(
        group_id: &[u8],
        decrypted_message_bytes: &[u8],
        idempotency_key: &str,
    ) -> Vec<u8> {
        let separator = b"\t";
        let mut id_vec = Vec::new();
        id_vec.extend_from_slice(group_id);
        id_vec.extend_from_slice(separator);
        id_vec.extend_from_slice(idempotency_key.as_bytes());
        id_vec.extend_from_slice(separator);
        id_vec.extend_from_slice(decrypted_message_bytes);
        super::hash::sha256(&id_vec)
    }

    pub fn serialize_group_id(group_id: &[u8]) -> String {
        hex::encode(group_id)
    }
}

#[derive(Clone, Debug)]
pub struct VersionInfo {
    pkg_version: Arc<str>,
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            pkg_version: env!("CARGO_PKG_VERSION").into(),
        }
    }
}

impl VersionInfo {
    pub fn pkg_version(&self) -> &str {
        &self.pkg_version
    }

    // Test only function to update the version of the client
    #[cfg(test)]
    pub fn test_update_version(&mut self, version: &str) {
        self.pkg_version = version.into();
    }
}
