# 0009 — Modals: must and can only; may banned

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0005, 0007)

## Context

A policy needs normative sentences and an instruction needs normative
sentences. Normative sentences carry 4 forces:
- the obligation
- the Prohibition
- the permission
- the ability

The modals of English are ambiguous. The word "may" marks a permission or
marks a possibility. The word "can" marks an ability or marks a permission.
The obligation of "must" is stronger than the obligation of "should", so
the word "should" opens a scale of the obligation. The word "will" marks
the Tense and the word "would" marks the Tense. The decision "0008" gives
the rule. If the senses of an ambiguous word do not have a synonym, then
the word is a Ban. The word is not a Redirect. A modal is dense, because
one token carries the whole force. The density is precise and is
cheap. The decision "0006" prefers a dense text, so a modal follows the
decision.

## Decision

The language allows 3 modals:
- "must"
- "can"
- "cannot"

The word "must" carries the Form Tag "MODAL_MUST" and marks an obligation.
The phrase "must not" marks a Prohibition, because the word "not" follows
the modal. The word "can" carries the Form Tag "MODAL_CAN". The word marks an
ability or marks a permission. The maintainers merged the 2 senses with a
fiat decision, so the word "can" has one meaning. The meaning covers the
phrase "is able to" and covers the phrase "is allowed to". The word
"cannot" carries the Form Tag "MODAL_CAN_NEG". The common orthography does
not separate the word "can" from the word "not", so the word "cannot" is
one token.

The language bans the word "may". The word "can" replaces a permission. A
writer restructures a possibility into a Conditional or adds the word
"sometimes".

The maintainers deferred 6 words:
- "should"
- "will"
- "would"
- "shall"
- "might"
- "could"

The scale of the obligation defers the word "should". The Tense defers the
word "will" and defers the word "would".

A modal takes a base verb and does not take a modal. The first version of
the language did not allow the phrase "must be old", because a modal did
not take a Copula. The decision "0032" allows a Copula after a modal, so
the phrase "must be old" became legal.

## Consequences

- The language says every force with one token. Every token has one
  meaning.
- The language does not mark a possibility with one token, so a
  possibility is a Gap. The maintainers accept the Gap. If the corpus
  needs a possibility, then the maintainers revisit the Gap with the
  evidence.
- The standard UD gives the Form Tag "AUX" to every modal of the corpus.
  Triage maps the Form Tag "AUX" to a modal, so Triage accepts the token.
