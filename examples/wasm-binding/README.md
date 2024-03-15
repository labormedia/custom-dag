# wasm-bingen example

# Build bindings
```
wasm-pack build --target nodejs
tsc _static/bindings.ts
node _static/bindings.js
```