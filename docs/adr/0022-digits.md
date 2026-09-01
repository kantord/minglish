# 0022 — digits: counts as an open lexer class

Date: 2026-09-01
Status: proposed (tentative). Takes up the numeral pattern ADR 0016 opened.

## Context

ADR 0016 admitted *one* (NUM_SG, exactly-one) and left the plural numeral
pattern open. Counts ("3 files", "2 sessions") are ordinary technical
prose and were unwritable. Three notations were on the table: digits,
number words, or both. Number words are a treadmill (every number is a new
lexicon entry) and both-forms violates one-meaning-one-form. Digits are an
open class the lexer can recognize by shape, like NAME (ADR 0018), so no
lexicon entry is needed per value.

## Decision

- A whitespace-delimited digit string is a **NUM_PL** token, produced by
  the lexer, never by the lexicon. It sits in determiner position and
  takes a **plural noun**: "the agent deleted 3 files", "3 agents retry
  the request", "a copy of 2 reports". Same positions as *a/the* + plural.
- ***one* stays a word** (ADR 0016, exactly-one, singular noun). The digit
  *1* is redirected to it. One meaning, one form.
- **0 is banned in count position** and redirected: subject → "no <noun> …"
  (ADR 0014); object → "… does not <verb> <nouns>" (the ADR 0014 routing
  for object-position negation). Reasons: "0 files" duplicates *no*, hides
  a universal negative behind a count-shaped token (breaking the
  first-token telegraph), and pairs with a plural that reads as "some
  exist" until the digit is processed. The word *zero* is banned the same
  way.
- Number words *two* … *ten* are banned with a redirect to the digit.
- Leading zeros are rejected ("03").
- Deferred, no position taken: measurement values ("the exit code is 0",
  "0 seconds" — a value, not a count over nouns; the natural home for a
  future 0), ordinals, units, thousands separators, decimals, negative
  numbers.

## Consequences

- Counts become expressible at zero lexicon cost; the lexicon report does
  not change (digits are not forms).
- Second lexer-produced class after NAME. Both are shape-recognized open
  classes; a future value slot (measurements) would be the third.
- Grammar gains one NPPL alternative and one NPInner alternative; LR(1)
  stays clean without annotations.
- Linter gains two agreement findings ("3 file", "one files") and three
  word-level redirects (0, 1, number words).
