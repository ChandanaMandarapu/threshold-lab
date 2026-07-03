use crate::frost::nonce::NonceCommitment;

/// Round 1 output: every signer broadcasts their nonce commitments to
/// the coordinator (or to each other, in a fully P2P setup).
#[derive(Debug, Clone)]
pub struct Round1Output {
    pub commitments: Vec<NonceCommitment>,
}

impl Round1Output {
    pub fn new() -> Self {
        Round1Output { commitments: Vec::new() }
    }

    pub fn collect(commitments: Vec<NonceCommitment>) -> Self {
        Round1Output { commitments }
    }

    pub fn participant_ids(&self) -> Vec<u64> {
        self.commitments.iter().map(|c| c.signer_id).collect()
    }
}