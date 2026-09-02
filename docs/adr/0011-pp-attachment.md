# 0011 — PP attachment decided by the preposition's lexical class

Date: 2026-09-01
Status: proposed (tentative)

## Context

The sentence "the system stores the report in the database" has 2 Parses
in English. The Prepositional Phrase attaches to the verb or attaches to
the noun. The attachment of a Prepositional Phrase is a classic
ambiguity of the structure. The Grammar shows an error. A rule must decide
the attachment. The decision "0006" puts the rule into the Sentence Shapes.

## Decision

The preposition decides the attachment. The language has 2 kinds of the
prepositions:
- the Noun Preposition
- the Verb Preposition

The word "of" is the Noun Preposition. The Form Tag of "of" is "PREP_N".
The Noun Preposition attaches to the prior noun. The phrase "a copy of the
report" is one example. The reader expects the attachment to the noun, so
the rule matches the reader.

The Form Tag "PREP_V" covers 7 Verb Prepositions:
- "in"
- "from"
- "to"
- "with"
- "on"
- "at"
- "for"

A Verb Preposition attaches to the verb of the clause. If a clause has 2
Verb Prepositions, then the order is ambiguous. The language bans 2 Verb
Prepositions in one clause. The language bans a chain of the word "of".
The phrase "the copy of the report of the user" is one example. The tokens
decide the Parse.

## Consequences

- If a writer puts a Verb Preposition on a noun, then the writer
  restructures the phrase with "of". The writer changes the phrase "the
  input from the user" into the phrase "the input of the user". The rule
  turns a silent ambiguity into an explicit rewrite.
- The maintainers rejected 3 alternatives. One alternative puts every
  preposition on the verb. The alternative breaks the phrase "a copy of the
  report". One alternative puts a preposition on the prior word. The
  alternative conflicts with the expectation of the reader. One alternative
  decides the attachment with a score. The alternative violates the
  decision "0006".
- The maintainers updated the corpus. The maintainers applied the rule, so the
  maintainers restructured 2 sentences.
