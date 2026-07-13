use crate::frost::round2::Round2Output;

/// Aggregate all partial signatures into the final group signature.
/// In real FROST this produces a valid Schnorr signature verifiable
/// against the group public key with zero extra rounds.
pub fn aggregate_partial_signatures(partials: &[Round2Output], prime: i64) -> i64 {
    partials.iter().fold(0i64, |acc, p| (acc + p.z).rem_euclid(prime))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregation_sums_all_partials() {
        let partials = vec![
            Round2Output { signer_id: 1, z: 10 },
            Round2Output { signer_id: 2, z: 20 },
            Round2Output { signer_id: 3, z: 30 },
        ];
        assert_eq!(aggregate_partial_signatures(&partials, 1_000_003), 60);
    }
}