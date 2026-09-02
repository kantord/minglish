# 0014 — every / no, and the first-token telegraph principle

Date: 2026-09-01
Status: proposed (tentative)

## Context

Quantifiers broke 5 sentences of the Dogfood. The Dogfood had 8 sentences. Every list of the
Gaps has the quantifiers. The sentence "every X does not Y" has 2 Parses in
English. The readers disagree on the Parse. A fiat Parse violates the
expectation of a reader. If a quantifier sits inside a sentence, then the
reader rebuilds the logic from the syntax. The phrase "retries no request"
is one example.

## Decision

The word "every" has the Form Tag "QUANT_UNIV". The word "every" takes a
singular noun. The word "every" marks a universal statement. A universal
statement does not tolerate an exception. A Bare Plural tolerates an
exception. The word "every" appears in the subject and appears in the
object. The language bans the word "all". The language bans the word
"each". One meaning has one word.

The word "no" has the Form Tag "QUANT_NEG". The word "no" takes a singular
noun. The word "no" marks a universal Negation. The word "no" appears in
the subject. The word "no" does not appear in the object. The word "no"
opens every universal Negation. The phrase "retries no request" is a Ban.
The sentence "the agent does not retry requests" says the meaning.

A quantified subject takes a positive predicate. A quantified subject bans
4 phrases:
- "not"
- "does not"
- "must not"
- "cannot"

Every Ban has a home. A Negation of a Bare Plural is one home. The phrase
"no X can" is one home. The word "some" is one home. The phrase "no X
must" is a Ban, because English gives 2 Parses to the phrase. The phrase
"no X can" is legal. The sentence "no agent can delete the file" has one
Parse.

A quantified subject appears in a plain statement. A quantified subject
does not appear inside a Conditional. A quantified subject does not appear
inside a Coordination. The bound is provisional. If the corpus shows the evidence, then the maintainers revisit the bound.

If the word "every" appears in the object, then the order of the words is
the order of the scope.

## The first-token telegraph principle

The First Token of a sentence announces the Sentence Shape. The reader
does not need a lookahead. The language has 6 First Tokens:
- the word "if"
- the word "do"
- the word "no"
- the word "every"
- a Bare Plural
- a determiner

The word "if" opens a Conditional. The word "do" opens a Prohibition. The
word "no" opens a universal Negation. The word "every" opens a universal
statement. A Bare Plural opens a generic statement. A determiner opens a
plain statement. Every future Sentence Shape must take a distinct First
Token. The rule helps the reader. The rule helps a Language Model. The rule
helps the parser. The reader knows the frame. The Language Model gets a
constraint. The parser sees a distinct First Token for every Sentence
Shape.

## Consequences

- The language can say a universal rule. The language can say a
  Prohibition of an ability. The 2 Sentence Shapes are natural. The 2
  Sentence Shapes are dense.
- The Bans cause rewrites. Every rewrite is short. Every rewrite is clear.
- The Grammar duplicates the positive predicates. The maintainers must
  clean the macros of the Grammar.
