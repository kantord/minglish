# 0026 — so / because: two causal connectives, two information structures

Date: 2026-09-01
Status: proposed (tentative)

## Context

The rewrite of the decision "0001" dropped the phrase "hence this ADR".
The language did not have a causal connective. The decision "0012" says
one thing. A causal relation is a claim. The sweep counted about 11 tokens of
the word "so" in the decisions. The sweep counted about 7 tokens of the
word "because". The corpus has about 17 tokens of the word "because". The
corpus has about 15 tokens of the word "since". The word "since" marks the
time in the corpus. The corpus has about 60 tokens of the word "so". The
word "so" has 3 senses in the corpus.

English offers 2 orders. The phrase "A, so B" puts the reason at the front.
The phrase "B because A" puts the effect at the front. The 2 orders differ
in the information. One clause is old. One clause is new. The reader
prefers the old clause at the front. The decision "0006" puts the old
clause at the front. If the effect sits at the front, then the sentence has an ambiguity of the attachment. The phrase "B because A and C" is one example.

The decision "0007" fixed one order for the Conditional. The decision
"0007" is not a precedent. If the consequent sits at the front, then the consequent is a hypothesis. The
reader holds the hypothesis in the memory. A causal sentence asserts the 2
clauses, so the order does not cost the reader.

## Decision

The language has 2 causal constructions:
- the phrase "<clause>, so <clause>"
- the phrase "<clause>, because <clause>"

The word "so" marks a result. The reason is old. The result is new. The
word "because" marks a reason. The result is old. The reason is new. The seam has a mandatory comma. A causal sentence does not allow a Coordination inside a clause, so the Grammar fixes the scope of the connective.

The word "so" does not open a sentence. The word "because" does not open a
sentence. The rule keeps the First Token. If a reason sits in a prior
sentence, then the writer merges the 2 sentences. A causal verb is the
alternative. The sentence "the expense explains the decision" is one
example.

The word "so" has one sense. The sense of the degree is a Ban. The phrase "so
big" is one example. The sense of the purpose is a Ban. The phrase "so that" is
one example. The language bans 5 words:
- "since"
- "hence"
- "therefore"
- "thus"
- "as"

The advice names the 2 shapes.

The maintainers deferred 3 questions:
- the causal Imperative
- the causal Conditional
- the causal chain

The phrase "A, so B, so C" is a causal chain. The maintainers deferred the
purpose. The phrase "in order to" marks a purpose.

## Consequences

- The decision "0001" paid the debt. The sentence "the expense of a
  reversal grows with the seed, so the maintainers record the decision" is
  the result.
- The Linter names 3 repairs. The Linter finds a comma. The Linter finds
  the word "because" at the front. The Linter finds the word "so" at the
  front.
- The correct connective depends on the prior sentence. The tool "lint-file" checks the prior sentence. The tool "lint-file" measures the
  continuity of the topic.
