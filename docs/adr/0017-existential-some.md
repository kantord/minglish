# 0017 — some: the existential, completing the quantification square

Date: 2026-09-01
Status: proposed (tentative)

## Context

The Bare Plural of the decision "0013" marks a generic statement. The
decision "0014" added the word "every" for a universal statement and added
the word "no" for a universal Negation. The Negation of a universal
statement did not have a Sentence Shape, so the decision "0014" deferred
the Negation to the present decision.

## Decision

The word "some" has the Form Tag "QUANT_EXIST" and takes a plural noun. The
word "some" appears in the subject and is the First Token of an existential
statement. The sentence "some agents retry the request" is one example. The
word "some" has a minimum of one agent and does not have a maximum.

The word "some" allows a Negation, because the word "not" cannot cover the
word "some" in English. The sentence "some agents do not retry the request"
has one Parse and is the Negation of a universal statement. The square of
the quantifiers becomes complete. The language has 5 quantified Sentence
Shapes:
- the word "every"
- the word "no"
- the word "some"
- the phrase "some … not"
- the Bare Plural

The phrase "some agent" names an unknown identity in English. The sense is
different, so the phrase is a Ban. The phrase "some twenty files" is a Ban,
because the approximate sense does not enter the language. The argument of
the decision "0014" bans the word "no" in the object and bans the word
"some" in the object. If a quantifier sits inside a sentence, then the
reader rebuilds the logic from the syntax.

## Consequences

- Every corner of the square has one Sentence Shape. Every quantified
  sentence announces the Sentence Shape with the First Token.
- The word "some" takes every plural predicate and takes the Negation. The
  decision "0014" limits the word "every" to a positive predicate and
  limits the word "no" to a positive predicate. The bound stays, because
  the 2 words have a real Scope Ambiguity with a Negation.
