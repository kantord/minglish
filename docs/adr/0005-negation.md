# 0005 — Negation: not with do-support, fixed predicate scope

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0004)

## Context

*not* is ×204 in triage; prohibition ("do not delete the file") is core to
instructional text. English verbal negation requires do-support, and *do* is
an auxiliary — a category v0 otherwise avoids. Constituent negation
("not all users", "not old") introduces scope ambiguity.

## Decision

- Enabled: *not* (NEG), *do* (NEG_AUX_BASE), *does* (NEG_AUX_3SG).
- *do/does* are negation carriers only: intended grammar permits them solely
  in `do|does + not + VERB_BASE` (and bare `do not + VERB_BASE` as a
  prohibition/imperative). Emphatic *do* ("the parser does accept it") and
  interrogative *do* are out.
- Copular negation is `is/are + not` directly; no do-support.
- Scope rule, fixed: *not* negates the clause's main predicate, nothing else.
  Constituent negation is banned — no "not all users", no "a not old file".
- Not enabled (*did* later added by ADR 0010): *doesn't/
  don't* (no contractions in minglish orthography).

## Consequences

- Negated statements and prohibitions become expressible in natural English.
- One auxiliary enters the language, but fenced to a single construction.
- The fence and the scope rule are grammar-tier; token-level triage counts
  any do/does/not as OK.
