pub fn hash_leaf_string(value: &String) -> String {
    let mut result = value.clone();
    result.push_str(value.clone().as_str());
    result
}

pub fn hash_node_string(left: &String, right: &String) -> String {
    let mut result = left.clone();
    result.push_str(right.as_str());
    result
}
