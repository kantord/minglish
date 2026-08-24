# 0013 — Genericity: bare plurals, generic-only by fiat

Date: 2026-09-01
Status: proposed (tentative)

## Context

Dogfooding ADR 0001 (docs/dogfood-adr-0001.md) showed mandatory determiners
deleted English's genericity device: characterizing claims ("humans miss
checks") could only be rendered as false definites ("the people…") or
ambiguous indefinites ("a person…") — propositional changes, caught by
human review, invisible to every metric. Genericity was already on the
research list of CNL semantic failure modes.

## Decision

- A **bare plural** (no determiner; optional adjective; optional of-PP) is
  legal and has exactly one reading by fiat: a **characterizing/generic
  statement about the kind** ("machines find collisions", "agents retry
  requests"). The English existential reading of bare plurals ("dogs are
  barking") is banned; existentials will use *some* when introduced.
- Generic ≠ universal, deliberately: "birds fly" tolerates exceptions;
  *every* (future ADR) will not. The two stay distinct forms with distinct
  meanings.
- Bare **singular** remains banned. Singular generics rephrase to plural.
- The mandatory-determiner rule gets this one carve-out: its purpose was
  ambiguity prevention, and the fiat single reading preserves that purpose.
- Positions: subject, object, PP object, copular complement. Not inside
  of-PPs for now (bound; revisit with evidence).

## Consequences

- Characterizing prose (policy text, ADRs, documentation) becomes
  expressible without meaning change; two retracted dogfood pairs return in
  faithful generic form.
- Zero token cost — the generic is *shorter* than the false definite it
  replaces (ADR 0006 §4).
- Writers must learn one fiat rule: bare plural = generic, never "some".
