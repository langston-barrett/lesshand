# Memorability

<!-- def: memorability -->

Lesshand should be easy to learn, so its abbreviations should be easy to
remember. It should also be easy to read, so its abbreviations should echo the
word or phrase they abbreviate. To evaluate these aspects of Lesshand, we assign
every abbreviation a score called *memorability*. Good abbreviations have high
memorability.

<!-- ref: r007-memorable -->
[R007: Memorable](reqs.md#r007-memorable) requires that all abbreviations in
Lesshand have a memorability of more than 5, except those for the 128 most
common words.

Memorability is additive. For an abbreviation `long --> short`, it works as
follows:

- Add 3 if `short` starts with the same letter as `long`
- Add 3 if `short` ends with the same letter as `long`
- Add the length of `short` if it is a prefix of `long`
- Add the length of `short` if it is equal to `long` without all of the vowels
- Add 1 for each character that `short` shares with `long` (if they appear in
  the same order)
- Add 10 for being a contraction of another abbreviation (e.g., `cn'` gets +10
  because of `cn`)
- Remove 1 if `long` contains an apostrophe and `short` does not.

For example:

- `and --> d`: has a memorabilty of 5
- `can --> cn`: has a memorability of 10
- `government --> govt`: has a memorability 10
- `if I remember correctly --> ,iirc` has a memorability of 7
