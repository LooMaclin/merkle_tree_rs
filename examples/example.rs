extern crate merkle_tree;

use merkle_tree::{MerkleTree, SerializationFormat};

fn main() {
    let mut merkle_tree = MerkleTree::from(&mut ["a", "b", "c"], SerializationFormat::Json);
    merkle_tree.build();
    println!("Merkle tree root hash: {:?}", merkle_tree.get_merkle_root());
    println!("Merkle tree audit proot: {:?}", merkle_tree.audit_proof(&[172, 141, 131, 66, 187, 178, 54, 45, 19, 240, 165, 89, 163, 98, 27, 180, 7, 1, 19, 104, 137, 81, 100, 182, 40, 165, 79, 127, 195, 63, 196, 60]).unwrap());
    merkle_tree.push(&String::from("d"));
    println!("Merkle tree root hash: {:?}", merkle_tree.get_merkle_root());
    println!("Merkle tree audit proot: {:?}", merkle_tree.audit_proof(&[172, 141, 131, 66, 187, 178, 54, 45, 19, 240, 165, 89, 163, 98, 27, 180, 7, 1, 19, 104, 137, 81, 100, 182, 40, 165, 79, 127, 195, 63, 196, 60]).unwrap());
}