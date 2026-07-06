use std::fmt;

#[derive(Debug)]
pub enum VaultError {
    InsufficientShards { needed: usize, got: usize },
    InvalidShard(String),
    ConfigError(String),
}

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VaultError::InsufficientShards { needed, got } => {
                write!(f, "need {} shards, only have {}", needed, got)
            }
            VaultError::InvalidShard(msg) => write!(f, "invalid shard: {}", msg),
            VaultError::ConfigError(msg) => write!(f, "config error: {}", msg),
        }
    }
}

impl std::error::Error for VaultError {}