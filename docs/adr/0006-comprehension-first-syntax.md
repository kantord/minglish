# 0006 — Syntax principle: comprehension-first branching, minimal context need

Date: 2026-08-31
Status: accepted (governing principle for all grammar-tier decisions; rewritten in minglish 2026-09-02)

## Context

The Grammar needs a rule for a decision. The research grounds the
difficulty of a sentence in the Cognitive Load. The file
`docs/research/cnl-design-findings.md` describes the research. The 3
metrics predict the difficulty. A rule of the style does not predict the
difficulty.

A Ban of a shape overshoots. One sentence had a heavy branch. The sentence
carried a small load, so the sentence was easy. A good constraint puts a
bound on the load. A good constraint does not ban a shape.

A reader has the profile of a Language Model. A reader reads a
sentence in the order of the words. A Language Model reads a sentence in
the order of the words. The reader has a small memory. The Language Model
has a small memory.

## Decision

The maintainers weigh 5 criteria:
- the Cognitive Load
- the familiarity
- the Context Need
- the density
- the expressiveness

The Cognitive Load outranks the familiarity. The familiarity outranks the
Context Need. The Context Need outranks the density. The density outranks
the expressiveness.

If a structure has a small Cognitive Load, then the structure wins. The
rules of the branching help a reader and help a Language Model. The rules do not follow a taste.
The rules do not follow a tradition.

If the ideal branching is not available, then the maintainers choose the
common construction of English. A familiar construction helps the reader.
A surprise costs the reader.

A sentence needs a small context. The rule explains the Ban of the
Anaphoric Pronouns. The rule explains the repetition of the nouns. The rule
explains the rule of the scope.

The target is a dense text. A dense text carries a precise meaning. The
maintainers prefer the short formulation inside the bounds. A long text is
not free. A verbose text is noisy. The noise causes ambiguities. If a
rule adds words, then the rule must remove an ambiguity. The repetition of
the nouns removes an ambiguity. The word "then" removes an ambiguity.

The expressiveness is a real goal. A text of Minglish is pleasant. A text of
Minglish is not robotic. The 4 criteria outrank the
expressiveness. If 2 formulations tie on the 4 criteria, then the
maintainers choose the expressive formulation. The maintainers do not
trade the precision for the charisma.

A constraint is a bound. A constraint is not a Ban of a shape. If a rule
adds words, then the rule must earn the words. The maintainers accept a
long text for a small load.

The primary tool is the set of the Sentence Shapes. The Grammar builds the
comprehension into the Sentence Shapes. Every Sentence Shape puts the head at the front of the sentence. The language cannot say a bad shape. The score of a sentence is the
secondary tool. The score verifies the claim. The score
monitors the load. The score gathers the evidence for a future bound. The score
does not protect the readability of the text.

## Consequences

- Every future decision of the Grammar must justify the choice against the
  5 criteria.
- The research defines the metrics of the Cognitive Load. The tool
  "parse-report" measures the metrics on the Parse of a sentence.
- If a natural formulation conflicts with an easy formulation, then the easy
  formulation wins. The familiarity limits the loss.
