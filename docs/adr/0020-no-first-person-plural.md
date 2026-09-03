# 0020 — we/our/us banned: name the group

Date: 2026-09-01
Status: proposed (tentative)

## Context

The sweep of the decisions shows 2 big blockers:
- "we"
- "our"

The Gap "adr0001-05" shows the 2 blockers. The situation resolves the
Pronoun "I" and resolves the Pronoun "you". The word "we" carries 2
ambiguities, so the word "we" is not an Indexical Pronoun in the sense of
the decision "0002". The word "we" includes the reader or excludes the
reader. The extent of the group is unknown. The word "we" matches 4
groups:
- the authors
- the maintainers
- the project
- the contributors

The sentence "We commit the generated files" drops the doer, so the
sentence has the problem of a Passive. The maintainers can choose a fiat
meaning of the word "we". The meaning "we = the authors" is one example. A
fiat meaning changes the common meaning of the word "we" in technical
texts. The text does not announce the fiat meaning.

## Decision

The language bans 3 words:
- "we"
- "our"
- "us"

The advice names the group. The Linter offers 3 examples:
- the maintainers
- the team
- the project

The writer can use an Imperative or can use a generic statement. The Ban
puts the question "who acts" to the writer. The question is the purpose of
the product, because a policy must answer the question.

The maintainers changed the sentence "We commit the generated files" of
the decision "0001" into the sentence "The maintainers commit the
generated files".

If a good text needs the word "we", then the maintainers revisit the Ban.
The maintainers did not see the evidence, because the writer named the
group in every case.

## Consequences

- The Ban reduces the expressiveness of the text. The expressiveness is the
  5th criterion of the decision "0006". The maintainers accept the small
  expense, because the reference becomes precise.
- The word "maintainer" enters the Lexicon and becomes the main example of
  a replacement.
- The Gap "adr0001-05" had 2 halves:
  - "we"
  - "commit"

  The Ban removes the first half, so the Ban narrows the Gap to the verb
  "commit".
