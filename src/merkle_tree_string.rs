use std::default::Default;
use serialization_format::SerializationFormat;
use rayon::prelude::*;
use hash_function_string::{hash_leaf_string, hash_node_string};

#[derive(Debug, Eq, PartialEq)]
pub struct MerkleTreeString {
    /// Хранилище уровней дерева. 0-ой уровень всегда используется для хранения "листьев",
    /// а именно хэшей сериализованных в векторы байт данных транзакций.
    pub layers: Vec<Vec<String>>,
    /// Формат сериализации. Меняется по желанию пользователя.
    pub format: SerializationFormat,
    /// Флаг отвечающий за паралелизацию генерации нового слоя дерева.
    pub parallel: bool,
    /// Флаг указывающий на то - было ли дерево уже построено или нет.
    pub builded: bool,
}

impl Default for MerkleTreeString {
    fn default() -> MerkleTreeString {
        let mut layers = Vec::with_capacity(256);
        layers.push(Vec::with_capacity(512));
        MerkleTreeString {
            layers: layers,
            parallel: true,
            builded: false,
            format: SerializationFormat::MsgPack,
        }
    }
}

impl MerkleTreeString {
    pub fn build(&mut self) {
        match self.layers[0].len() {
            0 => {
                panic!("No leafs in tree!");
            }
            1 => {
                debug!("Tree have one leaf. Merke root hash == hash(leaf[0])");
                let hashed_leaf = hash_leaf_string(&self.layers[0][0]);
                debug!("Layers len: {}", self.layers.len());
                self.layers.push(Vec::with_capacity(1250));
                self.layers[1].push(hashed_leaf);
            }
            _ => {
                debug!("Tree have more one leaf.");
                self.recursive_create_nodes(0);
            }
        }
        self.builded = true;
    }

    pub fn from(leafs: &mut [String], format: SerializationFormat) -> MerkleTreeString {
        let log2_leafs = (leafs.len() as f64).log2();
        println!("log2 leafs: {}", log2_leafs);
        let mut base_layer = Vec::from(leafs);
        //            base_layer = leafs.iter().map(|element| {
        //                hash_leaf_string(element)
        //            }).collect::<Vec<String>>();
        let mut layer_len = base_layer.len();
        base_layer.reserve(layer_len);
        let mut layers: Vec<Vec<String>> = Vec::with_capacity(((log2_leafs).round()) as usize);
        layers.push(Vec::with_capacity(layer_len));
        while layer_len != 1 {
            debug!("Create new layer with capacity: {}", layer_len);
            layers.push(Vec::with_capacity(layer_len));
            layer_len = layer_len / 2;
        }
        layers[0].append(&mut base_layer);
        MerkleTreeString {
            layers: layers,
            parallel: true,
            builded: false,
            format: format,
        }

    }

    fn recursive_create_nodes(&mut self, current_layer_index: usize) {
        debug!("Layer index: {}", current_layer_index);
        if self.layers[current_layer_index].len() > 1 {
            if self.parallel {
                self.generate_new_layer_parallel(current_layer_index);
            } else {
                self.generate_new_layer_sequence(current_layer_index);
            }
            self.recursive_create_nodes(current_layer_index + 1);
        }
    }

    fn generate_new_layer_parallel(&mut self, current_layer_index: usize) {
        let mut new_layer = self.layers[current_layer_index]
            .par_chunks(2)
            .map(|pair| {
                if pair.len() == 2 {
                    hash_node_string(&pair[0], &pair[1])
                } else {
                    hash_leaf_string(&pair[0])
                }
            })
            .collect::<Vec<String>>();
        let current_layer_len = self.layers[current_layer_index].len();
        self.create_new_layer(current_layer_index, current_layer_len, &mut new_layer);
    }

    fn generate_new_layer_sequence(&mut self, current_layer_index: usize) {
        let mut new_layer = self.layers[current_layer_index]
            .chunks(2)
            .map(|pair| {
                if pair.len() == 2 {
                    hash_node_string(&pair[0], &pair[1])
                } else {
                    hash_leaf_string(&pair[0])
                }
            })
            .collect::<Vec<String>>();
        self.create_new_layer(current_layer_index, new_layer.len() * 2, &mut new_layer);
    }

