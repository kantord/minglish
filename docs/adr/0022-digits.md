# 0022 — digits: counts as an open lexer class

Date: 2026-09-01
Status: proposed (tentative). Takes up the numeral pattern ADR 0016 opened.

## Context

The decision "0016" enabled the word "one" but did not decide the plural
numerals. Technical texts use quantities. The language did not have
quantities, so a writer cannot write the phrase "3 files".

The maintainers considered 3 notations:
- the digits
- the words
- a mix of the 2 notations

The words are a treadmill, because a new number needs a new entry of the
Lexicon. A mix gives 2 spellings to one meaning, so a mix violates the rule
of one spelling. The digits are not a Closed Class. The decision "0018" gave the Names to
the Lexer, because the Lexer recognizes a Name by the shape. The Lexer
recognizes a digit by the shape, so a digit does not need an entry of the
Lexicon.

## Decision

A run of the digits is one token with the Form Tag "NUM_PL", so the Lexer
produces the token. The Lexicon does not produce the token. The token sits
in the position of a determiner and takes a plural noun. The token appears
in the sentence "the agent deleted 3 files" and appears in the sentence
"3 agents retry the request". The phrase "a copy of 2 reports" has the
token, so a quantity sits in every position of a plural Noun Phrase.

The language keeps the word "one". The word "one" marks one thing and takes
a singular noun. The Linter maps the digit "1" to the word "one", because
one meaning has one spelling.

The digit "0" is a Ban in the position of a quantity. The Redirect of
the digit "0" follows the decision "0014". If the quantity is
the subject, then the Redirect names the word "no". If the quantity is the
object, then the Bare Plural becomes the object of a Negation. The maintainers
had 3 reasons. The phrase "0 files" duplicates the word "no". The phrase
"0 files" is a universal Negation in the shape of a quantity, so the First
Token does not announce the Negation. The digit "0" takes a plural noun, so
the reader expects an existential statement. The reader drops the expectation
after the digit "0". The word "zero" is a Ban with the Redirect.

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

The Linter maps the 9 words to the digits. The digit "0" does not open a
run of the digits, so the Lexer rejects the string "03".

The maintainers deferred 6 questions:
- the measurements
- the Ordinals
- the units
- the separators
- the decimals
- the negative numbers

The sentence "the exit code is 0" has a measurement. The phrase "0 seconds"
has a measurement. A measurement is a value and does not count things. A
measurement does not count things, so a measurement is the future home of
the digit "0".

## Consequences

- The language can say a quantity. A quantity does not cost an entry of
  the Lexicon, because the digits are not Surface Forms. The report of the
  Lexicon does not grow.
- The Names were the first class of the Lexer. The quantities are the second
  class. The Lexer recognizes the 2 classes by the shape. If the language
  adds the measurements, then the measurements become the third class.
- The Grammar gains one alternative of the rule "NPPL" and gains one
  alternative of the rule "NPInner". The 2 alternatives do not cause an
  ambiguity, so the Grammar keeps the property "LR(1)". The Grammar does
  not need a declaration.
- The Linter gains 2 rules. The first rule rejects the phrase "3 file". The
  second rule rejects the phrase "one files". The Linter gains 3 Redirects.
  The first Redirect maps the digit "0" to a Negation. The second Redirect
  maps the digit "1" to the word "one". The third Redirect maps the 9 words
  to the digits.
