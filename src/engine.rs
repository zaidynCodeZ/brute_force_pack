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

pub struct Engine<'a, A: Alphabet<N>, const N: usize , V: Verifier> {
    batches: usize,
    target: &'a [u8],
    hash: V,
    alphabet: A,
    length: usize,
}

impl<'a, A: Alphabet<N> + Send + Sync, const N: usize, V: Verifier + Send + Sync> Engine<'a, A, N, V> {
    pub fn build(target: &'a [u8], hash: V, alphabet: A, length: usize) -> Self {
        let exponent: u32 = generator::MAX_LENGTH as u32 - 1;
        let batches = (N).checked_pow(exponent).expect("WAYYY too many possible options");
        Engine {
            batches,
            target,
            hash,
            alphabet,
            length,
        }
    }

    pub fn run(&mut self) -> Option<String> {
        let thread_count = rayon::current_num_threads();
        let chunk_size = self.batches / thread_count;
        let r = self.batches % chunk_size;
        let result = (0..thread_count)
            .into_par_iter()
            .find_map_any(|thread_idx| {
                let start_batch = thread_idx * chunk_size;
                let mut worker = Worker::new(self.alphabet, &self.hash, self.length, start_batch, chunk_size);
                worker.run()
            });
        return result;
    }
}