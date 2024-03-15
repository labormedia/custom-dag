"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.main = void 0;
var wasm_bindgen = require("../pkg");
function main() {
    // wasm_bindgen('../pkg/wasm_binding_bg.wasm');
    var none_value = wasm_bindgen.none_option_u32;
    var some_value = wasm_bindgen.some_option_u32;
    var node_prime = wasm_bindgen.new_node(1, none_value(), none_value());
    var node_a = wasm_bindgen.new_node(2, some_value(1), none_value());
    var node_b = wasm_bindgen.new_node(3, some_value(1), some_value(2));
    var node_c = wasm_bindgen.new_node(4, some_value(2), none_value());
    var node_d = wasm_bindgen.new_node(5, some_value(3), some_value(6));
    var node_e = wasm_bindgen.new_node(6, some_value(3), none_value());
    // Because of how memory is managed, reference nodes should exist while being inserted.
    var order = wasm_bindgen.topological_order([node_prime, node_a, node_b, node_c, node_e, node_d]);
    console.log("Topological Order : ", order);
}
exports.main = main;
console.log("Running example.");
main();
