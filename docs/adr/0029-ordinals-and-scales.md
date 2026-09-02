# 0029 — Ordinals, scale words, decimals

Date: 2026-09-02
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

The rewrite of the 28 decisions met 6 Ordinals. The file
"docs/rewrite-archetypes.md" has the archetype "A1". The archetype "A1" is the Ordinal. A
rewrite dropped the Ordinal or turned the Ordinal into a chain. The maintainer said one thing. The language cannot skip the
Ordinals. A big number did not have a shape. The phrase "20 million"
is one example. A decimal did not have a shape. The phrase "3.14" is one
example.

## Decision

The language has 3 Ordinals:
- "first"
- "second"
- "third"

The Form Tag of the 3 words is "ORD". The 4th Ordinal uses the digits. Every later Ordinal uses the digits. A suffix follows the digits. The phrase "4th" is one example. The phrase
"21st" is one example. The Linter maps the string "1st" to the word
"first". The Linter maps the string "2nd" to the word "second". The Linter
maps the string "3rd" to the word "third". Every Ordinal has one shape. An Ordinal follows the determiner. An Ordinal precedes the noun. The phrase "the first
word" is one example. The phrase "the 4th file" is one example.

The language has 3 Scale Words:
- "thousand"
- "million"
- "billion"

A Scale Word follows the digits. A Scale Word precedes the plural noun. The phrase "20 million files" is one example. A Scale Word needs the digits. A Scale Word does not follow a Number Word.

A decimal is a quantity. The digits write a decimal. The phrase "3.5 minutes" is one example.
A round integer prefers a Scale Word. The phrase "20 thousand" is better
than the string "20000". The Linter does not enforce the preference. A
decimal stays in digits.

The maintainers deferred 4 questions:
- the bare Ordinal
- the fractions
- the ranges
- the superlatives

The phrase "the first" is a bare Ordinal. The word "half" is
a fraction. The phrase "3 to 5 files" is a range. The word "largest" is a
superlative.

## Consequences

- If an Ordinal was the claim, then the maintainers can revert the rewrite
  of the archetype "A1".
- The Lexer gains 2 shapes. The Lexicon gains 6 Function Words.
- The Ordinal "second" excludes the unit "second". A duration uses the word "minute". The maintainers decide the units in a future decision.
