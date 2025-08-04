# Developer documentation

## Build

To install from source, you'll need to install Rust and [Cargo][cargo]. Follow
the instructions on the [Rust installation page][install-rust]. Then, get the
source:

```bash
git clone https://github.com/langston-barrett/lesshand
cd lesshand
```

Finally, build everything:

```bash
cargo build --release
```

You can find binaries in `target/release`. Run tests with `cargo test`.

[cargo]: https://doc.rust-lang.org/cargo/
[install-rust]: https://www.rust-lang.org/tools/install

## Docs

HTML documentation can be built with [mdBook][mdbook]:

```sh
cd doc
mdbook build
# or
mdbook serve
```

[mdbook]: https://rust-lang.github.io/mdBook/

## Linting and formatting

See [Linting and formatting](lint.md).


## Profile

<!-- TODO: dhat, Poor Man's profiler

### perf

```sh
cargo build --profile=profiling
perf record ./target/profiling/lesshand-design metrics corpora/gutenberg-out/ff/f168b70c0d6881f52cbde414ca2f6c4d7287e827649bed6a892550b178d3c6.txt
```

-->

### Samply

To use [Samply]:

[Samply]: https://github.com/mstange/samply

```sh
cargo install samply
cargo build --profile=profiling
samply record ./target/profiling/lesshand-design metrics corpora/gutenberg-out/ff/f168b70c0d6881f52cbde414ca2f6c4d7287e827649bed6a892550b178d3c6.txt
```

## Warnings

Warnings are disallowed in the CI build. Warnings are configured in the top-
level `Cargo.toml` file.  We reject all warn-by-default `rustc` lints, as well
as a subset of [allowed-by-default lints]. The goal is to balance high-quality,
maintainable code with not annoying developers.

To allow a lint in one spot, use:

```rust
#[allow(name_of_lint)]
```

[allowed-by-default lints]: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html

## Speeding things up

To speed things up a bit, try putting this in `.cargo/config.toml`:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```
