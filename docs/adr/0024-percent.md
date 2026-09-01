# 0024 — percent: a share of a named set

Date: 2026-09-01
Status: proposed (tentative). Extends ADR 0022 (digits).

## Context

Rewriting ADR 0001 in minglish hit "43% of its swaps had no benefit". The
proportion is propositional content (ADR 0012 tier 1): dropping it or
turning it into an invented count ("43 of 100 swaps") changes the claim.
Every metrics discussion in this repository states shares the same way.
ADR 0022 deferred units; a share is not a unit, it is a quantifier over a
named set.

## Decision

- *percent* is a closed-class marker (PERCENT). The construction is
  **digits + percent + of + plural NP**: "43 percent of the swaps did not
  reduce the ambiguity". Plural agreement; usable wherever a plural NP is.
- The of-phrase is mandatory: the set must be named in the sentence. Bare
  "43 percent did not reduce the ambiguity" is unwritable — the reference
  to *what* is the same discourse dependence ADR 0002 bans.
- Digits only, per ADR 0022: "one percent" and "0 percent" are out (the
  latter is *no*; the former is rare enough to wait for evidence).
- The symbol *%* is not a token; write the word.
- Deferred: percent of a singular mass ("50 percent of the file"),
  percentages as predicates ("the rate is 43 percent"), decimals.

## Consequences

- Shares are expressible with no invented numbers.
- Second closed-class quantity word after *one*; the lexer class NUM_PL
  gains one construction and no ambiguity (PERCENT is a distinct
  terminal).
