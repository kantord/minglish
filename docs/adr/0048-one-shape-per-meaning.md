# 0048 — One shape per meaning; same-verb Coordination banned

Date: 2026-09-04
Status: proposed (tentative). Extends ADR 0041.

## Context

A writer can build 2 shapes for one meaning. A writer says "the
mechanism stores a word and stores a message". A writer says "the
mechanism stores: a word and a message".

The decision "0041" kept the first shape, and the decision "0041"
did not ban the first shape. The language did not have a rule for
the case.

## Decision

The decision "0048" names a rule. One meaning has one shape. An
exception needs the evidence. The evidence needs a measurement, and
the measurement compares the shapes.

No evidence exists for the first shape. The decision bans the first
shape.

The Grammar cannot ban the first shape, because the Grammar cannot
compare 2 words. The Grammar accepts the sentence. The Linter
compares 2 words. A writer repeats one verb in a Coordination, so
the writer builds the first shape. The Linter rejects the first
shape, and the Linter names the second shape.

## Consequences

- A writer keeps one shape of the Coordination.
- A future decision can add an exception, and the decision needs the
  evidence.
- The decision does not check every old decision of the language.
  The decision covers the first shape.
