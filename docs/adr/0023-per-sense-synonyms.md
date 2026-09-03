# 0023 — One meaning per word: per-sense synonyms first, absolute findability floor

Date: 2026-09-01
Status: proposed (tentative). Amends ADR 0008 (supersedes its point 3 and
the relative frequency trigger). Decides *same*.

## Context

The queue held the word "same". The word "same" opened the question,
because the word has 2 senses in one Category. The phrase "reads the same
file" refers to one file. The reader finds the file in the prior text, so
the decision "0002" bans the first sense. The phrase "have the same format"
compares 2 things. The word "identical" is a precise synonym of the second
sense. The old rule of the decision "0008" compared the 2 frequencies on
the scale Zipf. If the distance was bigger than one point, then the old
rule flagged the synonym. The word "same" has 5.80 points on the scale Zipf,
and the word "identical" has 4.21 points. The distance is 1.59 points, so
the old rule flagged the word "identical".

The old rule confuses 2 quantities. The decision "0006" measures the
expense of a word by the rarity of the word. The expense of a sentence
falls, because the padding goes. The precise words stay, so a dense text
uses rare words. The findability of a word is a different quantity. A
writer produces a familiar word from the memory. The decision "0008"
protects the findability. Every Redirect names a candidate. The findability
is a property of the candidate and does not depend on the distance. The
word "identical" is familiar. The maintainers wrote the decision "0008"
against the word "necessitate", because the word "necessitate" is not
familiar.

## Decision

Every word has one meaning inside a Category. A homograph is one word with
2 Categories, so the goal does not split the homograph.

The Redirects are the first defense. If a Rejected Sense has a precise
synonym, then the Redirect names the synonym. A writer can pick the precise
word from the Redirect. A Language Model finds the precise word in the
Redirect. If a Rejected Sense does not have a precise synonym, then the
sense becomes a Ban. A Ban does not give a replacement, so the writer
changes the sentence.

The guard of the frequency becomes an absolute floor on the candidate. The
floor is 3.5 points on the scale Zipf. If the score of a candidate is
smaller than the floor, then Lexgen shows a warning. The maintainers
review the candidate. The table of the frequencies describes English, so
the table underrates technical words. The warning is not an error, so the
warning does not stop Lexgen. The floor replaces the third point of the
decision "0008", so the old rule of one point goes. The maintainers
checked the floor against the 36 Redirects. The floor flags 2 candidates:
- "outcast"
- "emit"

The candidate "outcast" has 3.09 points, and the candidate "emit" has 3.25
points. The maintainers reviewed the 2 candidates and kept the 2
candidates.

The word "same" becomes a Ban. The Ban gives an advice for every sense. The
first sense refers to one thing, so the writer repeats the noun with the
word "the". The word "the" carries the identity. The second sense compares
2 things, so the writer uses the word "identical". The sentence "the copies
are identical" is one example. The word "identical" enters the Lexicon and
is an ordinary adjective.

The maintainers deferred the phrase "identical to the report", because the
Complement does not take a Prepositional Phrase. A later decision can add
the phrase.

## Consequences

- The maintainers curate every sense of a word, so the Redirects grow. If
  the score of a candidate is smaller than the floor, then the report of
  the Lexicon shows the Redirect to the maintainers.
- The file "docs/ideas.md" describes a repair. The Redirects become the
  table of the replacements for the repair. If the repair finds the roles
  of the words in a Rejection, then the repair maps a Rejected Sense to the
  synonym. The repair is mechanical. The row of the Redirect explains the
  choice. If a synonym is absent, then the repair has a hole. The repair
  needs every synonym, so the second point is a rule. The second point is
  not a preference.
- The anaphoric sense of the word "same" joins 3 Bans:
  - "it"
  - "that"
  - "we"

  The remedy of the decision "0002" stays, so the writer repeats the noun.
