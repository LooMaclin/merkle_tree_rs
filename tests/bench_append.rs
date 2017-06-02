#![feature(test)]
extern crate test;
extern crate merkle_tree;

#[cfg(test)]
mod tests {
    use merkle_tree::{MerkleTree, SerializationFormat};
    use test;
    use test::Bencher;

    extern crate crypto;

    use self::crypto::sha2::Sha256;
    use self::crypto::digest::Digest;

    #[bench]
    fn append_10000_bincode_parallel(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = true;
            merkle_tree.push(&["a"]);
            merkle_tree.build();
            (0..n).fold((), |_, index| { merkle_tree.push(&[index]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn append_100000_bincode_parallel(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = true;
            merkle_tree.push(&["a"]);
            merkle_tree.build();
            (0..n).fold((), |_, index| { merkle_tree.push(&[index]); });
            merkle_tree.build()
        })
    }
}