# Rewrite archetypes

Recurring problems met while rewriting the ADRs in minglish, logged as they
occur (2026-09-02). Each is a candidate for the review after all ADRs
parse: decide whether it is a language gap, a linter gap, or an accepted
rewrite cost.

| # | archetype | example | how it was handled | count |
|---|---|---|---|---|
| A1 | ordinal / ranking (*first*, *second*, *largest*, *worst*) | "the second-largest gap" | drop declared as `ranking`; "X outranks Y" chains | 6 |
| A2 | *only* | "permits them solely in …" | ban with advice; state the allowed case, name the excluded ones | 4 |
| A3 | comparative (*more expensive*, *shorter*, *longer*) | "length is cheaper than load" | two absolute sentences; `comparative` drop | 5 |
| A4 | change of state (*become*, *get longer*, *enters*) | "become expressible" | "the language can say X"; *enter* as a verb | 4 |
| A5 | noun + non-of PP as a unit | "a Copula with a Participle", "the metrics for the load" | conditional, or restructure with a verb | 8 |
| A6 | inline list / enumeration of clause items (rules, steps) | numbered rule lists | Enumeration for noun items; rules become sentences | 6 |
| A7 | NP coordination, esp. in subject | "a reader and a language model" | repeat the verb, or "X with Y" | 7 |
| A8 | mass noun without determiner | "adds noise", "instructional text" | "the noise"; adjective rewrite; Bare Plural | 5 |
| A9 | resultative / copula + PP (*keep the text readable*, *is in the lexicon*) | "keeps the loss small" | verb rewrite (*limits the loss*) | 5 |
| A10 | exemplification ("e.g.", "no X, no Y" as examples) | "no 'not all users'" | "the phrase X is one example" | 9 |
| A11 | project jargon needing a term | copula, conjunct, tier | domain model entry with definition | 45+ |
| A12 | one-tag collisions with a needed sense | *mean/meaning*, *finding/find*, *hidden/hide* | pick the verb or the noun; rewrite the other | 6 |
| A13 | sequence / time (*after*, *then*, *later*, *before*, *until*) | "did later added by ADR 0010" | restructure; `sequence` relation is the top missing form | 7 |
| A14 | ditransitive (*gives X Y*, *tells the writer one thing*) | "gives every sentence one parse" | "gives Y to X" | 3 |
| A15 | stale claims in the source | "documentation-only until the grammar tier" | rewrite to current facts, `update` drop | 9 |
| A16 | *be* after a modal (*must be*, *can be*) | "a hyphenated word must be transparent" | plain copula, or a conditional | 7 |
| A17 | noun phrase + PP as a copular complement (*is a signal for X*, *is a big blocker in Y*) | "is the wrong tool for a Name" | of-phrase, or a second sentence | 12 |
| A18 | a word needed in two categories (*count*, *cost*, *match*, *state*, *fix*, *change*) | "a count", "the cost" | one category wins; the other sense gets a different word (*quantity*, *expense*) | 14 |
| A19 | a term used lowercase in its general sense (*gap*, *name*, *form*) | "the gap is big" | a different general word (*distance*, *word*) | 8 |
| A20 | "X of Y" with a bare plural or *every* inside | "5 of 8 sentences", "a chain of Noun Prepositions" | determiner on the inner noun, or restructure | 6 |

Counts are approximate tallies over the 28 rewrites (2026-09-02). Archetypes
A1, A3, A13, A16, A17 and A18 are language questions; A11 and A19 are
vocabulary policy; the rest are accepted rewrite costs unless the review
says otherwise.
