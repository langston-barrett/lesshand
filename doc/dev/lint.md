# Linting and formatting

Lesshand uses several code linters and formatters.

## Format

### Rust

Rust code formatted with [rustfmt]. You can install rustfmt with [rustup] like
so:

```sh
rustup component add rustfmt
```

and then run it like this:

```sh
cargo fmt --all
```

[rustfmt]: https://rust-lang.github.io/rustfmt
[rustup]: https://rustup.rs/

### Python

Python code formatted with [ruff].

<!-- def: ruff-format -->
```sh
ruff format ./**/*.py
```

[ruff]: https://astral.sh/ruff

## Lint

### Rust

All code should pass [Clippy][clippy]. You can install Clippy with rustup
like so:

```sh
rustup component add clippy
```

and then run it like this:

```sh
cargo clippy --workspace -- --deny warnings
```

[clippy]: https://doc.rust-lang.org/stable/clippy/

### Python

Python code linted with [ruff].

<!-- def: ruff-check -->
```sh
ruff check ./**/*.py
```

[ruff]: https://astral.sh/ruff

### Markdown

We run [typos] on the README and `doc/` to spell-check the documentation. To run
it locally, try:

```bash
typos README.md doc/
```

[typos]: https://github.com/crate-ci/typos

### GitHub Actions

We run [zizmor] on our workflows.

```bash
zizmor .github
```

[zizmor]: https://docs.zizmor.sh/
