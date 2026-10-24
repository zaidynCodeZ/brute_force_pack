use crate::Alphabet;
pub static MAX_LENGTH: usize = 8;

pub struct Generator<A: Alphabet<N>, const N: usize> {
    length: usize,
    _alphabet: A,
    odometer: [u8; MAX_LENGTH - 1],
}

impl<A: Alphabet<N>, const N: usize> Generator<A, N> {
    pub fn print_candidate(candidate: u64) {
        let mut c = "".to_string();
        let mut candidate = candidate;
        let mask: u8 = 255;
        while candidate > 0 {
            c.push((candidate as u8 & mask) as char);
            candidate >>= 8;
        }
        c = c.chars().rev().collect();
        println!("{}", c);
    }

    pub fn with_seed(alphabet: A, length: usize, seed: usize) -> Self {
        let mut odometer = [0; MAX_LENGTH - 1];
        let mut seed = seed;
        let mut index = 0;

        while index <= length && seed > 0 {
            let digit = seed % N;
            seed /= N;

            odometer[index] = digit as u8;
            index += 1;
        }

        Generator { 
            length, 
            _alphabet: alphabet,
            odometer, 
        }
    }

    pub fn next_batch(&mut self, buffer: &mut [u64; N]) {
        let mut packed: u64 = 0;
        for i in 0..self.length - 1 {
            packed |= (A::ALPHABET[self.odometer[i] as usize] as u64) << (i + 1) * 8;
        }

        for i in 0..N {
            buffer[i] = packed | A::ALPHABET[i] as u64;
            // Self::print_candidate(buffer[i]);
        }
        self.odometer[0] += 1;
        self.carry();
    }

    #[inline(always)]
    fn carry(&mut self) {
        if (self.odometer[0] as usize) < N {
            return;
        }

        let mut idx: usize = 0;
        while idx < MAX_LENGTH - 2 && self.odometer[idx] as usize >= N {
            self.odometer[idx] = 0;
            idx += 1;
            self.odometer[idx] += 1;  
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
