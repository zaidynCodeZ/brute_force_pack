pub mod generator;
pub mod verifier;

use generator::Generator;
use verifier::Verifier;
use crate::Alphabet;

pub struct Worker <'a, A: Alphabet<N>, const N: usize, T: Verifier> {
    verifier: &'a T,
    generator: Generator<A, N>,
    batches: usize,
}

impl<'a, A: Alphabet<N>, const N: usize, T: Verifier> Worker<'a, A, N, T> {
    pub fn new(alphabet: A, verifier: &'a T, length: usize, seed: usize, batches: usize) -> Self {
        let generator = Generator::with_seed(alphabet, length, seed);
        Worker {
            verifier,
            generator,
            batches,
        }
    }

    pub fn run(&mut self) -> Option<u64>{
        let mut batch = [0u64; N];
        for _ in 0..self.batches {
            self.generator.next_batch(&mut batch);
            for i in batch.iter() {
                if self.verifier.verify(*i) {
                    return Some(*i);
                }
            }
        }
        return None;
    }
}