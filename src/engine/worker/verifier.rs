// use sha2::{Digest, Sha256};

pub trait Verifier {
    fn verify(&self, word: u64) -> bool;  
}

// pub struct SHA256 {
//     pub target: [u8; 32],
// }
// impl Verifier for SHA256 {
//     fn verify(&self, word: u64) -> bool {
//         let encoded: [u8; 32] = Sha256::digest(word).into();
//         for i in 0..32 {
//             if encoded[i] != self.target[i] {
//                 return false;
//             }
//         }
//         return true;
//     }
// }

pub struct Plaintext {
    pub target: u64,
}

impl Plaintext {
    pub fn new(word: &str) -> Self {
        let mut packed = 0u64;
        let mut shift = 0;
        for c in word.chars().rev() {
            packed |= (c as u64) << shift;
            shift += 8;
        }

        Plaintext { target: packed }
    }
}

impl Verifier for Plaintext {
    fn verify(&self, word: u64) -> bool {
        self.target == word
    }
}


#[cfg(test)]
mod test {
    use super::*;
}