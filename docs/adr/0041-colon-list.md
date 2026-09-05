# 0041 — A colon opens an inline list of 2 objects

Date: 2026-09-04
Status: proposed (tentative). Revises ADR 0040.

## Context

The decision "0040" kept the Ban. A writer drops the second verb of
the predicate. The bare phrase drops the second verb, so the Grammar
cannot parse the bare phrase.

The maintainer suggested a colon. A writer can say the new sentence
"the mechanism stores: a word and a message".

## Decision

The colon opens the list. The verb takes the colon, and 2 Noun
Phrases follow the colon. Every item repeats the Modifier of the
decision "0004".

The colon marks the position of the second verb. The register of the
sentence is technical, and the register is not spoken. The colon
matches a common convention of a technical document.

The colon marks a new position for the Grammar. The new position
does not cover the state of the decision "0040". The phrase of the
verb allows the shape. The bare predicate allows the shape, so an
Imperative allows the shape, and a Prohibition allows the shape.

## Consequences

- A writer can name 2 things of one verb in one sentence. The writer
  keeps a clear scope. A writer does not repeat the verb "stores".
- The decision "0004" bans a chain of 3 Conjuncts. A writer uses the
  Enumeration for 3 items.
- The Linter drops the advice for the shape of the colon.
