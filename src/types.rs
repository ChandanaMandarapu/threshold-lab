/// A single Shamir share: (x, y) coordinate on the secret polynomial.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shard {
    pub x: u64,
    pub y: i64,
}

impl Shard {
    pub fn new(x: u64, y: i64) -> Self {
        Shard { x, y }
    }
}

/// Configuration for a secret sharing scheme.
#[derive(Debug, Clone)]
pub struct SchemeConfig {
    pub threshold: usize,
    pub total_shares: usize,
    pub prime: i64,
}

impl SchemeConfig {
    pub fn new(threshold: usize, total_shares: usize, prime: i64) -> Self {
        assert!(threshold <= total_shares, "threshold cannot exceed total shares");
        assert!(threshold >= 1, "threshold must be at least 1");
        SchemeConfig { threshold, total_shares, prime }
    }
}