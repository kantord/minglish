# 0026 — so / because: two causal connectives, two information structures

Date: 2026-09-01
Status: proposed (tentative)

## Context

Rewriting ADR 0001 dropped "hence this ADR": the language had no causal
connective, and ADR 0012 counts causal structure as propositional (tier 1).
Frequencies: *so* ×11 and *because* ×7 in the ADRs; *because* ×17, *since*
×15 (mostly temporal), *so* ×60 (mixed senses) in UD-EWT.

English offers cause-first ("A, so B" / "because A, B") and effect-first
("B because A") orders. They are not one meaning with two forms: they
differ in information structure — which clause is given and which is new
— and given-before-new is a comprehension aid (ADR 0006, and the standard
advice in the style literature). Effect-first is also where attachment
ambiguity lives ("B because A and C").

Fixing the conditional to one order (ADR 0007) is not a precedent against
two orders here: a consequent-first conditional makes the reader hold an
unevaluable hypothetical open; in a causal both clauses are asserted, so
effect-first costs no suspended evaluation.

## Decision

- Two constructions, one fiat meaning each, comma mandatory at the seam:
  - **Result**: `<clause> , so <clause>` — cause is given, result is new.
  - **Reason**: `<clause> , because <clause>` — result is given, reason is
    new.
- Clauses inside carry no coordination (as in the conditional), so the
  scope of the connective is fixed by construction.
- Neither connective can start a sentence (first-token telegraph, ADR
  0014). Cross-sentence causation is written by merging the two sentences
  or with a causal verb (*explain*, and topic-first "the expense explains
  the decision").
- *so* has only the result sense: degree ("so big") and purpose ("so
  that") are unwritable by construction. *since*, *hence*, *therefore*,
  *thus*, *as* are banned with advice naming the two shapes.
- Deferred: imperatives and conditionals inside a causal; causal chains
  (A, so B, so C); purpose ("in order to").

## Consequences

- The ADR 0001 debt is paid: "the expense of a reversal grows with the
  seed, so the maintainers record the decision".
- The linter names three repairs: missing comma, fronted *because*,
  sentence-initial *so*.
- Which connective is right depends on the previous sentence — a check
  only a document-level lint can make (scripts/lint-file.py, topic
  continuity).
