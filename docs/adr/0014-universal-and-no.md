# 0014 — every / no, and the first-token telegraph principle

Date: 2026-09-01
Status: proposed (tentative)

## Context

The Dogfood has 8 sentences. Quantifiers break 5 sentences of the
Dogfood. The sentence "every X does not Y" has 2
Parses in English. The readers disagree on the Parse, so a fiat decision
violates the expectation of the readers. If a quantifier does not open the
sentence, then the reader rebuilds the logic from the syntax. The phrase
"retries no request" is one example.

## Decision

The word "every" has the Form Tag "QUANT_UNIV" and takes a singular noun.
The word "every" marks a universal statement, so the statement does not
tolerate an exception. A Bare Plural tolerates an exception, because a
Bare Plural marks a generic statement. The word "every" appears in the
subject and appears in the object. The language bans the word "all" and
bans the word "each". The Ban gives one word to every meaning.

The word "no" has the Form Tag "QUANT_NEG" and takes a singular noun. The
word "no" marks a universal Negation and appears in the subject. The object
cannot carry the word "no", so the word "no" opens every universal
Negation. The phrase "retries no request" is a Ban, so the sentence "the
agent does not retry requests" carries the meaning.

A quantified subject takes a positive predicate. The rule excludes 4
phrases:
- "is not"
- "does not"
- "must not"
- "cannot"

The 4 Bans have 3 unambiguous replacements:
- the Negation of a Bare Plural
- the phrase "no X can"
- the word "some"

The maintainers did not map every Ban to one replacement. The language
does not have the word "some", so the third replacement is future. The
phrase "no X must" is a Ban, because the phrase has 2 meanings in
English. The phrase "no X can" is legal, because the sentence "no agent
can delete the file" has one Parse.

A quantified subject opens a whole sentence. The subject does not appear
inside a Conditional and does not appear inside a Coordination. The rule is
a provisional bound, so the maintainers revisit the bound with the evidence
of the corpus.

If the word "every" appears in the object, then the order of the words
decides the scope: the order of the sentence is the order of the scope.

## The first-token telegraph principle

The First Token of a sentence announces the Sentence Shape. 4 First Tokens
are words:
- "if"
- "do"
- "no"
- "every"

3 First Tokens are Categories:
- a Bare Plural
- a determiner
- an Indexical Pronoun

Every future Sentence Shape must take a distinct First Token. A question is
one example of a future Sentence Shape. An existential statement is a
second example. The maintainers do not define an existential statement. The rule helps the reader, because the reader knows the
frame before the claim. The rule helps a Language Model, because the
Language Model gets a constraint from the First Token. The parser sees a
distinct First Token for every Sentence Shape, so the rule helps the
parser.

## Consequences

- The language can say a universal rule with the word "every". The language
  can say a universal Negation of an ability with the phrase "no X can". The 2
  shapes are the natural shapes of English and are dense.
- The Bans cause rewrites. The rewrites are shorter than the original
  phrases or are clearer than the original phrases.
- The Grammar duplicates the positive predicates, so the maintainers must
  clean the macros of LALRPOP.
