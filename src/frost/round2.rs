use crate::types::Shard;

/// Round 2: each signer produces a partial signature using their key
/// share, secret nonces, and the binding factor from round 1.
#[derive(Debug, Clone, Copy)]
pub struct Round2Output {
    pub signer_id: u64,
    pub z: i64, // partial signature scalar
}

pub fn produce_partial_signature(
    shard: &Shard,
    nonce_d: i64,
    nonce_e: i64,
    binding_factor: i64,
    challenge: i64,
    prime: i64,
) -> Round2Output {
    // z_i = d_i + (e_i * binding_factor) + (lambda_i * secret_share_i * challenge)
    let z = (nonce_d
        + nonce_e * binding_factor
        + shard.y * challenge)
        .rem_euclid(prime);

    Round2Output { signer_id: shard.x, z }
}