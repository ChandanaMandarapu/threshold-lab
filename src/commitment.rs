/// Feldman Verifiable Secret Sharing: commit to polynomial coefficients
/// using toy "exponentiation" commitments (g^coef mod p), so shareholders
/// can verify their share is consistent with the committed polynomial
/// without learning the secret.
use crate::polynomial::Polynomial;

#[derive(Debug, Clone)]
pub struct FeldmanCommitment {
    pub commitments: Vec<i128>, // g^a_i mod p for each coefficient a_i
    pub generator: i128,
    pub modulus: i128,
}

impl FeldmanCommitment {
    pub fn commit(poly: &Polynomial, generator: i128, modulus: i128) -> Self {
        let commitments = poly
            .coefficients
            .iter()
            .map(|&coef| mod_pow(generator, coef.rem_euclid(modulus as i64) as u64, modulus))
            .collect();
        FeldmanCommitment { commitments, generator, modulus }
    }

    /// Verify that a (x, y) share is consistent with the committed polynomial:
    /// g^y =? prod( C_i ^ (x^i) ) mod p
    pub fn verify_share(&self, x: u64, y: i64, modulus: i128) -> bool {
        let lhs = mod_pow(self.generator, y.rem_euclid(modulus as i64) as u64, modulus);

        let mut rhs: i128 = 1;
        for (i, &c_i) in self.commitments.iter().enumerate() {
            let exponent = (x as u64).pow(i as u32);
            rhs = (rhs * mod_pow(c_i, exponent, modulus)) % modulus;
        }

        lhs == rhs
    }
}

fn mod_pow(mut base: i128, mut exp: u64, modulus: i128) -> i128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}