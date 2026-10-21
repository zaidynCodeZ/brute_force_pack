use brute_force::*;
use crate::engine::worker::verifier;
use hex_literal::hex;

use std::time::Instant;

fn main() {
    let target = b"gumbo";
    let hash = verifier::Plaintext{ target };

    let mut engine = engine::Engine::build(b"gumbo", hash, Standard, 6);

    let start = Instant::now();
    let r = engine.run();
    if let Some(result) = r {
        println!("{} found in {:?}", result, start.elapsed());
    }
}