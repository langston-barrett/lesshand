# Automation

Lesshand includes a Cargo package called `x` that provides ad-hoc automation of
development tasks. It can be run with the script at the top level of the repo
that is called `x`. Try `x --help`.

## Building the WebAssembly demo assets

To build the WebAssembly bindings used by the docs page, run:

```
./x wasm
```

This requires `wasm-pack` to be installed. Install it with:

```
cargo install wasm-pack
```

The output is written to `doc/wasm/pkg/`, which the mdBook page
`doc/wasm-demo.md` loads at runtime.
