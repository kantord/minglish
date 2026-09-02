# 0028 — Enumeration: a block that lists things

Date: 2026-09-02
Status: proposed (tentative). First block-level structure.

## Context

The repair of the paragraphs turned the list of the pronouns into 9
sentences. The proposal was valid. The proposal was unreadable. Technical
texts enumerate the members of a set with a list. The language had one
kind of the structures. The structure was the sentence. The language
turned a list into a repetition.

## Decision

An Enumeration is a block. An Enumeration is not a sentence. The block has
one plain statement. The statement ends with a colon. The block has one
line for every item. The string "- " opens every item.

The items enumerate the last Noun Phrase of the statement. The Noun Phrase
is plural. A quantity has a plural noun. The phrase "every <noun>" is
legal. The Noun Phrase is the last phrase of the statement. A Prepositional Phrase does not
follow the Noun Phrase. An adjective does not follow the Noun Phrase. A
Coordination does not follow the Noun Phrase. If the statement has a
quantity, then the number of the items matches the quantity.

An item is one Noun Phrase. The language allows 4 kinds of the items:
- a quoted word
- a capitalized term
- a Name
- the phrase "the <noun>"

A clause is not an item. A nested list does not exist.

Every tool reads a block. The block is one unit. The command "just lint" takes a block
in one argument. The corpus keeps the intro with the items. The tool "lint-file" keeps the intro with the items. The extractor of the decisions
keeps the intro with the items. The repair of the paragraphs keeps the
intro with the items. Markdown turns a dash into a bullet.

The maintainers deferred 3 questions. One question is an ordered list. The
items of an ordered list are clauses. The steps of a procedure are one
example. One question is an inner colon. One question is the
intro. A future design can accept a question in the intro.

## Consequences

- One block replaces 4 sentences. The Linter checks the number of the
  items.
- The role of the intro is small. The items enumerate the object or
  enumerate the Complement. The reader does not guess the phrase of the
  list.
- A block has lines. A sentence has one line. The extractor marks the
  boundary of a line with the symbol "⏎".