    fn create_new_layer(&mut self,
                        current_layer_index: usize,
                        new_layer_base_capacity: usize,
                        mut new_layer: &mut Vec<String>) {
        let layers_last_index = self.layers.len() - 1;
        if layers_last_index >= current_layer_index + 1 {
            self.layers[current_layer_index + 1].append(&mut new_layer);
        } else {
            self.layers.push(Vec::with_capacity(new_layer_base_capacity));
            self.layers[current_layer_index + 1].append(&mut new_layer);
        }
    }

    pub fn push(&mut self, other: &String) {
        debug!("ADD NEW LEAF");
        if self.builded {
            self.layers[0].push(other.clone());
            self.recursive_repair_branch(0);
        } else {
            self.layers[0].push(other.clone());
        }
    }

    fn recursive_repair_branch(&mut self, layer_index: usize) {
        debug!("PROCESS LAYER: {}", layer_index);
        if self.layers[layer_index].len() > 1 {
            if self.layers.len() - 1 == layer_index {
                debug!("CREATE NEW LAYER");
                self.layers.push(Vec::with_capacity(4));
                let left = self.layers[layer_index][0].clone();
                let right = self.layers[layer_index][1].clone();
                let new_node = hash_node_string(&left, &right);
                self.layers[layer_index + 1].push(new_node);
            } else {
                if self.layers[layer_index].len() % 2 == 1 {
                    let last_node_index = self.layers[layer_index].len() - 1;
                    let last_node = self.layers[layer_index][last_node_index].clone();
                    let new_node = hash_leaf_string(&last_node);
                    self.layers[layer_index + 1].push(new_node);
                    self.recursive_repair_branch(layer_index + 1);
                } else {
                    for current_index in layer_index..self.layers.len() - 1 {
                        let last_node_index = self.layers[current_index].len() - 1;
                        let new_node;
                        if self.layers[current_index].len() % 2 == 0 {
                            let left = self.layers[current_index][last_node_index - 1].clone();
                            let right = self.layers[current_index][last_node_index].clone();
                            new_node = hash_node_string(&left, &right);
                        } else {
                            let right = self.layers[current_index][last_node_index].clone();
                            new_node = hash_leaf_string(&right);
                        }
                        debug!("Next layer len: {}", self.layers[current_index + 1].len());
                        let next_layer_last_index = self.layers[current_index + 1].len() - 1;
                        self.layers[current_index + 1].remove(next_layer_last_index);
                        self.layers[current_index + 1].push(new_node);
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        for layer_index in 0..self.layers.len() {
            println!("Layer size: {}", self.layers[layer_index].len());
            println!("Layer index: {}", layer_index);
            for node_index in 0..self.layers[layer_index].len() {
                print!(" {}", self.layers[layer_index][node_index]);
            }
            println!("\n");
        }
    }

    pub fn audit_proof(&mut self, hash: &String) -> Result<Vec<String>, &str> {
        if !self.builded {
            self.build();
        }
        if self.layers[0].len() == 1 {
            let root_hash = hash_leaf_string(hash);
            if root_hash == self.layers[1][0] {
                return Ok(vec![root_hash])
            } else {
                return Err("Tree invalidate.");
            }
        } else {
            let (index_of_transaction, hash) = self.layers[0]
                .iter()
                .enumerate()
                .find(|&(_, element)| *element == *hash)
                .unwrap();
            let proof_path = Vec::with_capacity(16);
            self.recursive_audit_path(hash.clone(), index_of_transaction, 0, proof_path)
        }

    }

    pub fn recursive_audit_path(&self,
                                hash: String,
                                hash_index: usize,
                                layer_index: usize,
                                mut path: Vec<String>)
                                -> Result<Vec<String>, &str> {
        if self.layers[layer_index].len() > 1 {
            let pair;
            let expected_node;
            if hash_index % 2 != 0 {
                pair = self.layers[layer_index][hash_index - 1].clone();
                expected_node = hash_node_string(&pair, &hash);
            } else {
                pair = self.layers[layer_index][hash_index + 1].clone();
                expected_node = hash_node_string(&hash, &pair);
            }
            let parent_index = hash_index / 2;
            let actual_node = self.layers[layer_index + 1][parent_index].clone();
            if expected_node != actual_node {
                return Err("Tree invalidate.");
            } else {
                path.push(expected_node.clone());
                self.recursive_audit_path(expected_node, parent_index, layer_index + 1, path)
            }
        } else {
            Ok(path)
        }
    }
}

#[cfg(test)]
mod tests {

    extern crate env_logger;
    use super::MerkleTreeString;
    use super::SerializationFormat;

    #[test]
    fn build_tree() {
        let _ = env_logger::init();
        let mut merkle_tree_sequence: MerkleTreeString = MerkleTreeString::default();
        merkle_tree_sequence.parallel = false;
        merkle_tree_sequence.push(&String::from("a"));
        merkle_tree_sequence.build();
        let mut merkle_tree_parallel: MerkleTreeString = MerkleTreeString::default();
        merkle_tree_parallel.push(&String::from("a"));
        merkle_tree_parallel.build();
        assert_eq!(merkle_tree_sequence.layers, merkle_tree_parallel.layers);
    }

    #[test]
    #[should_panic(expected = "No leafs in tree!")]
    fn build_empty_tree() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::default();
        merkle_tree.build();
    }

    #[test]
    fn build_tree_one_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::default();
        merkle_tree.push(&String::from("a"));
        merkle_tree.build();
        merkle_tree.print();
    }

    #[test]
    fn build_tree_two_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::default();
        merkle_tree.push(&String::from("a"));
        merkle_tree.push(&String::from("a"));
        merkle_tree.build();
    }

    #[test]
    fn build_tree_three_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::default();
        merkle_tree.push(&String::from("a"));
        merkle_tree.push(&String::from("a"));
        merkle_tree.push(&String::from("a"));
        merkle_tree.build();
    }

