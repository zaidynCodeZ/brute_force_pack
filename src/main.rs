use brute_force::*;
use crate::engine::worker::verifier;
//use hex_literal::hex;

use std::time::Instant;

fn main() {
    let target = "zumbos";
    let hash = verifier::Plaintext::new(target);

    let mut engine = engine::Engine::build(hash, LowerCaseLetters, 6);

    let start = Instant::now();
    let r = engine.run();
    if let Some(result) = r {
        println!("{} found in {:?}", result, start.elapsed());
    } else {
        println!("Target not found");
    }
}