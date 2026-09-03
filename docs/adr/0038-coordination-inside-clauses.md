# 0038 — Coordination inside a Conditional or a causal sentence

Date: 2026-09-04
Status: proposed (tentative). Extends ADR 0037. Revises ADR 0007 and ADR 0026.

## Context

The rewrite of the decisions met the word "and" inside a clause of a
Conditional. A writer needed one condition for 2 different subjects. A
Conditional kept the old Ban of a Coordination inside a clause. A causal
sentence kept the old Ban of a Coordination inside a clause.

The second shape of a Coordination got a fiat Sentence Shape from the
decision "0037", so a comma marks the new clause. The decision "0037" kept
the old Ban of a Coordination inside a clause, because a suffix of the
first shape repeats the comma of a causal sentence.

## Decision

The Ban ends. A Conditional allows a Coordination inside every clause. A
causal sentence allows a Coordination inside every clause. The first
shape keeps one subject, so a comma does not follow the first shape
inside a clause. The second shape of a Coordination needs a comma inside
a clause.

The construction of the Grammar moves the comma. A token follows the
comma, and the token decides the shape. The word "then" keeps the old
shape. The word "so" keeps the old shape. The word "because" keeps the
old shape. The word "but" keeps the new shape. The word "and" keeps the
new shape. The word "or" keeps the new shape. The Grammar picks one
shape, because the new rule does not add an ambiguity.

## Consequences

- A writer can give one condition for 2 different subjects.
- A writer can give one reason for 2 different subjects. A writer can give
  one result for 2 different subjects.
- The old sentence "a Conditional's clauses carry no 'and'/'or'" becomes
  false, so the Linter drops the advice.
