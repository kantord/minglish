# 0032 — "be" after a modal

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0009; ADR 0003 keeps every other fence.

## Context

The rewrite of the decisions met 7 sentences of the archetype "A16". The
sentence "a hyphenated word must be transparent" is one example. The decision "0009" did not allow the phrase "must be". A rewrite lost the force of the modal. A Conditional replaced the obligation. A verb
replaced the possibility. The obligation is a claim. The possibility is a
claim. The decision "0012" bans the loss of a claim.

## Decision

The word "be" enters the language. The word "be" has one position. The
position follows a modal. The language allows 4 phrases:
- "must be"
- "must not be"
- "can be"
- "cannot be"

The word "be" takes the Complement of a Copula. The Complement is an
adjective or is a Noun Phrase. The Complement is not a Participle. The
sentence "a term can be a verb" is one example. The sentence "the intro
cannot be a question" is one example.

The word "be" does not appear in a different position. The word "be" is
not an Auxiliary. The word "be" is not an infinitive. The word "be" does
not open an Imperative. The fences of the decision "0003" stay. The fences of the decision "0010" stay.

## Consequences

- If a sentence of the archetype "A16" is good, then the maintainers can revert the rewrite.
- The Grammar gains 3 predicates. A quantified subject takes "must be" and
  takes "can be". The word "no" takes "can be".
- The Linter names the position. If a modal does not precede the word "be", then the Linter shows an advice.
