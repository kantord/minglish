# 0009 — Modals: must and can only; may banned

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0005, 0007)

## Context

A policy needs normative sentences. An instruction needs normative
sentences. A normative sentence has one force. The language has 4 forces:
- the obligation
- the Prohibition
- the permission
- the ability

A modal of English is ambiguous. The word "may" marks a permission or marks
a possibility. The word "can" marks an ability or marks a permission. The
word "should" opens a scale of the obligation. The word "will" marks the
Tense. The word "would" marks the Tense. If an ambiguous word does not have
a synonym, then the decision "0008" bans the word. A modal is dense. One
token carries the whole force. The decision "0006" prefers a dense text.

## Decision

The language enables 3 modals:
- "must"
- "can"
- "cannot"

The Form Tag of "must" is "MODAL_MUST". The word "must" marks an
obligation. The phrase "must not" marks a Prohibition. The Form Tag of
"can" is "MODAL_CAN". The word "can" marks an ability or marks a
permission. The language merges the 2 senses. The Form Tag of "cannot" is
"MODAL_CAN_NEG". The word "cannot" is one token.

The language bans the word "may". A writer replaces a permission with the
word "can". A writer restructures a possibility into a Conditional.

The maintainers deferred 6 words:
- "should"
- "will"
- "would"
- "shall"
- "might"
- "could"

A modal takes a verb. A modal does not take a modal. A modal does not take
a Copula. The phrase "must be old" is one example of a Ban.

## Consequences

- The language says an obligation with one token. The language says a
  Prohibition with one token. The language says a permission with one
  token. The language says an ability with one token. Every token has one
  meaning.
- The language cannot say a possibility. The Gap is acceptable. If the
  corpus needs a possibility, then the maintainers revisit the Gap.
- The corpus gives the Form Tag "AUX" to a modal. Triage accepts the Form
  Tag "AUX" for a modal.
