# 0025 — about / ~: approximate quantities

Date: 2026-09-01
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

The rewrite of the decision "0001" turned the phrase "~5 forms per verb"
into an exact quantity. The rewrite turned the phrase "~10 rules" into an
exact quantity. The approximation is a claim. The phrase "10 rules" carries a precision. The original sentence refused the precision. Technical texts
use approximate quantities.

## Decision

The word "about" is a Function Word. The Form Tag of "about" is "APPROX".
The word "about" takes one position. The digits follow the word "about". The phrase "about 10 rules" is one example. The phrase "about 43
percent of the swaps" is one example. The preposition "about" is a Ban.
The phrase "about the file" is one example. The Form Tag "APPROX" has one
position.

The symbol "~" marks the approximation. The phrase "~10 rules" is one
example. The Lexer gives one token to the 2 spellings. The 2 spellings
have one meaning. The repository uses the symbol in the prose. The symbol
"%" is different. The word "percent" exists, so the symbol "%" duplicates
the word.

The word "about" does not take the word "one". The word "one" marks an
exact quantity. The Lexer rejects the string "~ 5".

## Consequences

- The language can say an approximate quantity. A quantity does not carry a false precision.
- The token "APPROX" has 2 spellings. The maintainers accept the 2
  spellings, because the 2 spellings are a rule of the Lexer. The rule does
  not touch the Lexicon.
