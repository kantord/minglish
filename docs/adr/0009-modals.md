# 0009 — Modals: must and can only; may banned

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0005, 0007)

## Context

Normative language (obligation, prohibition, permission, ability) is the
core of policy- and instruction-type text. English modals are also among its
most ambiguous words: *may* is permission or epistemic possibility ("the
agent may retry"), *can* is ability or permission, *should* opens a deontic
strength scale, *will/would* entangle tense. Per ADR 0008, an ambiguous
word whose senses have no findable one-word substitute is a ban candidate,
not a redirect candidate. Modals are dense (one token carries the whole
normative force) — exactly the precise-and-cheap density ADR 0006 §4 wants.

## Decision

- Enabled: *must* (MODAL_MUST) — obligation; *must not* — prohibition
  (composed with NEG). *can* (MODAL_CAN) — ability and permission, merged by
  fiat ("is able/allowed to"). *cannot* (MODAL_CAN_NEG) — single token, as
  standard orthography writes the modal negation.
- Banned: *may* (permission → *can*; possibility → rephrase, e.g. a
  conditional or "sometimes"). Deferred: *should* (deontic scale),
  *will/would* (tense), *shall*, *might*, *could*.
- Intended grammar: `MODAL (+ not) + VERB_BASE`. Modals never stack and do
  not combine with the copula in v0 (no "must be old" yet).

## Consequences

- Obligation, prohibition, permission, and ability are expressible in one
  token each, with one meaning each.
- Epistemic possibility has no direct form — an accepted gap; revisit with
  evidence if the corpus needs it.
- UD tags modals AUX; triage maps AUX→MODAL_* as OK at token level.
