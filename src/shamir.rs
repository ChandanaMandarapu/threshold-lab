use crate::lagrange::reconstruct_secret;
use crate::polynomial::Polynomial;
use crate::types::{SchemeConfig, Shard};

/// Split a secret into `total_shares` shards, requiring `threshold` to reconstruct.
pub fn split_secret(secret: i64, cfg: &SchemeConfig) -> Vec<Shard> {
    let degree = cfg.threshold - 1;
    let poly = Polynomial::random_with_secret(secret, degree, cfg.prime);

    (1..=cfg.total_shares as u64)
        .map(|x| Shard::new(x, poly.eval(x as i64, cfg.prime)))
        .collect()
}

/// Reconstruct a secret from any `threshold`-sized subset of shards.
pub fn reconstruct(shards: &[Shard], cfg: &SchemeConfig) -> Result<i64, String> {
    if shards.len() < cfg.threshold {
        return Err(format!(
            "not enough shards: need {}, got {}",
            cfg.threshold,
            shards.len()
        ));
    }

    Ok(reconstruct_secret(shards, cfg.prime))
}