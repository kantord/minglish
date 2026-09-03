# 0035 — become: a copula of change

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0003.

## Context

The rewrite of the decisions found 4 sentences of the archetype "A4". The
sentence "the ratio becomes honest" is one example. The rewrite used the
Copula "is". The Copula "is" kept the result but dropped the transition. The
transition is a claim.

## Decision

The word "become" is a Copula and marks a transition. The language adds 3
Surface Forms:
- "becomes"
- "become"
- "became"

The Form Tag of "becomes" is "BECOME_SG". The Form Tag of "become" is
"BECOME_PL", and the Form Tag of "became" is "BECOME_PAST". The word "become"
takes the Complement of a Copula, so the Complement is not a Participle. The
Complement is an adjective or is a Noun Phrase. The sentence "the guard
becomes a floor" is one example. A plural subject takes the Surface Form
"become". The sentence "some guards become floors" is one example.

## Consequences

- The language can say an explicit transition.
- The Bans of the decision "0003" stay. The language bans the Passive and
  bans the Progressive.
- If a sentence of the archetype "A4" is good, then the maintainers can
  revert the rewrite.
