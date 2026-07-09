use threshold_lab::types::SchemeConfig;
use threshold_lab::shamir::{split_secret, reconstruct};

#[test]
fn split_and_reconstruct_roundtrip() {
    let cfg = SchemeConfig::new(3, 5, 1_000_003);
    let secret = 424242;

    let shards = split_secret(secret, &cfg);
    let subset = &shards[1..4]; // any 3 of 5

    let recovered = reconstruct(subset, &cfg).expect("reconstruction failed");
    assert_eq!(recovered, secret);
}