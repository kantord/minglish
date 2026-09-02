# 0004 — Coordination: and/or enabled, but deferred

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002/0003; rewritten in minglish 2026-09-02)

## Context

Triage counts about 550 tokens of the word "and" in the corpus. Triage
counts about 77 tokens of the word "or" in the corpus. The 2 words make a
big hole in the Coverage.

The scope of a Coordination causes big ambiguities in English. The phrase
"old men and women" is ambiguous. The phrase "the sensor and the valve in
the cabinet" is ambiguous. The Grammar decides the scope. The token "and"
is not ambiguous. The token "or" is not ambiguous.

Triage counts about 82 tokens of the word "but". The word "but" is
different, because "but" marks a contrast. A contrast is not a claim.

## Decision

The language enables 2 conjunctions:
- "and"
- "or"

The Form Tag of "and" is "CONJ". The Form Tag of "or" is "CONJ". The
language has 2 conjunctions. The maintainers deferred the word "but". The
decision "0021" enabled the word "but".

The Grammar enforces 3 rules. A Coordination joins 2 phrases of one
Category. A Modifier does not cover a Coordination. Every Conjunct carries
the Modifiers of the Conjunct. The phrase "the old files and the old
reports" is correct. The phrase "the old files and reports" is not correct.
A Prepositional Phrase does not cover a Coordination. A Prepositional Phrase
attaches to one Conjunct.

## Consequences

- The language can say a compound statement. The language can say an
  alternative.
- The rules add words to a text. The words are the expense of a clear
  scope. The rules match the Ban of the Anaphoric Pronouns.
- The Grammar enforces the rules. Triage does not see the scope, because
  Triage checks the tokens.
