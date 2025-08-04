# Requirements

This page lists requirements for abbreviations that are included with Lesshand.
These requirements are enforced by automated tests.

In the remainder of this document, we use the word *source* for the word or
phrase being abbreviated, and *target* for the abbreviated version.

## R001: Typeable  <!-- def: r001-typeable -->

Lesshand should be typeable. It's the 21st century, after all! Thus,
abbreviations must not introduce characters that do not appear on ANSI US QWERTY
keyboards.

## R002: Words  <!-- def: r002-words -->

No abbreviation should target an existing word. We check this against the most
common 4000 words from the [COCA corpus].

[COCA corpus]: https://www.wordfrequency.info/samples.asp

## R003: Effort  <!-- def: r003-effort -->

An abbreviation should save the user at least 5 [effort](effort.md), unless it
is for one of the 128 most common words in which case it should save at least 1.

### Example

"s" is not a good abbreviation for "sad", because it doesn't save much effort.

"n" is an acceptable abbreviation for "no". "no" is exceptionally common,
so this abbreviation saves considerable effort by virtue of being used so
frequently.

## R004: Valid source  <!-- def: r004-valid-src -->

The source of an abbreviation may only contain the characters `A-z`, `'`, and
` `.

## R005: Valid target  <!-- def: r005-valid-tgt -->

The target of an abbreviation may only contain the characters `A-z`, `0-9`, `'`,
, `,`, and `;`.

## R006: Unique  <!-- def: r006-unique -->

No two abbreviations may share a target.

## R007: Memorable  <!-- def: r007-memorable -->

Abbreviations must have a [memorability](memorability.md) of at least 5,
unless the abbreviation is for one of the 128 most common words and is only one
character (e.g., `for --> f`, `the --> l`).

## R008: Uniquely memorable  <!-- def: r008-uniquely-memorable -->

An abbreviation must not have a higher [memorability](memorability.md) for a
common word other than its source.

## R009: Not known to be ambiguous  <!-- def: r009-known-ambiguous -->

No abbreviations should have the following highly ambiguous targets:

- `ar`: "another", "around"
- `cm`: "come", "came"
- `ev`: "electric vehicle", "even", "ever", "every"
- `mt`: "might", "most", "must"
- `se`: "sea", "see", "set", "she"
- `sm`: "same", "some"
- `te`: "the", "their", "there"
- `th`: "the", "that", "their", "this"
- `tr`: "their", "they're"
- `wh`: "with", "what", "when", "where", "why"
- `wn`: "wane", "won", "when"
- `wr`: "were", "where"
- `wt`: "wait", "what"
- `yr`: "year", "your"

This list is simply based on human judgment.

## R010: Unambiguous  <!-- def: r010-unambiguous -->

An abbreviation should have an [ambiguity](ambiguity.md) of less than 1100,
unless it is an abbreviation for one of the 128 most common words.

### Example

"se" is an ambiguous (bad) abbreviation, because it could stand for "sea",
"see", "set", "she", etc. It has an ambiguity of 6562.
