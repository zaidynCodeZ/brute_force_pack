pub mod worker;
use crate::Alphabet;
use worker::{
    Worker,
    verifier::Verifier,
    generator,
};
use std::marker::{
    Send,
    Sync,
};
use rayon::prelude::*;

pub struct Engine<A: Alphabet<N>, const N: usize , V: Verifier> {
    batches: usize,
    hash: V,
    alphabet: A,
    length: usize,
}

impl<A: Alphabet<N> + Send + Sync, const N: usize, V: Verifier + Send + Sync> Engine<A, N, V> {
    pub fn build(hash: V, alphabet: A, length: usize) -> Self {
        if length > generator::MAX_LENGTH {
            panic!("Length longer than max allowed generator length");
        }
        let exponent: u32 = length as u32 - 1;
        let batches = (N).checked_pow(exponent).expect("WAYYY too many possible options");
        Engine {
            batches,
            hash,
            alphabet,
            length,
        }
    }

    pub fn run(&mut self) -> Option<u64> {
        let thread_count = rayon::current_num_threads();
        let chunk_size = self.batches / thread_count;
        let r = self.batches % chunk_size;
        let result = (0..thread_count)
            .into_par_iter()
            .find_map_any(|thread_idx| {
                let start_batch = thread_idx * chunk_size;
                let mut worker = Worker::new(self.alphabet, &self.hash, self.length, start_batch, chunk_size + r);
                worker.run()
            });
        result
    }
}