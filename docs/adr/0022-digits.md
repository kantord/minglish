# 0022 — digits: counts as an open lexer class

Date: 2026-09-01
Status: proposed (tentative). Takes up the numeral pattern ADR 0016 opened.

## Context

The decision "0016" enabled the word "one". The decision "0016" did not
decide the plural numerals. Technical texts use quantities. The
phrase "3 files" is one example. The language did not have a quantity.

The maintainers considered 3 notations:
- the digits
- the words
- a mix of the 2 notations

The words are a treadmill. Every number is a new entry of the Lexicon.
A mix violates the rule of one word. The digits are not a Closed Class. The
Lexer recognizes a digit by the shape. The Lexer recognizes a Name by the
shape. A digit does not need an entry of the Lexicon.

## Decision

A run of the digits has the Form Tag "NUM_PL". The run is one token. The Lexer
produces the token. The Lexicon does not produce the token. The token sits
in the position of a determiner. The token takes a plural noun. The
sentence "the agent deleted 3 files" is one example. The sentence "3
agents retry the request" is one example. The phrase "a copy of 2 reports"
is one example. A quantity takes the positions of a plural Noun Phrase.

The language keeps the word "one". The word "one" marks one thing. The Linter
maps the digit "1" to the word "one". One meaning has one word.

A quantity does not use the digit "0". The digit "0" is a Ban. The Linter maps the digit "0". If
the quantity is the subject, then the writer uses the word "no". If the quantity
is the object, then the writer uses a Negation with a Bare Plural. The
phrase "0 files" duplicates the word "no". The phrase "0 files" covers a
universal Negation with a quantity. The phrase breaks the First Token. The reader sees a plural noun. The reader expects an existential statement. The
word "zero" is a Ban.

The language bans 9 words:
- "two"
- "three"
- "four"
- "five"
- "six"
- "seven"
- "eight"
- "nine"
- "ten"

The Linter maps the 9 words to the digits. The Lexer rejects the
string "03".

The maintainers deferred 6 questions:
- the measurements
- the ordinals
- the units
- the separators
- the decimals
- the negative numbers

The sentence "the exit code is 0" has a measurement. A measurement is a
value. A measurement does not count things. A measurement is the future
home of the digit "0".

## Consequences

- The language can say a quantity. A quantity does not cost an entry of the
  Lexicon. The report of the Lexicon keeps the size.
- The Lexer produces 2 classes of the tokens. The Names are one class.
  The quantities are one class. The Lexer recognizes the 2 classes by the
  shape. A future class holds the measurements.
- The Grammar gains 2 alternatives. The Grammar does not have an error.
- The Linter gains 2 rules. The phrase "3 file" is one example. The
  phrase "one files" is one example. The Linter gains 3 Redirects. The
  Linter maps the digit "0". The Linter maps the digit "1". The
  Linter maps the 9 words.
