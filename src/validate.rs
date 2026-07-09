use crate::types::{Shard, SchemeConfig};
use crate::errors::VaultError;

pub fn validate_shards(shards: &[Shard], cfg: &SchemeConfig) -> Result<(), VaultError> {
    if shards.len() < cfg.threshold {
        return Err(VaultError::InsufficientShards {
            needed: cfg.threshold,
            got: shards.len(),
        });
    }

    let mut seen_x = std::collections::HashSet::new();
    for s in shards {
        if !seen_x.insert(s.x) {
            return Err(VaultError::InvalidShard(format!(
                "duplicate x-coordinate: {}", s.x
            )));
        }
    }

    Ok(())
}