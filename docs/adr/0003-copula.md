# 0003 — Copula: is/are only, present tense, no passive/progressive

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002)

## Context

*be* is the single largest missing lemma in triage (~850 AUX tokens in
UD-EWT). Predication ("the file is old") is unavoidable for general text.
But the full *be* paradigm drags in tense (*was/were*), aspect
(*been/being*), and — via *be* + participle — the passive and progressive
constructions, all grammar-tier decisions not yet made. The reduced-relative
and participle ambiguities are the nastiest residual class our research
identified.

## Decision

- Enabled: *is* (COPULA_SG) and *are* (COPULA_PL) as a dedicated fiat
  category — not VERB_*, because the copula's slot is unique (takes ADJ or
  NOUN-phrase complements, no objects).
- Intended restriction (enforceable only once a grammar tier exists): copula
  complements are ADJ or noun phrases only. *is* + participle ("is stored",
  "is running") is out of v0 — no passive, no progressive.
- Not enabled: *be, am, been, being*. (*was/were* later added by ADR 0010.)

## Consequences

- Property statements and class membership become expressible ("the queue is
  empty", "the parser is a program").
- No way to express past states or ongoing processes yet; revisit with the
  tense question.
- The complement restriction is documentation-only until the grammar tier;
  triage counts any *is/are* token as OK because token-level checking cannot
  see the construction.
