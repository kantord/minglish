# 0016 — have (possession) and one (exactly-one numeral)

Date: 2026-09-01
Status: proposed (tentative)

## Context

The decision "0015" rejects the Compound "form-tag" and unfolds the
Compound into the sentence "every word has one tag". The sentence failed on
the word "has" and failed on the word "one". Triage counts about 335 tokens
of the word "have" in the general corpus. The Auxiliary "have" is
dangerous, because the Auxiliary marks the perfect aspect. The Pronoun
"one" is dangerous. The phrase "a good one" shows the danger and the
generic phrase "one must" shows the danger. A writer cannot write the 2
dangerous usages in the Grammar, so the 2 usages are safe.

## Decision

The language admits 3 Surface Forms of the verb "have":
- "have"
- "has"
- "had"

The verb "have" is transitive. The verb marks a possession or marks a
property. The Auxiliary "have" does not need a Ban, because the Grammar
does not have a Participle. The sentence "the agent has deleted the file"
fails, because the verb "has" needs a Noun Phrase. The phrase "have to"
fails, because the Grammar does not have an infinitive. The structure of
the Grammar excludes the 2 usages, so the decision does not add a Ban. The
decision "0010" excluded the 2 aspects in the identical way.

The word "one" has the Form Tag "NUM_SG" and takes the position of a
determiner. A singular noun follows the numeral. The numeral marks one
thing by a fiat decision. The word "a" does not exclude a second thing, so
the numeral differs from the word "a". The language does not admit the
Pronoun "one". The word "one" is the first numeral. A later decision can
add the plural numerals with the Form Tag "NUM_PL". The phrase "three
files" shows the pattern. The decision "0016" opens the pattern but does
not adopt the pattern.

## Consequences

- The Linter parses the sentence "every word has one tag". A writer can
  say a possession, so the verb "have" opens a big class of the prose. The
  prose describes the properties of a thing.
- The rendering of the Compound "form-tag" is approximate. The glossary of
  the project separates a Surface Form from a word. The sentence "every
  word has one tag" merges the 2 concepts, so the sentence has a
  Propositional Loss. The decision "0012" defines the Propositional Loss. An exact sentence waits for a transparent rendering
  of the phrase "surface form".
- If a future decision admits a Participle, then the maintainers must ban
  the Auxiliary "have". If a future decision admits an infinitive, then the
  maintainers must ban the phrase "have to". The condition matches the
  condition of the decision "0010".
