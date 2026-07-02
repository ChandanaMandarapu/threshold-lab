use rand::Rng;

/// Each signer generates two secret nonces (d, e) per FROST spec, and
/// publishes commitments to them before signing begins.
#[derive(Debug, Clone, Copy)]
pub struct SecretNonce {
    pub d: i64,
    pub e: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct NonceCommitment {
    pub signer_id: u64,
    pub big_d: i128, // g^d
    pub big_e: i128, // g^e
}

impl SecretNonce {
    pub fn generate(prime: i64) -> Self {
        let mut rng = rand::thread_rng();
        SecretNonce {
            d: rng.gen_range(1..prime),
            e: rng.gen_range(1..prime),
        }
    }

    pub fn commit(&self, signer_id: u64, generator: i128, modulus: i128) -> NonceCommitment {
        NonceCommitment {
            signer_id,
            big_d: mod_pow(generator, self.d as u64, modulus),
            big_e: mod_pow(generator, self.e as u64, modulus),
        }
    }
}

fn mod_pow(mut base: i128, mut exp: u64, modulus: i128) -> i128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 { result = (result * base) % modulus; }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}