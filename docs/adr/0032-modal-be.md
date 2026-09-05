# 0032 — "be" after a modal

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0009; ADR 0003 keeps every other fence.

## Context

The rewrite of the decisions found 7 sentences of the archetype "A16". The
sentence "a hyphenated word must be transparent" is one example. A rewrite
lost the force of the modal, because the decision "0009" did not allow the
phrase "must be". A Conditional replaced the obligation, and a verb replaced
the possibility. The obligation is a claim, and the possibility is a claim.
The decision "0012" bans the loss of a claim.

## Decision

The language adds the word "be" in one position. The word "be" follows a
modal. The language allows 4 phrases:
- "must be"
- "must not be"
- "can be"
- "cannot be"

The word "be" takes the Complement of a Copula. The Complement is an
adjective or is a Noun Phrase. The Complement is not a Participle. The
decision gives 2 examples:
- "a term can be a verb"
- "the intro cannot be a question"

The word "be" is not an Auxiliary. The word "be" is not an infinitive.
The word "be" does not open an Imperative. The fences of the decision "0003" stay.
The fences of the decision "0010" stay.

## Consequences

- If a sentence of the archetype "A16" is good, then the maintainers can
  revert the rewrite.
- The Grammar gains 3 predicates, because the language has 3 modals. A
  quantified subject takes "must be" and takes "can be". The word "no"
  takes "can be".
- If a modal does not precede the word "be", then the Linter names the
  position of the word "be".
