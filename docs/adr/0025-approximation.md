# 0025 — about / ~: approximate quantities

Date: 2026-09-01
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

The rewrite of the decision "0001" lost 2 approximations:
- "~5 forms per verb"
- "~10 rules"

The rewrite turned the 2 phrases into exact quantities. The approximation
is a claim, so the loss of the approximation is a Propositional Loss in
the decision "0012". The phrase "10 rules" is more precise than the
original phrase, so the rewrite added a false claim. Technical texts use
approximate quantities.

## Decision

The word "about" is a Function Word with the Form Tag "APPROX". The
Grammar allows the word "about" before digits and does not allow the word
in a different position. The decision gives 2 examples:
- "about 10 rules"
- "about 43 percent of the swaps"

The Form Tag "APPROX" has one position, so a writer cannot write the
preposition "about" in the phrase "about the file".

The symbol "~" is a second spelling of the word "about". The Lexer turns
the phrase "~10 rules" into the tokens of the phrase "about 10 rules", so
the 2 spellings have one meaning. The symbol "~" is the common notation in
the prose of the repository. The word "percent" exists, so the symbol "%"
is not a token. A second spelling of "percent" duplicates the word.

The Grammar does not allow the word "about" before the word "one", because
the word "one" marks an exact quantity. The symbol "~" must touch the
digits, so the Lexer rejects the string "~ 5".

## Consequences

- The language can say an approximate quantity, so a writer does not add
  a false precision.
- The Form Tag "APPROX" is the first token with 2 spellings. The
  maintainers accept the 2 spellings, because the rule is orthographic.
  The Lexer holds the rule, so the rule does not touch the Lexicon.
