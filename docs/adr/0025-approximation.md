# 0025 — about / ~: approximate counts

Date: 2026-09-01
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

The ADR 0001 rewrite turned "~5 forms per verb" and "~10 rules" into exact
counts. The approximation is propositional (ADR 0012 tier 1): "10 rules"
claims a precision the source refused. Approximate counts are ordinary in
technical prose.

## Decision

- *about* is a closed-class marker (APPROX) allowed **only immediately
  before a digit count**: "about 10 rules", "about 43 percent of the
  swaps". The preposition sense ("about the file") is unwritable by
  construction — APPROX has no other position.
- The symbol form **~** attached to the digits ("~10 rules") lexes to the
  same tokens. Two spellings, one token, one meaning; the symbol is the
  common notation in the repository's own prose. (Contrast *%*: not a
  token, because *percent* is a word and *%* would be a second one.)
- Not before *one* (exactly-one is not approximable) and not detached
  ("~ 5" is rejected).

## Consequences

- Approximate counts are expressible without false precision.
- First token with two surface spellings. Accepted because the mapping is
  orthographic, not lexical, and the lexer owns it.
