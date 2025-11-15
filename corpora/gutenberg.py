#!/usr/bin/env python

"""Fetch the Project Gutenberg corpus"""

from argparse import ArgumentParser
from hashlib import sha256
from pathlib import Path
from subprocess import check_call

HERE = Path(__file__).absolute().parent
IN = HERE / "gutenberg-in"
OUT = HERE / "gutenberg-out"

START = b"*** START OF THE PROJECT GUTENBERG EBOOK"
END = b"*** END OF THE PROJECT GUTENBERG EBOOK"

if __name__ == "__main__":
    parser = ArgumentParser(description=__doc__)
    args = parser.parse_args()

    if not IN.exists():
        check_call(
            [
                "rsync",
                "-amv",
                "--include",
                "*/",
                "--include",
                "[p123456789][g0123456789]*[.-][t0][x.]t[x.]*[t8]",
                "--exclude",
                "*",
                "aleph.gutenberg.org::gutenberg",
                str(IN),
            ]
        )
    assert IN.exists()

    if OUT.exists():
        print("Not overwriting", str(OUT))
        exit(1)

    OUT.mkdir()
    ins = tuple(IN.glob("**/*.txt"))
    for i, in_file in enumerate(ins):
        print(i, "/", len(ins))
        text = in_file.read_bytes()
        nm0 = sha256(text).hexdigest()
        d = OUT / str(nm0[:2])
        d.mkdir(exist_ok=True)
        nm = nm0[2:]
        out_file = (d / nm).with_suffix(".txt")
        with open(out_file, mode="wb") as out:
            book = START not in text
            for line in text.splitlines():
                book &= not line.startswith(END)
                if book:
                    out.write(line + b"\n")
                book |= line.startswith(START)
        out_text = out_file.read_bytes()
        assert len(out_text) > 0
