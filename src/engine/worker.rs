pub mod generator;
pub mod verifier;

use generator::Generator;
use verifier::Verifier;
use crate::Alphabet;

pub struct Worker <'a, A: Alphabet<N>, const N: usize, T: Verifier> {
    verifier: &'a T,
    generator: Generator<A, N>,
    batches: usize,
    length: usize,
}

impl<'a, A: Alphabet<N>, const N: usize, T: Verifier> Worker<'a, A, N, T> {
    pub fn new(alphabet: A, verifier: &'a T, length: usize, seed: usize, batches: usize) -> Self {
        let generator = Generator::with_seed(alphabet, length, seed);
        Worker {
            verifier,
            generator,
            batches,
            length,
        }
    }

    pub fn run(&mut self) -> Option<String>{
        let mut batch = [generator::Word::new(0, self.length); N];
        for _ in 0..self.batches {
            self.generator.next_batch(&mut batch);
            for i in batch.iter() {
                if self.verifier.verify(&i.as_bytes()[..self.length]) {
                    let output = i.as_str().to_string();
                    return Some(output);
                }
            }
        }
        return None;
    }
}