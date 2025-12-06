#!/usr/bin/env python3

"""Fetch the Enron email corpus"""

from argparse import ArgumentParser
from gzip import GzipFile
from hashlib import sha256
from pathlib import Path
from tarfile import TarFile
from urllib.request import urlopen

HERE = Path(__file__).absolute().parent
TAR = Path("enron_mail_20150507.tar.gz")
URL = f"https://www.cs.cmu.edu/~enron/{TAR.name}"
OUT = HERE / "enron"

if __name__ == "__main__":
    parser = ArgumentParser(description=__doc__)
    args = parser.parse_args()

    if OUT.exists():
        print("Not overwriting", str(OUT))
        exit(1)

    OUT.mkdir()

    (op, arg) = (urlopen, URL)
    if TAR.exists():
        (op, arg) = (open, TAR)  # type: ignore[assignment]

    # TODO(lb): Remove headers, rename files
    with op(arg) as stream:  # ty: ignore[invalid-argument-type]
        with GzipFile(
            filename=TAR,
            fileobj=stream,  # ty: ignore[invalid-argument-type]
        ) as gz:
            with TarFile(name=TAR.with_suffix(".tar"), fileobj=gz) as tar:
                info = tar.next()
                while info is not None:
                    print(info.name)

                    # Filename and directory
                    nm0 = sha256(info.name.encode("utf8")).hexdigest()
                    d = OUT / str(nm0[:2])
                    d.mkdir(exist_ok=True)
                    nm = nm0[2:]
                    out_file = (d / nm).with_suffix(".txt")

                    info = tar.next()
                    if info is None:
                        continue
                    f = tar.extractfile(info)
                    if f is None:
                        continue
                    with open(out_file, mode="wb") as out:
                        go = False
                        for line in f:
                            if go:
                                out.write(line + b"\n")
                            else:
                                go |= line == b"\r\n"
                    assert len(out_file.read_bytes())

    assert TAR.exists()
