# 0030 — Comparatives: adjective + than, standard mandatory

Date: 2026-09-02
Status: proposed (tentative)

## Context

The archetype "A3" of the file "docs/rewrite-archetypes.md" is the
Comparative. The rewrites dropped 5 Comparatives and turned the sentence
"length is cheaper than load" into 2 absolute sentences. The maintainer
allowed the Comparative with one condition. Every adjective must declare
the shape of the Comparative in the Seed.

## Decision

A Comparative is a Complement with a mandatory standard. The Sentence Shape
is the phrase "<subject> is <comparative> than <noun phrase>". The sentence
"the load is heavier than the length" is one example. If a writer drops the
standard, then the Linter rejects the sentence. The standard is explicit, so
a reader can check the claim.

Lexgen decides the shape of an adjective. If an adjective has one syllable,
then the adjective is short. If an adjective of 2 syllables ends with "y",
then the adjective is short. A short adjective inflects. The word "big"
becomes "bigger" and the word "easy" becomes "easier". The Form Tag of
"bigger" is "ADJ_CMP". An entry of the Seed can override the shape with the
slot "comparative". The value "none" removes the shape. A long adjective
uses the word "more". The phrase "more transparent than the Compound" is
one example. If a writer writes "more big", then the Linter maps the phrase
to the word "bigger".

The maintainers deferred 5 questions:
- the Modifier
- the phrase "less … than"
- the phrase "as … as"
- the superlatives
- the verb

The Modifier holds a Comparative in the phrase "a bigger file". The verb
holds a Comparative in the phrase "costs more than".

## Consequences

- If the comparison carried the claim, then the maintainers can revert the
  rewrite of the archetype "A3".
- A short adjective gains one Surface Form and a long adjective does not
  gain a Surface Form. The data must attest the new Surface Form. If the
  data does not attest a rare Surface Form, then the script "seedcheck.py"
  writes the Surface Form into the Seed.
- A Comparative is a distinct shape of the Copula, so the rule of the
  decision "0003" gains one clause.
