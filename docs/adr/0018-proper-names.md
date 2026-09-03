# 0018 — Proper names: capitalization mid-sentence, quotes for identifiers

Date: 2026-09-01
Status: proposed (tentative)

## Context

The Dogfood needs Names. The sentence "Minglish needs a lexicon" has a
Name. The file "seed.json" is a Name. The tool Lexgen is a Name. The
database WordNet is a Name. The Names are not a Closed Class. The Lexicon is the wrong tool. A Name does not belong in the Lexicon. A quoted span can hold every Name. The quoted spans are heavy.

## Decision

A Name has 2 shapes:
- a capitalized word
- a quoted span

A capitalized word is a Name. The rule is the convention of English. The
rule is strict. An unquoted Name must have a capital. An unquoted Name
must sit inside the sentence. An unquoted Name does not match a word of the Lexicon. If a capitalized word follows a capitalized word,
then the Linter merges the 2 words into one Name. If a capitalized word opens a
sentence, then the Linter folds the word into the Lexicon. If the Lexicon
does not have the word, then the Linter shows an error. The error suggests an appositive or suggests a quoted span. The Linter does not turn a typo into a
Name. If a capitalized word has a twin in the Lexicon, then the Linter shows an error. The error names a typo or names a
Name with a Collision. The word "I" is the Pronoun. The word "I" is not a
Name.

A quoted span is a Name. A quoted span keeps the case of the identifier. A
quoted span keeps the spelling of the identifier. The file "seed.json" is
one example. A quoted span allows every character. A quoted span is one
thing. A quoted span is one opaque Noun Phrase. A quotation is a different
construction. The decision does not cover a quotation. A quotation needs a
future design. The file "docs/ideas.md" describes the design. The design
parses the quoted span.

A Name is a singular Noun Phrase. A Name can follow a noun. The phrase
"the tool Lexgen" is one example. A Name does not inflect. A Name is
opaque. The writer repeats the Name. The rule matches the decision "0002".
A Name has a flat expense in the metric. The frequency of a Name is
meaningless.

## Consequences

- The decision "0027" allowed the appositive inside the phrase of a Noun
  Preposition.
- The language can describe the language. The language can describe the
  files. The language can describe the tools. The Names unblocked rows of
  the Dogfood.
- A Name is not a miss of the Lexicon. A Name has a flat expense.
- If a brand does not have a capital, then the brand needs a capital at the front of a sentence. A writer
  can introduce the brand with a noun.
