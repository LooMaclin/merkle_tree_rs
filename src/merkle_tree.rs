use serde::Serialize;
use std::default::Default;
use serialization_format::SerializationFormat;
use rayon::prelude::*;
use hash_function::{hash_leaf, hash_node};


/// Структура хранящая и отвечающая за Merkle Tree.
#[derive(Debug, Eq, PartialEq)]
pub struct MerkleTree {
    /// Список слоёв дерева. Включая 0-ой слой с "листьями" (хэшами транзакций).
    pub layers: Vec<Vec<[u8; 32]>>,
    /// Формат сериализации транзакций перед тем как их хэшировать.
    pub format: SerializationFormat,
    /// Флаг отвечающий за параллелизацию при построении новых слоёв дерева.
    pub parallel: bool,
    /// Флаг указывающий на состояние дерева. Было оно построено или нет.
    pub builded: bool,
}

impl Default for MerkleTree {
    fn default() -> MerkleTree {
        let mut layers = Vec::with_capacity(256);
        layers.push(Vec::with_capacity(512));
        MerkleTree {
            layers: layers,
            parallel: true,
            builded: false,
            format: SerializationFormat::MsgPack,
        }
    }
}

impl MerkleTree {
    /// Производит построение дерева основываяся на 0-ом слое "листьев".
    /// # Panics
    /// В случае, если 0-ой слой "листьев" пуст - паникует.
    pub fn build(&mut self) {
        match self.layers[0].len() {
            0 => {
                panic!("No leaves in tree!");
            }
            1 => {
                debug!("Tree have one leaf. Merke root hash == hash(leaf[0])");
                let hashed_leaf = hash_leaf(&self.layers[0][0]);
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

    /// Возвращает merkle root hash в качестве 32-байтного массива.
    pub fn get_merkle_root(&self) -> [u8; 32] {
        self.layers.last().unwrap()[0].clone()
    }

    /// Производит создание "основы" Merkle tree.
    /// Принимает входной слайс транзакций, сериализует их, хэширует и добавляет в нулевой уровень.
    /// Так же заранее выделяет слои для будущего заполнения дерева резервируя чуть больше места чем нужно.
    pub fn from<Serializable>(leaves: &mut [Serializable], format: SerializationFormat) -> MerkleTree
        where Serializable: Serialize + Clone
    {
        let log2_leaves = (leaves.len() as f64).log2();
        println!("log2 leaves: {}", log2_leaves);
        let mut base_layer = leaves.iter()
            .map(|element| {
                let serialized_element = format.serialize(&element);
                hash_leaf(&serialized_element)
            })
            .collect::<Vec<[u8; 32]>>();
        let mut layer_len = base_layer.len();
        base_layer.reserve(layer_len);
        let mut layers: Vec<Vec<[u8; 32]>> = Vec::with_capacity(((log2_leaves).round()) as usize);
        layers.push(Vec::with_capacity(layer_len));
        while layer_len != 1 {
            debug!("Create new layer with capacity: {}", layer_len);
            layers.push(Vec::with_capacity(layer_len));
            layer_len = layer_len / 2;
        }
        layers[0].append(&mut base_layer);
        MerkleTree {
            layers: layers,
            parallel: true,
            builded: false,
            format: format,
        }

    }

    /// Рекурсивно создаёт слои дерева поднимаясь вверх.
    /// В зависимости от флага `parallel` делает это многопоточно, либо в одном потоке.
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

    /// Генерирует новый слой дерева параллельно.
    fn generate_new_layer_parallel(&mut self, current_layer_index: usize) {
        let mut new_layer = self.layers[current_layer_index]
            .par_chunks(2)
            .map(|pair| {
                if pair.len() == 2 {
                    hash_node(&pair[0], &pair[1])
                } else {
                    hash_leaf(&pair[0])
                }
            })
            .collect::<Vec<[u8; 32]>>();
        let current_layer_len = self.layers[current_layer_index].len();
        self.create_new_layer(current_layer_index, current_layer_len, &mut new_layer);
    }

    /// Генерирует новый слой дерева однопоточно.
    fn generate_new_layer_sequence(&mut self, current_layer_index: usize) {
        let mut new_layer = self.layers[current_layer_index]
            .chunks(2)
            .map(|pair| {
                if pair.len() == 2 {
                    hash_node(&pair[0], &pair[1])
                } else {
                    hash_leaf(&pair[0])
                }
            })
            .collect::<Vec<[u8; 32]>>();
        self.create_new_layer(current_layer_index, new_layer.len() * 2, &mut new_layer);
    }

    /// Создаёт новый слой, если предсгенерированных слоёв в дереве не хватает.
    /// Резервирует место для последующих вставок.
    fn create_new_layer(&mut self,
                        current_layer_index: usize,
                        new_layer_base_capacity: usize,
                        mut new_layer: &mut Vec<[u8; 32]>) {
        let layers_last_index = self.layers.len() - 1;
        if layers_last_index >= current_layer_index + 1 {
            self.layers[current_layer_index + 1].append(&mut new_layer);
        } else {
            self.layers.push(Vec::with_capacity(new_layer_base_capacity));
            self.layers[current_layer_index + 1].append(&mut new_layer);
        }
    }

    /// Добавляет хэш сериализованной транзакции в слой "листьев" (0-ой слой).
    /// Если дерево было до этого построено - вызывает функцию пересчёта узлов дерева.
    pub fn push<Serializable>(&mut self, other: &Serializable)
        where Serializable: Serialize
    {
        debug!("ADD NEW LEAF");
        if self.builded {
            let serialized_other = self.format.serialize(&other);
            let hashed_other = hash_leaf(&serialized_other);
            self.layers[0].push(hashed_other);
            self.recursive_repair_branch(0);
        } else {
            let serialized_other = self.format.serialize(&other);
            let hashed_other = hash_leaf(&serialized_other);
            self.layers[0].push(hashed_other);
        }
    }

    ///
    /// Рекурсивно "восстанавливает" ветку. Либо добавляя новые узлы, либо заменяя старые узлы,
    /// если их хэш должен быть обновлён. В случае, если текущий уровень равен последнему,
    /// а количество узлов на нём больше единицы - добавляет новый слой и добавляет туда новый
    /// merkle root hash.
    fn recursive_repair_branch(&mut self, layer_index: usize) {
        debug!("PROCESS LAYER: {}", layer_index);
        if self.layers[layer_index].len() > 1 {
            if self.layers.len() - 1 == layer_index {
                debug!("CREATE NEW LAYER");
                self.layers.push(Vec::with_capacity(4));
                let left = self.layers[layer_index][0];
                let right = self.layers[layer_index][1];
                let new_node = hash_node(&left, &right);
                self.layers[layer_index + 1].push(new_node);
            } else {
                if self.layers[layer_index].len() % 2 == 1 {
                    let last_node_index = self.layers[layer_index].len() - 1;
                    let last_node = self.layers[layer_index][last_node_index];
                    let new_node = hash_leaf(&last_node);
                    self.layers[layer_index + 1].push(new_node);
                    self.recursive_repair_branch(layer_index + 1);
                } else {
                    for current_index in layer_index..self.layers.len() - 1 {
                        let last_node_index = self.layers[current_index].len() - 1;
                        let new_node;
                        if self.layers[current_index].len() % 2 == 0 {
                            let left = self.layers[current_index][last_node_index - 1];
                            let right = self.layers[current_index][last_node_index];
                            new_node = hash_node(&left, &right);
                        } else {
                            let right = self.layers[current_index][last_node_index];
                            new_node = hash_leaf(&right);
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


    /// Простая функция, которая была использована для debug.
    /// Печатает дерево послойно.
    pub fn print(&self) {
        for layer_index in 0..self.layers.len() {
            println!("Layer size: {}", self.layers[layer_index].len());
            println!("Layer index: {}", layer_index);
            for node_index in 0..self.layers[layer_index].len() {
                print!("\n{:?}\n", self.layers[layer_index][node_index]);
            }
            println!("\n");
        }
    }

    /// Производит поиск хэша транзакции в слое "листьев" дерева и поднимаясь до корня рекурсивно
    /// проверяет корректность всех хэшей на пути для этого хэша транзакции. Возвращает так называемый
    /// proof path - т.е список хэшей, который подтверждает, что хэш транзакции был использован при
    /// построении дерева.
    ///
    /// # Failures
    /// В случае, если какой-либо из узлов на пути инвалидирован, т.е хэш актуальных в дереве значений
    /// не совпадает с вычисленным либо наоборот - возвращается ошибка `Tree invalidate`.
    ///
    pub fn audit_proof(&mut self, hash: &[u8; 32]) -> Result<Vec<[u8; 32]>, &str> {
        if !self.builded {
            self.build();
        }
        if self.layers[0].len() == 1 {
            let root_hash = hash_leaf(hash);
            if root_hash == self.layers[1][0] {
                return Ok(vec![root_hash]);
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
            self.recursive_audit_path(*hash, index_of_transaction, 0, proof_path)
        }

    }

    /// Поднимается до корня рекурсивно, проверяет корректность всех хэшей на пути для этого хэша транзакции.
    /// Возвращает так называемый proof path - т.е список хэшей, который подтверждает, что хэш транзакции был использован при
    /// построении дерева.
    ///
    /// # Failures
    /// В случае, если какой-либо из узлов на пути инвалидирован, т.е хэш актуальных в дереве значений
    /// не совпадает с вычисленным либо наоборот - возвращается ошибка `Tree invalidate`.
    ///
    pub fn recursive_audit_path(&self,
                                hash: [u8; 32],
                                hash_index: usize,
                                layer_index: usize,
                                mut path: Vec<[u8; 32]>)
                                -> Result<Vec<[u8; 32]>, &str> {
        if self.layers[layer_index].len() > 1 {
            let pair;
            let expected_node;
            if hash_index % 2 != 0 {
                pair = self.layers[layer_index][hash_index - 1];
                expected_node = hash_node(&pair, &hash);
            } else {
                pair = self.layers[layer_index][hash_index + 1];
                expected_node = hash_node(&hash, &pair);
            }
            let parent_index = hash_index / 2;
            let actual_node = self.layers[layer_index + 1][parent_index];
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
    use super::MerkleTree;
    use super::SerializationFormat;
    use super::{hash_leaf, hash_node};

    #[test]
    fn build_tree() {
        let _ = env_logger::init();
        let mut merkle_tree_sequence: MerkleTree = MerkleTree::default();
        merkle_tree_sequence.parallel = false;
        merkle_tree_sequence.push(&["a"]);
        merkle_tree_sequence.build();
        let mut merkle_tree_parallel: MerkleTree = MerkleTree::default();
        merkle_tree_parallel.push(&["a"]);
        merkle_tree_parallel.build();
        assert_eq!(merkle_tree_sequence.layers, merkle_tree_parallel.layers);
    }

    #[test]
    #[should_panic(expected = "No leaves in tree!")]
    fn build_empty_tree() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::default();
        merkle_tree.build();
    }

    #[test]
    fn build_tree_one_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::default();
        merkle_tree.push(&["a"]);
        merkle_tree.build();
        merkle_tree.print();
    }

    #[test]
    fn build_tree_two_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::default();
        merkle_tree.push(&["a"]);
        merkle_tree.push(&["a"]);
        merkle_tree.build();
    }

    #[test]
    fn build_tree_three_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::default();
        merkle_tree.push(&["a"]);
        merkle_tree.push(&["a"]);
        merkle_tree.push(&["a"]);
        merkle_tree.build();
    }

    #[test]
    fn build_tree_four_leaf() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::default();
        merkle_tree.push(&["a"]);
        merkle_tree.push(&["a"]);
        merkle_tree.push(&["a"]);
        merkle_tree.push(&["a"]);
        merkle_tree.build();
        merkle_tree.print();
    }

    #[test]
    fn build_tree_from_leaves() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::from(&mut ["a", "b", "c", "d"],
                                                           SerializationFormat::Json);
        merkle_tree.print();
        merkle_tree.build();
        merkle_tree.print();
    }

    #[test]
    fn repair_branch() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::from(&mut ["a", "b"],
                                                           SerializationFormat::Json);
        merkle_tree.print();
        merkle_tree.build();
        let serialized_hashed_a =
            hash_leaf(&SerializationFormat::Json.serialize(&String::from("a")));
        debug!("serialized hashed a: {:?}", serialized_hashed_a);
        let serialized_hashed_b =
            hash_leaf(&SerializationFormat::Json.serialize(&String::from("b")));
        debug!("serialized hashed a: {:?}", serialized_hashed_a);
        let merkle_root_hash_of_a_and_b = hash_node(&serialized_hashed_a, &serialized_hashed_b);
        debug!("merkle root hash of a and b transactions: {:?}",
               merkle_root_hash_of_a_and_b);
        assert_eq!(merkle_root_hash_of_a_and_b, merkle_tree.get_merkle_root());
        merkle_tree.print();
        let mut merkle_tree_two: MerkleTree = MerkleTree::from(&mut ["a"],
                                                               SerializationFormat::Json);
        merkle_tree_two.build();
        merkle_tree_two.push(&String::from("b"));
        merkle_tree_two.print();
        assert_eq!(merkle_tree_two.get_merkle_root(),
                   merkle_root_hash_of_a_and_b);
        assert_eq!(merkle_tree_two, merkle_tree);
    }

    #[test]
    fn audit_proof() {
        let _ = env_logger::init();
        let mut merkle_tree: MerkleTree = MerkleTree::from(&mut ["a", "b", "c", "d", "e"],
                                                           SerializationFormat::Json);
        merkle_tree.print();
        merkle_tree.build();
        merkle_tree.print();
        let proof_path = merkle_tree.audit_proof(&[172, 141, 131, 66, 187, 178, 54, 45, 19, 240, 165, 89, 163, 98, 27, 180, 7, 1, 19, 104, 137, 81, 100, 182, 40, 165, 79, 127, 195, 63, 196, 60]).unwrap();
        debug!("proof path: {:?}", proof_path);
        assert_eq!(vec![[224, 150, 20, 187, 185, 181, 68, 71, 210, 163, 91, 57, 42, 191, 172,
                         41, 131, 97, 49, 134, 252, 218, 101, 205, 255, 82, 137, 117, 72, 120,
                         140, 89],
                        [243, 222, 175, 150, 147, 229, 193, 133, 250, 27, 86, 246, 219, 151, 51,
                         244, 155, 175, 6, 23, 223, 146, 224, 116, 207, 83, 107, 170, 154, 252,
                         5, 32],
                        [50, 111, 53, 115, 90, 175, 184, 3, 43, 167, 87, 45, 15, 87, 106, 177,
                         119, 3, 240, 177, 194, 92, 104, 105, 85, 17, 37, 18, 59, 224, 113, 39]],
                   proof_path);
    }

}
