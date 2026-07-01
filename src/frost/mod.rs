/// FROST (Flexible Round-Optimized Schnorr Threshold signatures) style
/// two-round signing protocol. Round 1: each signer commits to nonces.
/// Round 2: each signer produces a partial signature using their share
/// and the aggregated commitments. Nobody ever reconstructs the private key.
pub mod nonce;
pub mod round1;
pub mod round2;
pub mod aggregate;

pub use nonce::NonceCommitment;
pub use round1::Round1Output;
pub use round2::Round2Output;