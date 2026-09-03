# 0002 — Pronouns: third person banned, indexicals allowed

Date: 2026-08-31
Status: proposed (tentative — revisit once real corpus experience exists; rewritten in minglish 2026-09-02, see docs/dogfood-adr-0002.md)

## Context

Triage counts the unknown tokens of the corpus. About 2200 unknown tokens
are Pronouns. The Pronouns make a big hole in the Coverage.

An Anaphoric Pronoun refers to a noun of a prior sentence. The reader must
find the noun. The sentence "it failed" needs a Discourse Layer. The file
`docs/research/cnl-design-findings.md` describes the results of the
research. If a tool splits a sentence into 2 sentences, then the ambiguity
reappears in the Pronouns of the sentences.

An Indexical Pronoun refers to the writer or refers to the reader. An
Indexical Pronoun is not an Anaphoric Pronoun. An Indexical Pronoun does not
cause a Reference Ambiguity. Instructions use Indexical Pronouns.

## Decision

The language bans every Anaphoric Pronoun:
- "it"
- "its"
- "they"
- "them"
- "their"
- "he"
- "him"
- "his"
- "she"
- "her"
- "this"
- "that"
- "these"
- "those"

The writer repeats the noun.

The language allows 4 Indexical Pronouns:
- "I"
- "you"
- "my"
- "your"

The Form Tag of "I" is "PRON_1SG". The Form Tag of "you" is "PRON_2". The
Form Tag of "my" is "POSS_1SG". The Form Tag of "your" is "POSS_2".

The language does not have a Discourse Layer. The maintainers do not plan a
Discourse Layer for the initial version.

## Consequences

- The design of the language removes every Reference Ambiguity. The
  language does not have a Discourse Layer, so the maintainers do not build
  a Discourse Layer.
- The prose repeats the nouns. The repetition is an acceptable expense. The
  project prefers the clarity of the prose to the naturalness of the prose.
- The Lexicon contains a Ban for every Anaphoric Pronoun. The Linter
  explains the Ban to the writer. The Linter says "repeat the noun you
  mean".
