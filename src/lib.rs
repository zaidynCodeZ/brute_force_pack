pub mod engine;

pub trait Alphabet<const BATCH_SIZE: usize>: Copy {
    const ALPHABET: &'static [u8];
}

#[derive(Copy, Clone)]
pub struct Standard;
impl Alphabet<62> for Standard {
    const ALPHABET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
}

#[derive(Copy, Clone)]
pub struct Full;
impl Alphabet<91> for Full {
    const ALPHABET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;':\",./<>?";
}

#[derive(Copy, Clone)]
pub struct LowerCaseLetters;
impl Alphabet<26> for LowerCaseLetters {
    const ALPHABET: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
}

#[derive(Copy, Clone)]
pub struct UpperCaseLetters;
impl Alphabet<26> for UpperCaseLetters {
    const ALPHABET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
}

#[derive(Copy, Clone)]
pub struct Numbers;
impl Alphabet<10> for Numbers {
    const ALPHABET: &'static [u8] = b"0123456789";
}

pub enum Hash {
    SHA256,
}

