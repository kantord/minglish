# 0030 — Comparatives: adjective + than, standard mandatory

Date: 2026-09-02
Status: proposed (tentative)

## Context

The file "docs/rewrite-archetypes.md" has the archetype "A3". The archetype "A3" is the Comparative. The rewrites dropped 5 Comparatives. The rewrite turned the sentence "length is cheaper than load" into 2 absolute sentences. The maintainer said one thing. The language allows the Comparative. Every adjective needs a definition of the shape.

## Decision

A Comparative is a Complement. A Comparative has a mandatory standard.
The Sentence Shape is the phrase "<subject> is <comparative> than <noun phrase>". The sentence "the load is heavier than the length" is one
example. If a Comparative does not have the standard, then the Linter rejects the Comparative. The standard is explicit, so a reader can check the claim.

Lexgen decides the shape of an adjective. A short adjective has one
syllable. A short adjective inflects. The word "bigger" is one example. The
word "easier" is one example. The Form Tag of the shape is "ADJ_CMP". The slot "comparative" overrides the shape. The value "none" removes the shape. A long
adjective uses the word "more". The phrase "more transparent than the
Compound" is one example. If a writer writes "more big", then the Linter
maps the phrase to the word "bigger".

The maintainers deferred 5 questions:
- the Modifier
- the phrase "less … than"
- the phrase "as … as"
- the superlatives
- the verb

The phrase "a bigger file" puts a Comparative into a Modifier. The phrase "costs more than" puts a Comparative on a verb.

## Consequences

- If a comparison was the claim, then the maintainers can revert the
  rewrite of the archetype "A3".
- Every adjective gains one Surface Form at the maximum. The attestation covers the shape. The script "seedcheck.py" acknowledges a rare shape.
- A Comparative is a distinct shape of the Copula. The rule of the decision
  "0003" gains one addition.
