use std::{
    env,
    fs,
    error::Error,
};
use custom_dag::{
    Node,
    topological::Topology
};
use wasm_bindgen::prelude::{
    wasm_bindgen,
    JsValue,
};

#[wasm_bindgen(catch)]
pub fn some_option_u32(value:u32) -> JsValue {
    Option::<u32>::Some(value).into()
}

#[wasm_bindgen(catch)]
pub fn none_option_u32() -> JsValue {
    Option::<u32>::None.into()
}

#[wasm_bindgen]
pub fn new_node(id: u32, left: Option<u32>, right: Option<u32>) -> JsValue {
    let node_value = Node::new(id, left, right);
    serde_wasm_bindgen::to_value(&node_value).expect("Invalid format.")
}

#[wasm_bindgen]
pub fn topological_order(nodes_values: JsValue) -> JsValue {
    let nodes: Vec<Node<u32>> = serde_wasm_bindgen::from_value(nodes_values).expect("Invalid format.");
    let Some(order) = Topology::sort(&nodes).expect("Invalid format.") else { panic!("Invalid topological assumptions for this test data.") };
    serde_wasm_bindgen::to_value(&order).expect("Invalid format.")
}

#[wasm_bindgen]
pub fn shortest_and_longest_paths(nodes_values: JsValue) -> JsValue {
    let nodes: Vec<Node<u32>> = serde_wasm_bindgen::from_value(nodes_values).expect("Invalid format.");
    0_usize.into()
}

#[wasm_bindgen]
pub fn bfs_all_paths(nodes_values: JsValue, id: u32) -> JsValue {
    let nodes: Vec<Node<u32>> = serde_wasm_bindgen::from_value(nodes_values).expect("Invalid format.");
    0_usize.into()
}
