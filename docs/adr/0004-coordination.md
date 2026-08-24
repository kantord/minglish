# 0004 — Coordination: and/or enabled, but deferred

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002/0003)

## Context

*and* (×550) and *or* (×77) are top missing closed-class lemmas in triage.
Coordination scope is among the worst structural ambiguities in English
("old men and women"; "the sensor and the valve in the cabinet"), but all of
that is grammar-tier: the tokens themselves are indispensable and
unambiguous. *but* (×82) differs — adversative contrast is discourse-level
meaning, the first word whose semantics exceeds propositional content.

## Decision

- Enabled: *and*, *or* as category CONJ.
- Deferred: *but* and all other conjunctions. (*but* later enabled by ADR 0021.)
- Intended grammar rules, recorded now, enforceable only later:
  1. Coordination joins identical categories only.
  2. Modifiers never distribute over a coordination — each conjunct carries
     its own modifiers ("the old files and the old reports", never
     "the old files and reports").
  3. No shared-PP scope over conjuncts; attach per conjunct explicitly.

## Consequences

- Compound statements and alternatives become expressible.
- Texts get longer under rule 2 — the price of scope unambiguity, consistent
  with the repeat-the-noun pronoun policy.
- Token-level triage counts every and/or as OK; the scope rules are
  unmeasurable until a grammar tier exists.
