# 0016 — have (possession) and one (exactly-one numeral)

Date: 2026-09-01
Status: proposed (tentative)

## Context

The sentence "every word has one tag" unfolds the Compound "form-tag". The word "has" broke the sentence. The word "one" broke the sentence.
Triage counts about 335 tokens of the word "have". The word "have" carries a danger in the perfect aspect. The word "one" carries a danger in the phrase "a good one". The Grammar cannot say the 2 dangerous usages.

## Decision

The language enables 3 Surface Forms of "have":
- "have"
- "has"
- "had"

The verb "have" marks a possession. The sense of the Auxiliary does not need a Ban. The Grammar does not have a Participle, so the sentence "the agent
has deleted the file" fails. The verb "has" needs a Noun Phrase. The phrase
"have to" fails, because the Grammar does not have an infinitive. The
pattern matches the decision "0010".

The word "one" has the Form Tag "NUM_SG". The word "one" takes the position
of a determiner. The word "one" takes a singular noun. The word "one" marks
one thing. The word "a" does not mark the number. The Pronoun "one" is a
Ban. The decision opens the pattern of the numerals. The decision "0022"
enabled the digits.

## Consequences

- The Linter parses the sentence "every word has one tag". The language
  can say a possession.
- The rendering of the Compound "form-tag" is approximate. The glossary
  separates the Surface Form from the word. The 2 concepts differ, so the rendering is a Propositional Loss. The decision "0027" defined the term "Surface Form".
- If a future decision admits a Participle, then the maintainers must ban the perfect aspect of "have". If a future decision admits an infinitive,
  then the maintainers must ban the phrase "have to".
