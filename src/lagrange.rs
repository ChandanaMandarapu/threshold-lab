use crate::types::Shard;

/// Modular multiplicative inverse via extended Euclidean algorithm.
fn mod_inverse(a: i64, prime: i64) -> i64 {
    let (mut old_r, mut r) = (a, prime);
    let (mut old_s, mut s) = (1i64, 0i64);

    while r != 0 {
        let q = old_r / r;

        let tmp_r = old_r - q * r;
        old_r = r;
        r = tmp_r;

        let tmp_s = old_s - q * s;
        old_s = s;
        s = tmp_s;
    }

    old_s.rem_euclid(prime)
}

/// Reconstruct the secret (f(0)) from a set of shards using Lagrange interpolation.
pub fn reconstruct_secret(shards: &[Shard], prime: i64) -> i64 {
    let mut secret: i64 = 0;

    for (i, si) in shards.iter().enumerate() {
        let mut numerator: i64 = 1;
        let mut denominator: i64 = 1;

        for (j, sj) in shards.iter().enumerate() {
            if i == j {
                continue;
            }

            numerator = (numerator * (-(sj.x as i64)).rem_euclid(prime)).rem_euclid(prime);
            denominator =
                (denominator * (si.x as i64 - sj.x as i64)).rem_euclid(prime);
        }

        let term = (si.y * numerator).rem_euclid(prime)
            * mod_inverse(denominator, prime);

        secret = (secret + term).rem_euclid(prime);
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polynomial::Polynomial;

    #[test]
    fn reconstructs_known_secret() {
        let prime = 1_000_003;
        let poly = Polynomial::new(vec![99, 5, 2]); // secret = 99

        let shards: Vec<Shard> = (1..=3)
            .map(|x| Shard::new(x, poly.eval(x as i64, prime)))
            .collect();

        let secret = reconstruct_secret(&shards, prime);
        assert_eq!(secret, 99);
    }
}