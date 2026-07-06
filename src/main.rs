mod types;

use types::{Shard, SchemeConfig};

fn main() {
    let cfg = SchemeConfig::new(3, 5, 208351617316091241234123458251);
    println!("threshold-lab booting with config: {:?}", cfg);

    let s = Shard::new(1, 42);
    println!("sample shard: {:?}", s);
}