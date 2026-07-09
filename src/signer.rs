use crate::types::Shard;

#[derive(Debug, Clone)]
pub struct PartialSignature {
    pub signer_id: u64,
    pub value: i64,
}

/// Each participant produces a partial signature from their shard.
pub fn partial_sign(shard: &Shard, message_hash: i64, prime: i64) -> PartialSignature {
    let value = (shard.y * message_hash).rem_euclid(prime);

    PartialSignature {
        signer_id: shard.x,
        value,
    }
}