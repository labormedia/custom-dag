"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.main = void 0;
var wasm_bindgen = require("../pkg");
function main() {
    // wasm_bindgen('../pkg/wasm_binding_bg.wasm');
    var none_value = wasm_bindgen.none_option_u32;
    var some_value = wasm_bindgen.some_option_u32;
    var node_a = wasm_bindgen.new_node(0, none_value(), none_value());
    var node_b = wasm_bindgen.new_node(1, some_value(0), some_value(0));
    console.log("Hello", node_a, node_b);
    var order = wasm_bindgen.topological_order([node_a, node_b]);
    console.log("Topological Order : ", order);
}
exports.main = main;
console.log("Running example.");
main();
