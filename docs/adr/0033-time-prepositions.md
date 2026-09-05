# 0033 — after, before, until: time relative to an event

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0011; clause-level sequence deferred.

## Context

The rewrite of the decisions found 7 sentences of the archetype "A13".
The archetype "A13" names the pattern of the sentence "the ambiguity
reappears after the split". The
file "coherence-report.md" counts 39 tokens of a connective. The
connective marks a sequence. The language did not have a shape for a
sequence, so a writer turned every sequence into 2 sentences. The relation
of the 2 sentences was implicit. One example is the sentence "the ambiguity
reappears after the split".

## Decision

The language adds 3 Verb Prepositions:
- "after"
- "before"
- "until"

A Noun Phrase follows every new word and names an event. The sentence "the
ambiguity reappears after the split" is one example. The sentence "the
reader holds the consequent until the condition" is a second example. The
rules of the decision "0011" cover the 3 words, so the phrase attaches to
the verb. A clause has one Verb Preposition.

The sentence "after the test fails, the agent retries the request" is a
Ban, because the maintainers deferred the sequence of 2 clauses. The
maintainers deferred a shape for the ordered steps. The maintainers
deferred the word "then" at the front of a sentence.

## Consequences

- A writer names the event with a noun, so the language says the sequence.
- The Lexicon gains 3 Function Words, but the Grammar does not gain a shape.
- The file "coherence-report.md" names the Gap of the sequence.
