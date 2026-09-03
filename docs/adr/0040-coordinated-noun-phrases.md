# 0040 — Coordinated noun phrases: the Ban stays

Date: 2026-09-04
Status: proposed (tentative). Revises ADR 0004; closes the deferred rules.

## Context

The decision "0004" deferred 3 rules for a future Grammar. The second
rule keeps a Modifier inside one Conjunct, so a writer repeats the
Modifier for every Conjunct. The second rule bans the phrase "the old
files and reports". The second rule allows the phrase "the old files and
the old reports". The second rule removes the Scope Ambiguity. Every
Conjunct repeats the Modifier.

The rewrite of the decisions needed a Coordination of 2 Noun Phrases.

## Decision

The Ban stays. The maintainers built 3 versions of the rule. Every
version conflicts with the Grammar. The Grammar has one state, and the
state covers a Noun Phrase and covers a different Noun Phrase. The rule
needs a different Noun Phrase, and the different Noun Phrase covers the
state.

The Noun Phrase of the Grammar has 12 positions.

## Consequences

- A writer repeats the verb for 2 Noun Phrases, and the writer avoids a
  Coordination of 2 Noun Phrases. "the mechanism stores a word and stores
  a message" is one example.
- The maintainers keep the first rule of the decision "0004". The first
  rule limits a Coordination to phrases of one Category.
- The maintainers drop the second rule and drop the third rule of the
  decision "0004". The Grammar cannot enforce the 2 rules.
