# 0007 — Conditionals: mandatory "if <clause>, then <clause>" template

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0005)

## Context

*if* is ×87 in triage and conditionals are the heart of instructional text
("if the test fails, the agent retries the request"). The original design
sketch already listed *conditional* among the fixed sentence templates. A
bare two-clause conditional invites clause-boundary and attachment
ambiguity; English also permits consequent-first order ("the agent retries
if…"), which forces the reader to hold the consequent open until the
condition arrives.

## Decision

- Enabled: *if* (SCONJ_COND), *then* (THEN).
- One template, fixed order: `if <clause> , then <clause>`.
  - Condition first: gives context before the consequent (ADR 0006 §3), and
    the fronted material is bounded by one clause (a load bound, ADR 0006 §1).
  - Comma and *then* both mandatory: the consequent boundary is explicit,
    killing garden paths at the clause seam.
- Not enabled: consequent-first order, *unless*, *when/whenever*, *else*,
  nested conditionals inside the condition clause.

## Consequences

- Rules and reactive instructions become expressible in one unambiguous
  shape.
- Writers must restructure "X if Y" into "if Y, then X" — mechanical, and a
  future validator can suggest it.
- Token-level triage counts *if/then* tokens as OK; template enforcement is
  grammar-tier.
