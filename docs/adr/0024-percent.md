# 0024 — percent: a share of a named set

Date: 2026-09-01
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

The rewrite of the decision "0001" met the sentence "43% of its swaps had
no benefit". The share is a claim. If a translation drops the share, then
the translation changes the claim. If a translation invents a quantity, then
the translation changes the claim. The phrase "43 of 100 swaps" invents a quantity. The reports of the metrics use shares. The decision "0022" deferred
the units. A share is not a unit. A share is a quantifier. A share needs a
set.

## Decision

The word "percent" is a Function Word. The Form Tag of "percent" is
"PERCENT". The construction has 4 parts:
- the digits
- the word "percent"
- the word "of"
- a plural Noun Phrase

The sentence "43 percent of the swaps did not reduce the ambiguity" is one
example. The construction takes a plural verb. The construction takes
every position of a plural Noun Phrase.

The Noun Preposition is mandatory. The sentence names the set. The
sentence "43 percent did not reduce the ambiguity" is a Ban. The reader
does not know the set. The Ban matches the decision "0002".

The construction uses the digits. The phrase "one percent" is a Ban. The
phrase "0 percent" is a Ban. The word "no" marks the meaning of "0
percent". The phrase "one percent" waits for the evidence.

The symbol "%" is not a token. The writer writes the word.

The maintainers deferred 3 questions. One question is the share of a
singular noun. The phrase "50 percent of the file" is one example. One question puts a share into the Complement. The sentence "the rate is 43
percent" is one example. One question is the decimals.

## Consequences

- The language can say a share. A share does not invent a number.
- The language has 2 words for a quantity. The word "one" is one word. The
  word "percent" is one word.
- The construction does not add an ambiguity. The Form Tag "PERCENT" is a
  distinct token.
