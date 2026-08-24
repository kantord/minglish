# 0011 — PP attachment decided by the preposition's lexical class

Date: 2026-09-01
Status: proposed (tentative)

## Context

"the system stores the report in the database" — verb or noun attachment?
PP attachment is the canonical structural ambiguity; an LR grammar surfaces
it as a conflict and something must legislate it. Per ADR 0006's enforcement
hierarchy, the fix belongs in the set of valid structures, not in scoring.

## Decision

Attachment is a lexical property of the preposition:

- **PREP_N** — *of* only. Attaches to the immediately preceding noun
  ("a copy of the report"). Of-PPs in English are near-universally nominal,
  so this matches reader priors (ADR 0006 §2).
- **PREP_V** — all other prepositions (*in, from, to, with, on, at, for*).
  Attach to the clause's verb, always.
- Bound: at most **one PREP_V PP per clause** in v0 — multiple-PP ordering
  ambiguity is excluded until priced; PREP_N PPs are not multiplied either
  (no "of the X of the Y" chains yet).

The parse is thereby deterministic from the token stream alone.

## Consequences

- Noun-attaching intent with a non-*of* preposition must be rephrased,
  usually via *of* ("the input from the user" → "the input of the user") —
  the system converts a silent ambiguity into an explicit rewrite.
- Alternatives rejected: verb-attachment-for-everything (makes "a copy of
  the report" unreadable), nearest-attachment (fights reader priors on
  instrument/location PPs), score-based disambiguation (violates the
  enforcement hierarchy).
- Corpus and pairs updated to comply; two sentences were rephrased, which is
  the mechanism working, not a regression.
