mod types;
mod polynomial;
mod lagrange;
mod shamir;

use types::SchemeConfig;
use shamir::{split_secret, reconstruct};

fn main() {
    let cfg = SchemeConfig::new(3, 5, 208351617316091241234123458251);
    let secret = 123456789;

    let shards = split_secret(secret, &cfg);
    println!("generated shards: {:?}", shards);

    let subset = &shards[0..3];
    match reconstruct(subset, &cfg) {
        Ok(s) => println!("reconstructed secret: {}", s),
        Err(e) => println!("error: {}", e),
    }
}