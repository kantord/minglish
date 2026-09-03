# 0002 — Pronouns: third person banned, indexicals allowed

Date: 2026-08-31
Status: proposed (tentative — revisit once real corpus experience exists; rewritten in minglish 2026-09-02, see docs/dogfood-adr-0002.md)

## Context

Triage counted the unknown tokens of the corpus "UD-EWT". The file
"docs/triage-report.md" records the result. About 2200 unknown tokens are
Pronouns, so the Pronouns are a big Gap in the Coverage. The Gap of one
Closed Class is bigger than the Gap of the Pronouns.

Every Pronoun of the third person is an Anaphoric Pronoun. An Anaphoric
Pronoun refers to a noun of a prior sentence, so the reader must find the
noun. A Discourse Layer resolves the referent of "it", so the sentence "it
failed" needs a Discourse Layer. If a tool splits a long sentence into 2
sentences, then the second sentence refers to the first sentence. The
ambiguity of the long sentence reappears in the reference. The file
"docs/research/cnl-design-findings.md" describes the problem.

An Indexical Pronoun refers to the speaker or refers to the hearer. The
reference is not anaphoric, so an Indexical Pronoun does not cause a
Reference Ambiguity. Instructions use Indexical Pronouns.

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

If a writer needs an Anaphoric Pronoun, then the writer repeats the noun.

The language allows 4 Indexical Pronouns:
- "I"
- "you"
- "my"
- "your"

The 4 Indexical Pronouns have 4 Form Tags:
- "PRON_1SG"
- "PRON_2"
- "POSS_1SG"
- "POSS_2"

The maintainers did not build a Discourse Layer and do not plan a Discourse
Layer for the initial version.

## Consequences

- The construction of the language removes every Reference Ambiguity, so
  the maintainers do not need a Discourse Layer.
- The prose repeats the nouns, so the prose is repetitive. The project
  accepts the repetition, because the project prefers the clarity over the
  naturalness.
- The current Lexicon does not contain the Anaphoric Pronouns. The Linter
  finds an unknown word and does not explain the Ban. A writer needs the
  message "repeat the noun you mean". The message needs a new mechanism in a
  future validator, so the maintainers deferred the message.
