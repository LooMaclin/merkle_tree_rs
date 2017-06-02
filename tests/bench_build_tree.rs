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

    /*
    ==============================================================
    ==============================================================
                            JSON BENCHMARK
    ==============================================================
    ==============================================================
    */

    #[bench]
    fn build_1_tree_parallel_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_10_tree_parallel_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100_tree_parallel_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000_tree_parallel_json(b: &mut Bencher) {
        b.iter(|| {
                   let n = self::test::black_box(1000);
                   let mut merkle_tree: MerkleTree = MerkleTree::default();
                   (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
                   merkle_tree.build()
               })
    }


    #[bench]
    fn build_10000_tree_parallel_json(b: &mut Bencher) {
        b.iter(|| {
                   let n = self::test::black_box(10000);
                   let mut merkle_tree: MerkleTree = MerkleTree::default();
                   (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
                   merkle_tree.build()
               })
    }

    #[bench]
    fn build_100000_tree_parallel_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000000_tree_parallel_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1_tree_sequence_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_10_tree_sequence_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100_tree_sequence_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000_tree_sequence_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }


    #[bench]
    fn build_10000_tree_sequence_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100000_tree_sequence_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000000_tree_sequence_json(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }


    /*
    ==============================================================
    ==============================================================
                            MSGPACK BENCHMARK
    ==============================================================
    ==============================================================
    */

    #[bench]
    fn build_1_tree_parallel_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_10_tree_parallel_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100_tree_parallel_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000_tree_parallel_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }


    #[bench]
    fn build_10000_tree_parallel_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100000_tree_parallel_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000000_tree_parallel_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1_tree_sequence_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_10_tree_sequence_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100_tree_sequence_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000_tree_sequence_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }


    #[bench]
    fn build_10000_tree_sequence_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100000_tree_sequence_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000000_tree_sequence_msgpack(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::MsgPack;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    /*
    ==============================================================
    ==============================================================
                            BINCODE BENCHMARK
    ==============================================================
    ==============================================================
    */

    #[bench]
    fn build_1_tree_parallel_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_10_tree_parallel_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100_tree_parallel_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000_tree_parallel_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }


    #[bench]
    fn build_10000_tree_parallel_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100000_tree_parallel_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000000_tree_parallel_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1_tree_sequence_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_10_tree_sequence_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100_tree_sequence_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000_tree_sequence_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }


    #[bench]
    fn build_10000_tree_sequence_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(10000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_100000_tree_sequence_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(100000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn build_1000000_tree_sequence_bincode(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(1000000);
            let mut merkle_tree: MerkleTree = MerkleTree::default();
            merkle_tree.format = SerializationFormat::Bincode;
            merkle_tree.parallel = false;
            (0..n).fold((), |_, _| { merkle_tree.push(&["a"]); });
            merkle_tree.build()
        })
    }

    #[bench]
    fn raw_hash_2000000_sequential(b: &mut Bencher) {
        b.iter(|| {
            let n = self::test::black_box(2000000);
            (0..n).fold((), |a, b| {
                let mut result = [0; 32];
                let mut sha = Sha256::new();
                sha.input(&[0; 32]);
                sha.result(&mut result);
                sha.reset()
            })
        })
    }
}
