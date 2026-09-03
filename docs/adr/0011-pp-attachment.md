# 0011 — PP attachment decided by the preposition's lexical class

Date: 2026-09-01
Status: proposed (tentative)

## Context

The sentence "the system stores the report in the database" has 2 Parses
in English. The Prepositional Phrase "in the database" attaches to the verb
"stores" or attaches to the noun "report". The attachment of a
Prepositional Phrase is a classic ambiguity of English, so the 2 Parses
conflict in the Grammar. A rule must decide the attachment. The decision
"0006" ranks the Sentence Shapes before a score. The remedy belongs to the
Sentence Shapes and does not belong to a score.

## Decision

The attachment is a property of the preposition. The language has 2 kinds
of a preposition:
- the Noun Preposition
- the Verb Preposition

The word "of" is the Noun Preposition and has the Form Tag "PREP_N". The
Noun Preposition attaches to the prior noun, so the phrase "of the report"
describes the copy in the phrase "a copy of the report". A writer of English
puts "of" on a noun, so a reader expects the attachment to the noun. The
rule follows the expectation of the reader, because the decision "0006"
prefers the common construction.

If a preposition is not the word "of", then the preposition is a Verb
Preposition. A Verb Preposition attaches to the verb of the clause. The Form
Tag "PREP_V" covers 7 Verb Prepositions:
- "in"
- "from"
- "to"
- "with"
- "on"
- "at"
- "for"

If a clause has 2 Verb Prepositions, then the order of the 2 Prepositional
Phrases is ambiguous. The maintainers did not measure the expense of the
ambiguity, so the first version does not allow 2 Verb Prepositions in one
clause. The bound is provisional. The first version does not allow a chain
of 2 Noun Prepositions, so the phrase "the copy of the report of the user"
is a Rejection. A future decision can allow the chain. The tokens of a
sentence decide the Parse, so the Parse is deterministic.

## Consequences

- If a writer puts a Verb Preposition on a noun, then the Grammar links the
  phrase to the verb. The writer must restructure the phrase. The common
  rewrite uses "of", so the phrase "the input from the user" becomes the
  phrase "the input of the user". The rule turns a silent ambiguity into an
  explicit rewrite.
- The maintainers rejected 3 alternatives. The first alternative puts every
  preposition on the verb, so the phrase "a copy of the report" becomes
  unreadable. A reader links a place to the verb and links a tool to the
  verb. The second alternative puts every preposition on the prior word, so
  the alternative conflicts with the expectation of the reader. The third
  alternative decides the attachment with a score, so the score violates
  the decision "0006".
- The maintainers updated the corpus and updated the Translation Pairs. The
  rule restructured 2 sentences. The rewrite of the 2 sentences is the work
  of the mechanism, so the rewrite is not a bug.
