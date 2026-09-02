# 0023 — One meaning per word: per-sense synonyms first, absolute findability floor

Date: 2026-09-01
Status: proposed (tentative). Amends ADR 0008 (supersedes its point 3 and
the relative frequency trigger). Decides *same*.

## Context

The queue had the word "same". The word "same" has 2 senses in one
Category. The phrase "reads the same file" marks one thing. The prior text
resolves the thing. The decision "0002" bans the reference. The phrase
"have the same format" marks 2 identical things. The word "identical" is a synonym of the sense. The rule of the decision "0008" flags the word
"identical". The word "same" is common. The word "identical" is rare. The
distance is big.

The rule of the distance confuses 2 quantities. If the padding goes, then the
expense of a sentence falls. A precise word stays. A dense text uses rare
words. The findability of a word is a different quantity. The findability belongs to the word of the Redirect. The findability does not depend on the distance. Every reader knows the word
"identical". A reader does not know the word "necessitate".

## Decision

Every word has one meaning inside a Category. The rule gives one meaning to a homograph. The rule ignores the Categories of the homograph.

The table of the Redirects is the main defense. If a Rejected Sense has a
synonym, then the Redirect names the synonym. A writer picks the precise
word. If a Rejected Sense does not have a synonym, then the sense is a Ban.

The guard of the frequency puts a floor on the word of the Redirect. The
floor is an absolute frequency. The guard writes a warning. The guard does not stop
Lexgen. The table of English underrates a technical word. The maintainers
dropped the relative distance. The floor flags 2 Redirects:
- "outcast"
- "emit"

The maintainers reviewed the 2 Redirects. The maintainers kept the 2
Redirects.

The word "same" is a Ban. The advice has 2 parts. One sense marks one
thing. The writer repeats the noun with the word "the". One sense marks 2 identical things. The writer uses the word "identical". The sentence "the copies are
identical" is one example. The word "identical" enters the Lexicon.

The maintainers deferred the phrase "identical to the report". A
Complement does not take a Prepositional Phrase.

## Consequences

- The Redirects grow. The report of the Lexicon flags a rare Redirect.
- The file "docs/ideas.md" describes a repair. The repair knows the role of a word. The repair maps a Rejected Sense to the synonym. The row
  of the Redirect explains the choice. If a synonym is absent, then the
  repair has a hole. The rule is a rule, because the repair needs every
  synonym.
- The word "same" joins the Bans of the reference. The word "it" is a Ban
  of the reference. The remedy is the repetition of the noun.
