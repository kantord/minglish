# 0049 — "other": verified, and never bare

Date: 2026-09-04
Status: proposed (tentative). Extends ADR 0043 and ADR 0048.

## Context

The word "other" was a Ban of the queue. A writer says "the
mechanism names: the Ban and every other Rejection".

The decision "0049" resolves the word "other".

## Decision

The word "other" is an Idiomatic Structure of the second Conjunct.
The word "other" needs the second Conjunct. The Grammar rejects the
sentence "the mechanism deletes every other Rejection".

The bare word "other" causes a Scope Ambiguity. The Category
"Sentence Shape" has 4 members. The decision bans the bare word
"other". The word "every" opens one shape of the word "other". The
word "some" opens a second shape of the word "other". The word
"every" marks every member of the Category. The word "some" marks
one member of the Category. The word "every" needs a singular noun,
and the word "some" needs a plural noun.

The Grammar does not verify the meaning of the sentence. The word
"Ban" is a member of the Category "Rejection". The model of the
decision "0027" already names the member. The Linter reads the
model, and the Linter verifies the member.

A second shape does not need the model. A writer says "the
mechanism names: the report and every other report". The word
"report" is not a capitalized term. The 2 Conjuncts repeat the word
"report". The Linter verifies the shape without the model.

The Linter rejects a false member, and the Linter names the fact.

## Consequences

- A writer names an old member. A writer marks a new Category with
  the word "other".
- A writer marks the first Conjunct with the word "report". A writer
  marks the second Conjunct with the word "other".
- The decision needs the model of the decision "0027" for the first
  shape. The decision does not need the model for the second shape.
