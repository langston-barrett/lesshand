# Evaluation

This page describes how Lesshand is evaluated against its [goals](goals.md).

We evaluated Lesshand by calculating our [metrics](goals.md) on a corpus
consisting of the following texts:

- [*Pride and prejudice*](https://www.gutenberg.org/files/1342/1342-0.txt) by Jane Austin
- [*The Importance of Being Earnest*](https://www.gutenberg.org/files/844/844-0.txt) by Oscar Wilde
- The Wikipedia pages:
  - [Ethics](https://en.wikipedia.org/wiki/Ethics)
  - [Ocean](https://en.wikipedia.org/wiki/Ocean)
  - [Wheel](https://en.wikipedia.org/wiki/Wheel)

The results were as follows:

- Typing or handwriting these texts in Lesshand would be about 13% [easier](effort.md).
- Encoding the texts in Lesshand reduced their size (number of characters) by 11.93%.
- Decoding the texts as if they were written in Lesshand produced a 0.06% change in content, providing good evidence that Lesshand is lossless.
- Encoding the texts twice produced *identical* results to doing so once, again providing good evidence that Lesshand is lossless.
- Roundtripping (encoding then decoding) the texts produced a result that was only 0.13% different from the original. Further investigation reveals that most of this is actually due to technical bugs in the software implementation, rather than the abbreviation scheme itself.

You can re-run this evaluation by collecting the above texts into `corpus.txt` and running:
```sh
cargo run -r --bin=lesshand-design -- metrics corpus.txt
```

## Most effective abbreviations

We further collected data on the most effective abbreviations. The top 7
abbreviations cumulatively save about 5% [effort](effort.md), the top 20
cumulatively save 8.5%, and the top 200 save about 12%. The following list shows
the top 20 in decreasing order of effectiveness, along with how much effort each
one saves.

```
the --> l: 1.71% (cumulative: 1.71%)
and --> d: 1.03% (cumulative: 2.74%)
to --> t: 0.70% (cumulative: 3.44%)
that --> tt: 0.46% (cumulative: 3.90%)
not --> x: 0.45% (cumulative: 4.35%)
you --> u: 0.44% (cumulative: 4.79%)
of --> o: 0.44% (cumulative: 5.24%)
was --> w: 0.36% (cumulative: 5.60%)
for --> f: 0.35% (cumulative: 5.95%)
her --> hr: 0.32% (cumulative: 6.27%)
they --> e: 0.25% (cumulative: 6.51%)
from --> m: 0.23% (cumulative: 6.75%)
very --> v: 0.23% (cumulative: 6.98%)
have --> hv: 0.23% (cumulative: 7.21%)
I --> i: 0.22% (cumulative: 7.43%)
this --> h: 0.22% (cumulative: 7.65%)
could --> cd: 0.20% (cumulative: 7.85%)
is --> s: 0.20% (cumulative: 8.05%)
be --> b: 0.20% (cumulative: 8.25%)
were --> ee: 0.19% (cumulative: 8.45%)
```

You can re-run this evaluation by collecting the above texts into `corpus.txt` and running:
```sh
cargo run -r --bin=lesshand-design -- top corpus.txt
```
