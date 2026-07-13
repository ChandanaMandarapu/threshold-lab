/// Simulates a Distributed Key Generation (DKG) ceremony: N participants
/// each generate a random polynomial, share it with everyone, and combine
/// received shares into a final private key share — no single party ever
/// learns the full private key.
use crate::polynomial::Polynomial;
use crate::types::Shard;
use crate::commitment::FeldmanCommitment;

#[derive(Debug, Clone)]
pub struct Participant {
    pub id: u64,
    pub poly: Polynomial,
    pub commitment: FeldmanCommitment,
}

#[derive(Debug, Clone)]
pub struct DkgResult {
    pub participant_id: u64,
    pub key_share: i64,
    pub public_commitments: Vec<FeldmanCommitment>,
}

pub fn run_dkg_round(
    participants: &[Participant],
    prime: i64,
) -> Vec<DkgResult> {
    let n = participants.len();
    let mut results = Vec::with_capacity(n);

    for receiver in participants {
        let mut key_share: i64 = 0;
        for sender in participants {
            let contribution = sender.poly.eval(receiver.id as i64, prime);
            key_share = (key_share + contribution).rem_euclid(prime);
        }

        results.push(DkgResult {
            participant_id: receiver.id,
            key_share,
            public_commitments: participants.iter().map(|p| p.commitment.clone()).collect(),
        });
    }

    results
}

pub fn derive_group_public_key(participants: &[Participant], generator: i128, modulus: i128) -> i128 {
    participants
        .iter()
        .map(|p| p.commitment.commitments[0]) // g^secret_i, index 0 = constant term
        .fold(1i128, |acc, c| (acc * c) % modulus)
}