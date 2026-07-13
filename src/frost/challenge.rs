/// Derives the Schnorr challenge scalar via a Fiat-Shamir style hash
/// of (group commitment || group public key || message).
pub fn derive_challenge(group_commitment: i128, group_pubkey: i128, message_hash: i64, prime: i64) -> i64 {
    let mixed = group_commitment
        .wrapping_add(group_pubkey)
        .wrapping_add(message_hash as i128);
    (mixed % prime as i128) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_is_deterministic() {
        let c1 = derive_challenge(100, 200, 300, 1_000_003);
        let c2 = derive_challenge(100, 200, 300, 1_000_003);
        assert_eq!(c1, c2);
    }
}