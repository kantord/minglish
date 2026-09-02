# 0031 — A prepositional phrase after a copular complement

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0003; ADR 0011 unchanged.

## Context

The rewrite of the 30 decisions met 12 sentences of the archetype "A17".
The sentence "the Lexicon is a bad tool for a Name" is one example.
A rewrite used the Noun Preposition or used 2 sentences. The Noun
Preposition changed the sense in 3 cases. The decision "0011" puts a Verb Preposition on the verb of the clause. A Copula is not a verb. A Copula does not take an
object. If a Copula precedes the phrase, then the phrase has one home.

## Decision

- A Copula takes a Noun Phrase. One Prepositional Phrase of a Verb Preposition can follow the Noun Phrase. The phrase attaches to the noun. The sentence "the Lexicon
  is a bad tool for a Name" is one example.
- The rule covers the Complement. The rule does not cover the subject. The
  sentence "the metrics for the load exist in the research" is a Ban,
  because the verb can take the phrase. The maintainers keep the rewrite of the archetype "A5".
- The rule does not cover the object of a verb. The decision "0011" keeps
  the attachment to the verb.

## Consequences

- If a sentence of the archetype "A17" is good, then the maintainers can revert the rewrite.
- The Grammar gains one Complement. The sentence has one Parse, because the Copula does not have an object.
- The maintainer said one thing. The phrase "the wrong tool for a Name" is a bad sentence in the context. A legal shape is not a good sentence.
