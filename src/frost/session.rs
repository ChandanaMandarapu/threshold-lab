use crate::frost::{Round1Output, Round2Output};
use crate::frost::nonce::{SecretNonce, NonceCommitment};
use crate::types::Shard;
use std::collections::HashMap;

/// Coordinates a full signing session across N participants: tracks
/// which round we're in, collects commitments, and drives the protocol
/// forward without any single party controlling the whole flow.
pub struct SigningSession {
    pub participants: Vec<u64>,
    pub threshold: usize,
    pub round1: Option<Round1Output>,
    pub round2_outputs: Vec<Round2Output>,
    pub secret_nonces: HashMap<u64, SecretNonce>,
}

impl SigningSession {
    pub fn new(participants: Vec<u64>, threshold: usize) -> Self {
        SigningSession {
            participants,
            threshold,
            round1: None,
            round2_outputs: Vec::new(),
            secret_nonces: HashMap::new(),
        }
    }

    pub fn is_ready_to_sign(&self) -> bool {
        self.round2_outputs.len() >= self.threshold
    }

    pub fn register_nonce(&mut self, signer_id: u64, nonce: SecretNonce) {
        self.secret_nonces.insert(signer_id, nonce);
    }

    pub fn submit_round2(&mut self, output: Round2Output) {
        self.round2_outputs.push(output);
    }
}