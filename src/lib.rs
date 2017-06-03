#![deny(warnings)]
/// ! Merkle Tree
/// Библиотека для создания и работы со структурой данных Merkle tree.
/// Позволяет создавать структуру данных основываясь на информации о транзакциях и добавлять новые
/// листы в деревья после построения.
/// Так же позволяет получать Audit Path для выбранного хэша транзакций.
#[macro_use]
extern crate log;
extern crate crypto;
extern crate bincode;
extern crate serde;
extern crate hex_slice;
extern crate serde_json;
extern crate rmp_serde;
extern crate rayon;

mod merkle_tree;
mod serialization_format;
// mod merkle_tree_string;
mod hash_function;
// mod hash_function_string;

pub use serialization_format::SerializationFormat;
pub use merkle_tree::MerkleTree;
// pub use merkle_tree_string::MerkleTreeString;
