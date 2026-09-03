# 0028 — Enumeration: a Block that lists things

Date: 2026-09-02
Status: proposed (tentative). First Block-level structure.

## Context

The repair of the paragraphs turned the list of the Pronouns into 9
sentences. The proposal was valid but was unreadable. Technical texts
enumerate the members of a set with a list but the language had one
structure. The structure was the sentence, so a list became a repetition.

## Decision

An Enumeration is a Block and is not a sentence. The Block has one plain
statement and has one line for every item. The statement ends with a
colon. The string "- " opens every item. Markdown turns the dash into a
bullet.

The items enumerate the last Noun Phrase of the statement. The rule is a
fiat decision. The Noun Phrase has 3 legal shapes:
- a plural noun
- a quantity
- the phrase "every <noun>"

The statement must end with the Noun Phrase. A Modifier cannot follow the
Noun Phrase. A Coordination cannot follow the Noun Phrase. If the
statement has a quantity, then the number of the items must match the
quantity.

An item is one Noun Phrase. An item has 4 legal shapes:
- a quoted word
- a capitalized term
- a Name
- the phrase "the <noun>"

A clause cannot be an item. The language does not allow a nested list.

A Block is one unit for every tool. The command "just lint" takes the whole
Block in one argument, so the argument holds the lines of the Block. The
intro stays with the items. The rule covers 4 tools:
- the corpus of the tests
- the tool "lint-file"
- the extractor of the decisions
- the repair of the paragraphs

The maintainers deferred 3 questions. The first question is an ordered
list. The items of an ordered list are clauses. The steps of a procedure
are one example and the rules are a second example. The second question is
a colon inside a sentence. The third question is an intro with the shape
of a question.

## Consequences

- The Block of the 4 Pronouns replaces 4 sentences. The Linter checks the
  number of the items.
- The last Noun Phrase is the object or is the Complement. The maintainers
  limited the role of the intro with a deliberate choice, so the reader
  does not guess the phrase of the list.
- The extractor reads one line for every sentence. An Enumeration is the
  first structure with lines, so the extractor gains one exception. The
  output of the extractor shows the symbol "⏎" at the boundaries of the
  lines.
