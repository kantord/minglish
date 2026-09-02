# 0035 — become: a copula of change

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0003.

## Context

The rewrite of the decisions met 4 sentences of the archetype "A4". The
sentence "the ratio becomes honest" is one example. A rewrite used the Copula "is". The rewrite kept the result. The rewrite dropped the transition. The transition is a claim.

## Decision

The word "become" is a Copula. The word "become" marks a transition. The language enables 3 Surface
Forms:
- "becomes"
- "become"
- "became"

The Form Tag of "becomes" is "BECOME_SG". The Form Tag of "become" is
"BECOME_PL". The Form Tag of "became" is "BECOME_PAST". The word "become"
takes the Complement of a Copula. The Complement is an adjective or is a
Noun Phrase. The Complement is not a Participle. The sentence "the guard
becomes a floor" is one example. A quantified subject takes the word
"become".

## Consequences

- The language can say a transition. The transition is explicit.
- The fences of the decision "0003" stay. The Passive is a Ban. The
  Progressive is a Ban.
- If a sentence of the archetype "A4" is good, then the maintainers can revert the rewrite.
