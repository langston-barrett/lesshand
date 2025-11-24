<!-- def: xref -->

# Cross-references

Lesshand aims to have extensive and accurate user- and developer-facing
documentation. To aid us in this task, we use a particular system of cross-
references and an associated tool called `xref`.

`xref` works by collecting *definitions* and *references* from comments embedded
in code and documentation. For example, a definition in a Markdown file might
look like this:
```markdown
<--! def: foo-algorithm -->
The algorithm for accomplishing `foo` works as follows...
```
The corresponding reference in a Rust file would look like:
```rust
// ref: foo-algorithm
fn foo() { /* ... */ }
```
The presence of these comments indicates that the behavior of the code following
the reference is documented at the site of the definition. The main idea is for
developers to use these comments as reminders. If a developer changes the code
near a `ref` comment, they should probably go look at the documentation around
the corresponding `def` to ensure that it is still accurate.

## The tool

While the primary purpose of these comments is not automation, the `xref` tool
does support a few light-weight checks. It checks that:

<!-- def: xref-err-ref-has-def -->
- Every `ref` refers to a matching `def`
<!-- def: xref-err-dup-defs -->
- No two `def`s have the same name
<!-- def: xref-err-def-has-ref -->
- Every `def` has at least one `ref`

In CI, the tool is run as
```sh
./x xref .github crates doc
```

## Conventions

<!-- def: xref-regex -->
The `xref` tool is quite simple. It essentially checks each line for the
following regex: ` (d|r)ef: [^ \n]+`. Conventionally, we only use kebab-case
alphanumeric identifiers.
