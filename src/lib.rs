const MAX_LENGTH: usize = 10;

pub mod generator {
    use std::str;    
    use crate::MAX_LENGTH;

    pub enum Alphabet {
        Standard,
        Full,
        LowerCaseLetters,
        UpperCaseLetters,
        Numbers,
    }

    impl Alphabet {
        pub fn get_alphabet(&self) -> &'static [u8] {
            static ALPHABET_STANDARD: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            static ALPHABET_FULL: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;':\",./<>?";
            static ALPHABET_LOWER_CASE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
            static ALPHABET_UPPER_CASE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            static ALPHABET_NUMBERS: &[u8] = b"0123456789";

            match self {
                Alphabet::Standard => ALPHABET_STANDARD,
                Alphabet::Full => ALPHABET_FULL,
                Alphabet::LowerCaseLetters => ALPHABET_LOWER_CASE_LETTERS,
                Alphabet::UpperCaseLetters => ALPHABET_UPPER_CASE_LETTERS,
                Alphabet::Numbers => ALPHABET_NUMBERS,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
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

    pub struct Generator {
        length: usize,
        alphabet: &'static [u8],
        odometer: [u8; MAX_LENGTH],
        word: Word,
        initiated: bool,
        consumed: bool,
    }

    impl Generator {
        pub fn new(alphabet: Alphabet, length: usize) -> Self {
            let alphabet = alphabet.get_alphabet();
            let word = Word::new(alphabet[0], length);
            let odometer = [0; MAX_LENGTH];
            Generator { 
                length, 
                alphabet, 
                odometer, 
                word, 
                initiated: false, 
                consumed: false 
            }
        }

        pub fn next_batch(&mut self, buffer: &mut [Word]) {
            let len = buffer.len();
            for i in 0..len {
                self.word.set(0, self.alphabet[i]);
                buffer[i] = self.word;
                self.odometer[0] += 1;
            }
            self.carry();
        }

        #[inline(always)]
        fn carry(&mut self) {
            let mut idx: usize = 0;
            let alpha_len = self.alphabet.len();
            while idx < self.length - 1 && self.odometer[idx] as usize >= alpha_len {
                self.odometer[idx] = 0;
                self.word.set(idx, self.alphabet[0usize]);
                idx += 1;
                self.odometer[idx] += 1;  
            }
            if idx >= self.length {
                self.consumed = true;
                return;
            } 
            self.word.set(idx, self.alphabet[self.odometer[idx] as usize]);
        }
    }

    impl Iterator for Generator {
        type Item = Word;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.initiated {
                self.initiated = true;
                return Some(self.word);
            }

            if self.consumed {
                return None;
            } else {
                if self.odometer[0] as usize + 1 < self.alphabet.len() {
                    self.odometer[0] += 1;
                    self.word.set(0, self.alphabet[self.odometer[0] as usize]);
                    return Some(self.word);
                }

                let mut idx: usize = 0;
                self.odometer[idx] += 1;
                while idx < self.length - 1 && self.odometer[idx] as usize >= self.alphabet.len() {
                    self.odometer[idx] = 0;
                    self.word.set(idx, self.alphabet[self.odometer[idx] as usize]);
                    idx += 1;
                    self.odometer[idx] += 1;  
                }
                if idx >= self.length {
                    self.consumed = true;
                    return None;
                } 
                self.word.set(idx, self.alphabet[self.odometer[idx] as usize]);
                Some(self.word)
            }
            
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
    }
}

pub mod verifier {
    use sha2::{Digest, Sha256};

    pub trait Verifier {
        fn verify(&self, word: &[u8]) -> bool;  
    }

    pub struct SHA256 {
        target: [u8; 32],
    }

    impl Verifier for SHA256 {
        fn verify(&self, word: &[u8]) -> bool {
            let encoded: [u8; 32] = Sha256::digest(word).into();
            println!("{:?}", encoded);
            for i in 0..32 {
                if encoded[i] != self.target[i] {
                    return false;
                }
            }
            return true;
        }
    }


    #[cfg(test)]
    mod test {
        use super::*;
        use hex_literal::hex;

        #[test]
        fn test_sha256_verifier() {
            let target = hex!("b0b5af04396d82cafbd0966d4602afc796c685ea8e7d2fcbfb91b473b2b7b7e5");
            let encoded: [u8; 32] = target.into();
            println!("{:?}", encoded);
            let sha = SHA256 {
                target: encoded,
            };

            let candidate = "gumbo".as_bytes();
            assert!(sha.verify(&candidate[..candidate.len()]));
        }
    }
}



