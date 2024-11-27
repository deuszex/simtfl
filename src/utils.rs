use blake2::{Blake2b512, Digest};

pub fn hash(dat: &[u8]) -> [u8;32]{
    let mut hasher = Blake2b512::new();
    hasher.update(dat);
    let res = hasher.finalize();
    res[0..32].try_into().expect("incorrect length")
}