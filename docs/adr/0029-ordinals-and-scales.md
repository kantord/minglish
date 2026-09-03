# 0029 — Ordinals, scale words, decimals

Date: 2026-09-02
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

The rewrite of the 28 decisions found 6 Ordinals, so the archetype "A1" of
the file "docs/rewrite-archetypes.md" covers the Ordinal. Some rewrites
dropped the Ordinal. Some rewrites turned the Ordinal into a chain.
The chain ranked 2 nouns in every sentence. The maintainer rejected the
loss, because the language needs the Ordinals. The Lexer did not recognize
2 quantities:
- the big number "20 million"
- the decimal "3.14"

## Decision

The language has 3 Ordinals:
- "first"
- "second"
- "third"

The 3 words have the Form Tag "ORD". Every later Ordinal uses the digits
with a suffix. An Ordinal follows the determiner and precedes the noun. The
language allows 3 phrases:
- "the first word"
- "the 4th file"
- "the 21st file"

Every Ordinal has one spelling. The Linter rejects 3 strings:
- "1st"
- "2nd"
- "3rd"

The Linter maps the 3 strings to the 3 words, so the string "1st" becomes
the word "first".

The language has 3 Scale Words:
- "thousand"
- "million"
- "billion"

A Scale Word follows the digits and precedes the plural noun. The phrase
"20 million files" is one example. A Scale Word needs the digits, so a
Number Word cannot precede a Scale Word.

A decimal is a quantity in digits. The phrase "3.5 minutes" is one example.
The style prefers a Scale Word for a round integer, so the phrase "20
thousand" is better than the string "20000". The Linter does not enforce
the preference. A decimal stays in digits.

The maintainers deferred 4 questions:
- the bare Ordinal "the first"
- the fraction "half"
- the range "3 to 5 files"
- the superlative "largest"

## Consequences

- If the Ordinal carried the claim, then the maintainers can revert the
  rewrite of the archetype "A1".
- The Lexer gains the shape of the Ordinal "4th" and gains the shape of the
  decimal "3.5". The Lexicon gains 6 Function Words.
- The unit "second" collides with the Ordinal "second", so a duration uses
  the word "minute". The maintainers decide the units in a future decision.
