use brute_force::generator::{
    Generator,
    Alphabet,
};
use std::time::Instant;

fn main() {
    let target = "gumbo";
    let alphabet = Alphabet::Standard;
    let generator = Generator::new(alphabet, target.len());
    println!("Searching...");
    let start = Instant::now();
    for i in generator {
        if i.as_str() == target {
            println!("Found!");
            println!("Elapsed time: {:?}", start.elapsed());
            return;
        }
    }
    panic!();
}
