# Goals and metrics

This page describes the goals of Lesshand, and metrics that are used to evaluate
whether Lesshand meets them. See [Evaluation](evaluation.md) for how Lesshand is
evaluated against these metrics.  See also [Requirements](reqs.md) for the
hard-and-fast rules that Lesshand's abbreviations conform to.

## Lossless

Lesshand should be lossless.

Related requirements:

- [R002: Words](./reqs.md#r002-words)
- [R006: Unique](./reqs.md#r006-unique)

<!-- TODO: put this somewhere else

The vast majority of shorthand systems allow for ambiguity to achieve brevity.
For instance, many systems specify that you should leave out vowels that
appear in the middle of words. This is an example of "lossy compression". Lossy
compression can't be unambiguously reversed, e.g., did "rd" stand for "reed",
"read", or "red"?

A *lossless* system has some advantages:

- Communicability: Lesshand can be decoded by a computer program, so you
  can write or type a document in Lesshand and easily share it with others.
  Moreover, you can share your shorthand directly with other Lesshand users with
  no fear of misunderstanding.

- Precision: You'll never have to worry that you would misinterpret your own
  writing, even if you return to it in 20 years.

-->

### Metric: Round-tripping

<!-- def: metric-roundtrip -->

Encoding and decoding should preserve more than 99% of English writing.

### Metric: Idempotency

<!-- def: metric-idempotent -->

Encoding should be more than 99% idempotent. That is, encoding twice should be
(almost exactly) the same as encoding once.

## Effective

Lesshand should save the user effort.

Related requirements:

- [R003: Effort](./reqs.md#r003-effort)

### Metric: Effort ratio

<!-- def: metric-effort -->

Encoding should reduce [effort](effort.md) by more than 5%.

### Metric: Compression ratio

<!-- def: metric-compression -->

Encoding should reduce text length by more than 3%.

## Incremental

You can get started right away and gradually increase your speed as you learn
more of Lesshand. You can freely intermix encoded and un-encoded text, i.e., you
won't "accidentally" write encoded versions of anything.

Related requirements:

- [R002: Words](./reqs.md#r002-words)
- [R006: Memorable](./reqs.md#r006-memorable)
- [R007: Uniquely memorable](./reqs.md#r006-uniquely-memorable)

### Metric: Decoding normal text

<!-- def: metric-decode -->

Decoding non-shorthand text should alter less than 3% of the text.
