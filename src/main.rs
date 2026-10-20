use brute_force::generator::{
    Generator,
    Alphabet,
    Word,
};
use std::time::Instant;

fn main() {
    println!("Inline iterator");
    let target = "gumbo";
    let alphabet = Alphabet::Standard;
    let generator1 = Generator::new(alphabet, target.len());
    println!("Searching...");
    let start = Instant::now();
    for i in generator1 {
        if i.as_str() == target {
            println!("Found!");
            println!("Elapsed time: {:?}", start.elapsed());
            break;
        }
    }

    println!("\nBatch iterator");
    let alphabet = Alphabet::Standard;
    let mut generator2 = Generator::new(alphabet, target.len());
    let mut batch = [Word::new(0,5); 62];
    let mut found = false;
    println!("Searching...");
    let start = Instant::now();
    while !found {
        generator2.next_batch(&mut batch);
        for i in 0..batch.len() {
            if batch[i].as_str() == target {
                println!("Found!");
                println!("Elapsed time: {:?}", start.elapsed());
                return;
            }
        }
    }
    panic!();
}
