# Ambiguity

<!-- def: ambiguity -->

Lesshand should be easy to read and write, so its abbreviations should be
reasonably unambiguous. To evaluate this aspect of Lesshand, we assign every
abbreviation a score called *ambiguity*. Good abbreviations have low ambiguity.

<!-- ref: r010-unambiguous -->
[R010: Unambiguous](reqs.md#r010-unambiguous) requires that all abbreviations in
Lesshand have an ambiguity of less than 1100.

The calculation of ambiguity is somewhat complex. It is based on summing the
[memorability](memorability.md) of an abbreviation as if it were an abbreviation
for any of the most common 4000 words in English, weighted by the frequency of
the word.
