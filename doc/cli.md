# Command-line interface

Lesshand provides a command-line interface. It can be downloaded from the
[releases page][releases] or [built from source](dev/dev.md).

[releases]: https://github.com/langston-barrett/lesshand/releases

The command-line interface has two commands: `encode` and `decode`. They each
take a file path as their only argument and provide output on stdout.

The following is a shell function named `lh` that can be used to quickly open
an editor to type Lesshand. When closing the editor, the expanded text will be
copied to the clipboard. This works on Linux or macOS.
```sh
if [[ ${OSTYPE} == darwin* ]]; then
  clipboard() { pbcopy; }
else
  clipboard() { xsel -ib; }
fi

lh() {
  f=$(mktemp)
  trap "rm -f '${f}'" EXIT HUP INT QUIT TERM
  "${EDITOR}" "${f}"
  lesshand-cli decode "${f}" | clipboard
  rm -f "${f}"
}
```
(Caution: on some systems, `mktemp` may return a path with world-readable
permissions.)
