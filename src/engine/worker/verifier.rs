use sha2::{Digest, Sha256};

pub trait Verifier {
    fn verify(&self, word: &[u8]) -> bool;  
}

pub struct SHA256 {
    pub target: [u8; 32],
}
impl Verifier for SHA256 {
    fn verify(&self, word: &[u8]) -> bool {
        let encoded: [u8; 32] = Sha256::digest(word).into();
        for i in 0..32 {
            if encoded[i] != self.target[i] {
                return false;
            }
        }
        return true;
    }
}

pub struct Plaintext<'a> {
    pub target: &'a [u8],
}
impl<'a> Verifier for Plaintext<'a> {
    fn verify(&self, word: &[u8]) -> bool {
        self.target == word
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