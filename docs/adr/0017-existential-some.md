# 0017 — some: the existential, completing the quantification square

Date: 2026-09-01
Status: proposed (tentative)

## Context

The language had 3 quantified Sentence Shapes:
- the Bare Plural
- the word "every"
- the word "no"

The Negation of a universal statement did not have a home. The decision
"0014" deferred the Negation of a universal statement to the decision
"0017".

## Decision

The word "some" has the Form Tag "QUANT_EXIST". The word "some" takes a
plural noun. The word "some" appears in the subject. The word "some" is the
First Token of an existential statement. The sentence "some agents retry
the request" is one example. The word "some" marks one thing at the
minimum.

The word "some" allows a Negation. The sentence "some agents do not retry
the request" has one Parse in English. The Negation stays inside the
quantifier. The sentence marks the Negation of a universal statement. The square of the quantifiers has 5 members:
- the word "every"
- the word "no"
- the word "some"
- the phrase "some … not"
- the Bare Plural

The phrase "some agent" is a Ban, because the phrase has a different
meaning in English. The phrase "some twenty files" is a Ban. The word
"some" does not appear in the object, because the reader does not see a quantifier in the object.

## Consequences

- Every corner of the quantification has one Sentence Shape. Every quantified
  sentence announces the Sentence Shape with the First Token.
- The word "some" takes the full set of the plural predicates. The set
  includes the Negation. The word "every" takes a positive predicate. The
  word "no" takes a positive predicate. The 2 words have a real Scope
  Ambiguity with a Negation.
