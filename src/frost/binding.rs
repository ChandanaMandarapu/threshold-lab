/// Computes the "binding factor" that ties each signer's nonces to the
/// specific message and set of participants — prevents Wagner's attack
/// on naive multi-signature schemes.
use crate::frost::nonce::NonceCommitment;

pub fn compute_binding_factor(
    signer_id: u64,
    message_hash: i64,
    all_commitments: &[NonceCommitment],
    prime: i64,
) -> i64 {
    // Toy hash: fold all commitment data + message + signer_id into one value.
    let mut acc: i64 = message_hash;
    for c in all_commitments {
        acc = acc
            .wrapping_add(c.signer_id as i64)
            .wrapping_add((c.big_d % prime as i128) as i64)
            .wrapping_add((c.big_e % prime as i128) as i64);
    }
    acc = acc.wrapping_add(signer_id as i64);
    acc.rem_euclid(prime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_factor_differs_per_signer() {
        let commitments = vec![
            NonceCommitment { signer_id: 1, big_d: 10, big_e: 20 },
            NonceCommitment { signer_id: 2, big_d: 30, big_e: 40 },
        ];
        let b1 = compute_binding_factor(1, 999, &commitments, 1_000_003);
        let b2 = compute_binding_factor(2, 999, &commitments, 1_000_003);
        assert_ne!(b1, b2);
    }
}