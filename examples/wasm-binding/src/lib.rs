use std::{
    env,
    fs,
    error::Error,
};
use custom_dag::{
    Node,
    topological::Topology
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
