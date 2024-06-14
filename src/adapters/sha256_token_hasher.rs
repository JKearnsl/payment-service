use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::application::common::hasher::Hasher;

pub struct Sha256TokenHasher {}

#[async_trait]
impl Hasher for Sha256TokenHasher {
    async fn hash(&self, value: &str) -> String {
        let mut hasher = Sha256::new();
        let value = value.to_owned();
        let hash = tokio::task::spawn_blocking(move || {
            hasher.update(value.as_bytes());
            format!("{:x}", hasher.finalize())
        }).await.unwrap();
        hash
    }

    async fn verify(&self, value: &str, hash: &str) -> bool {
        let mut hasher = Sha256::new();
        let value = value.to_owned();
        let hash = hash.to_owned();
        let result = tokio::task::spawn_blocking(move || {
            hasher.update(value.as_bytes());
            format!("{:x}", hasher.finalize()) == hash
        }).await.unwrap();
        result
    }
}
