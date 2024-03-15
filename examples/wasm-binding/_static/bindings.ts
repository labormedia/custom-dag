import * as wasm_bindgen from '../pkg';

export function main() {
    // wasm_bindgen('../pkg/wasm_binding_bg.wasm');

    let none_value = wasm_bindgen.none_option_u32;
    let some_value = wasm_bindgen.some_option_u32;
    const node_a = wasm_bindgen.new_node(0, none_value(), none_value());
    const node_b = wasm_bindgen.new_node(1, some_value(0), some_value(0));
    console.log("Hello", node_a, node_b);

    let order = wasm_bindgen.topological_order([node_a,node_b]);
    console.log("Topological Order : ", order);
}

console.log("Running example.")
main();