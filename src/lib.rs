pub mod generator {
    use std::str;

    const MAX_LENGTH: usize = 10;

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

        pub fn as_str<'a>(&'a self) -> &'a str {
            str::from_utf8(&self.word[..self.length]).expect("You done fucked up")
        }

        pub fn as_bytes(&self) -> &[u8] {
            &self.word[..self.length]
        }

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
}

