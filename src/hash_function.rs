use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn hash_leaf(value: &[u8]) -> [u8; 32] {
    let mut sha = Sha256::new();
    let mut result = [0; 32];
    sha.input(&value);
    sha.result(&mut result);
    result
}

pub fn hash_node(left: &[u8], right: &[u8]) -> [u8; 32] {
    let mut sha = Sha256::new();
    let mut result = [0; 32];
    sha.input(&left);
    sha.input(&right);
    sha.result(&mut result);
    result
}
