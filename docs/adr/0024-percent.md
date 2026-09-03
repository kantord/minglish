# 0024 — percent: a share of a named set

Date: 2026-09-01
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

The rewrite of the decision "0001" found the sentence "43% of its swaps had
no benefit". The share is a claim, so the loss of the share is a
Propositional Loss. The phrase "43 of 100 swaps" invents a number, so the
phrase changes the claim. The decision "0012" bans every Propositional
Loss. The reports of the repository describe every metric with a share of
one shape. The decision "0022" deferred the units. A share is not a unit,
because a share is a quantifier of a set.

## Decision

The word "percent" is a Function Word with the Form Tag "PERCENT". A share
has 4 parts:
- the digits
- the word "percent"
- the word "of"
- a plural Noun Phrase

The sentence "43 percent of the swaps did not reduce the ambiguity" is one
example. A share is a plural Noun Phrase, so a share takes a plural verb.
The writer can use a share in every position of a plural Noun Phrase.

The Noun Preposition is mandatory, because the sentence must name the set.
The sentence "43 percent did not reduce the ambiguity" is a Ban, because the
reader cannot find the set in the sentence. A bare share refers to a set of
a prior sentence, so the bare share is anaphoric. The decision "0002" bans
every anaphoric reference.

A share uses digits, because the decision "0022" chose the digits. The word
"no" replaces the phrase "0 percent", so the phrase "0 percent" is a Ban.
The phrase "one percent" is a Ban, because the phrase is rare. If a real
text needs the phrase "one percent", then the maintainers can remove the
Ban.

The symbol "%" is not a token, so the writer writes the word "percent".

The maintainers deferred 3 questions. The first question covers a share of
a singular noun. The phrase "50 percent of the file" is one example. The
second question covers a share in the position of the Complement. The
sentence "the rate is 43 percent" is one example. The third question covers
the decimals.

## Consequences

- The language can say a share, so the writer does not invent a number.
- The word "one" was the first Function Word for a quantity, so the word
  "percent" is the second Function Word for a quantity.
- The class "NUM_PL" of the Lexer gains one shape. The shape does not add an
  ambiguity, because the word "percent" has a distinct Form Tag.