    #[test]
    fn build_tree_four_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::default();
        merkle_tree.push(&String::from("a"));
        merkle_tree.push(&String::from("a"));
        merkle_tree.push(&String::from("a"));
        merkle_tree.push(&String::from("a"));
        merkle_tree.build();
        merkle_tree.print();
    }

    #[test]
    fn build_tree_from_leafs() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::from(&mut [String::from("a"),
                                                                             String::from("b"),
                                                                             String::from("c"),
                                                                             String::from("d")],
                                                                       SerializationFormat::Json);
        merkle_tree.print();
        merkle_tree.build();
        merkle_tree.print();
    }

    #[test]
    fn repair_branch() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::from(&mut [String::from("a"),
                                                                             String::from("b"),
                                                                             String::from("c"),
                                                                             String::from("d"),
                                                                             String::from("e")],
                                                                       SerializationFormat::Json);
        merkle_tree.print();
        merkle_tree.build();
        merkle_tree.print();
        println!("========================================================");
        let mut merkle_tree_two: MerkleTreeString =
            MerkleTreeString::from(&mut [String::from("a")], SerializationFormat::Json);
        merkle_tree_two.build();
        merkle_tree_two.push(&String::from("b"));
        merkle_tree_two.push(&String::from("c"));
        merkle_tree_two.push(&String::from("d"));
        merkle_tree_two.push(&String::from("e"));
        merkle_tree_two.print();
    }

    #[test]
    fn audit_proof() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTreeString = MerkleTreeString::from(&mut [String::from("a"),
                                                                             String::from("b"),
                                                                             String::from("c"),
                                                                             String::from("d"),
                                                                             String::from("e")],
                                                                       SerializationFormat::Json);
        merkle_tree.print();
        merkle_tree.build();
        merkle_tree.print();
        let proof_path = merkle_tree.audit_proof(&String::from("b")).unwrap();
        debug!("proof path: {:?}", proof_path);
        assert_eq!(vec![String::from("ab"), String::from("abcd"), String::from("abcdeeee")],
                   proof_path);
    }

}
