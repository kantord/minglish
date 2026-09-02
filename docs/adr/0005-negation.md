# 0005 — Negation: not with do-support, fixed predicate scope

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0004; rewritten in minglish 2026-09-02)

## Context

Triage counts about 204 tokens of the word "not" in the corpus. Instructions
need Prohibitions. The sentence "do not delete the file" is a Prohibition.

A Negation of a verb needs the word "do" in English. The word "do" is an
Auxiliary. The language avoids Auxiliaries. The word "not" can change one
Constituent. The phrase "not all users" is one example. The phrase "not old"
is one example. A Negation of a Constituent causes a Scope Ambiguity.

## Decision

The language enables 3 Function Words:
- "not"
- "do"
- "does"

The Form Tag of "not" is "NEG". The Form Tag of "do" is "NEG_AUX_BASE". The
Form Tag of "does" is "NEG_AUX_3SG".

The word "do" carries a Negation. The word "does" carries a Negation. The
Grammar allows the Auxiliary in one Sentence Shape. The Sentence Shape is
the phrase "does not delete". A Prohibition uses the phrase "do not". The
language bans the emphatic Auxiliary. The sentence "the parser does accept
the file" is one example. The language bans the Auxiliary in a question.

A Negation of a Copula uses "is not" or uses "are not". A Negation of a
Copula does not need an Auxiliary.

The Grammar fixes the scope of "not". The word "not" changes the truth of
the main verb. The word "not" does not change a Constituent. The language
bans a Negation of a Constituent. The phrase "not all users" is one example.
The phrase "a not old file" is one example.

The decision did not enable "did". The decision "0010" added "did". The
language bans every contraction. The word "doesn't" is a contraction. The
word "don't" is a contraction.

## Consequences

- The language can say a Negation. The language can say a Prohibition. The
  2 Sentence Shapes are natural sentences of English.
- One Auxiliary enters the language. The Grammar limits the Auxiliary to one
  Sentence Shape.
- The Grammar enforces the rule of the scope. The Grammar enforces the rule
  of the Auxiliary. Triage does not see the rules, because Triage checks the
  tokens.
