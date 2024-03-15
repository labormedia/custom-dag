# wasm-bingen example

# Install tools
````
npm install -g typescript
```

# Build bindings
```
wasm-pack build --target nodejs
tsc _static/bindings.ts
node _static/bindings.js
```