use std::str;    
use crate::Alphabet;
pub static MAX_LENGTH: usize = 10;

#[derive(Clone, Copy)]
pub struct Word {
    word: [u8; MAX_LENGTH],
    length: usize,
}

impl Word {
    pub fn new(start: u8, length: usize) -> Self {
        let word = [start; MAX_LENGTH];
        Word{ word, length }
    }

    #[inline(always)]
    pub fn as_str<'a>(&'a self) -> &'a str {
        str::from_utf8(&self.word[..self.length]).expect("You done fucked up")
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.word[..self.length]
    }
    
    #[inline(always)]
    pub fn set(&mut self, index: usize, value: u8) {
        self.word[index] = value;
    }
}

pub struct Generator<A: Alphabet<N>, const N: usize> {
    length: usize,
    alphabet: A,
    odometer: [u8; MAX_LENGTH],
    word: Word,
}

impl<A: Alphabet<N>, const N: usize> Generator<A, N> {
    pub fn new(alphabet: A, length: usize) -> Self {
        let word = Word::new(A::ALPHABET[0], length);
        let odometer = [0; MAX_LENGTH];
        Generator { 
            length, 
            alphabet, 
            odometer, 
            word, 
        }
    }

    pub fn with_seed(alphabet: A, length: usize, seed: usize) -> Self {
        let mut word = Word::new(A::ALPHABET[0], length);
        let mut odometer = [0; MAX_LENGTH];

        let mut seed = seed;
        let mut index = 1;

        while index < length && seed > 0 {
            let digit = seed % N;
            seed /= N;

            odometer[index] = digit as u8;
            word.set(index, A::ALPHABET[digit]);
            index += 1;
        }

        Generator { 
            length, 
            alphabet, 
            odometer, 
            word, 
        }
    }

    pub fn next_batch(&mut self, buffer: &mut [Word; N]) {
        for i in 0..N {
            self.word.set(0, A::ALPHABET[i]);
            buffer[i] = self.word;
        }
        self.odometer[0] += N as u8;
        self.carry();
    }

    #[inline(always)]
    fn carry(&mut self) {
        let mut idx: usize = 0;
        while idx < self.length - 1 && self.odometer[idx] as usize >= N {
            self.odometer[idx] = 0;
            self.word.set(idx, A::ALPHABET[0]);
            idx += 1;
            self.odometer[idx] += 1;  
        }
        if idx >= self.length {
            return;
        } 
        self.word.set(idx, A::ALPHABET[self.odometer[idx] as usize]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_batch_gen() {
        let mut batch = [Word::new(0,2); 26];
        let target1 = ["aa", "ba", "ca", "da", "ea", "fa", "ga", "ha", "ia", "ja", "ka", "la", "ma", "na", "oa", "pa", "qa", "ra", "sa", "ta", "ua", "va", "wa", "xa", "ya", "za"];
        let target2 = ["ab", "bb", "cb", "db", "eb", "fb", "gb", "hb", "ib", "jb", "kb", "lb", "mb", "nb", "ob", "pb", "qb", "rb", "sb", "tb", "ub", "vb", "wb", "xb", "yb", "zb"];
        let mut generator = Generator::new(Alphabet::LowerCaseLetters, 2);

        generator.next_batch(&mut batch);
        for i in 0..26 {
            assert_eq!(batch[i].as_str(), target1[i]);
        }

        generator.next_batch(&mut batch);
        for i in 0..26 {
            assert_eq!(batch[i].as_str(), target2[i]);
        }
    }

    #[test]
    fn test_seeded_batch_gen() {
        let mut batch = [Word::new(0,2); 26];
        let target = ["ab", "bb", "cb", "db", "eb", "fb", "gb", "hb", "ib", "jb", "kb", "lb", "mb", "nb", "ob", "pb", "qb", "rb", "sb", "tb", "ub", "vb", "wb", "xb", "yb", "zb"];
        let mut generator = Generator::new(Alphabet::LowerCaseLetters, 2);
        generator.seed(1);

        generator.next_batch(&mut batch);
        for i in 0..26 {
            assert_eq!(batch[i].as_str(), target[i]);
        }
    }
}
