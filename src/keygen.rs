use rand::Rng;

/// Simulate generating a random "master secret" as if it were a private key.
pub fn generate_master_secret(prime: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..prime)
}

/// Simulate deriving a public identifier from a secret (toy hash, not crypto-secure).
pub fn derive_public_id(secret: i64) -> String {
    format!("pub_{:x}", secret.unsigned_abs())
}