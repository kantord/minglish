# 0003 — Copula: is/are only, present tense, no passive/progressive

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002; rewritten in minglish 2026-09-02)

## Context

Triage counts about 850 tokens of the word "be" in the corpus. The word
"be" makes a big hole in the Coverage. Every text needs the Copula. The
sentence "the file is old" needs the Copula.

The Paradigm of "be" contains "was". The Paradigm of "be" contains "been".
The Paradigm of "be" contains "being". The Surface Forms mark the Tense and
mark the aspect. If a Participle follows a Copula, then the sentence is not a plain
statement. The sentence is a Passive or is a Progressive.

The maintainers did not decide 4 questions:
- the Tense
- the aspect
- the Passive
- the Progressive

The Participles cause big ambiguities. A Reduced Relative is one ambiguity
of the Participles.

## Decision

The language enables 2 Copulas:
- "is"
- "are"

The Form Tag of "is" is "COPULA_SG". The Form Tag of "are" is "COPULA_PL".
A Copula is not a verb. A Copula takes a Complement. A Copula does not take
an object.

A Complement is an adjective or is a Noun Phrase. A Complement is not a
Participle. The language bans the Passive. The language bans the
Progressive. The sentence "the file is stored" is a Passive. The sentence
"the agent is running" is a Progressive.

The language does not enable 3 Surface Forms of "be":
- "am"
- "been"
- "being"

The decision "0032" enabled the phrase "must be".

The decision "0010" added "was". The decision "0010" added "were".

## Consequences

- The language can say the sentence "the queue is empty". The language can
  say the sentence "the parser is a program".
- The language cannot say a past state. The language cannot say an ongoing
  process. The maintainers decide the Tense in a future decision.
- The Grammar enforces the Complement. Triage does not see the Complement,
  because Triage checks the tokens.
