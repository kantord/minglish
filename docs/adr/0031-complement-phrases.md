# 0031 — A prepositional phrase after a copular complement

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0003; ADR 0011 unchanged.

## Context

The rewrite of the 30 decisions found 12 sentences of the archetype "A17".
The sentence "the Lexicon is a bad tool for a Name" is one example. The
rewrite replaced the phrase with the Noun Preposition or wrote 2 sentences.
The Noun Preposition changed the sense in 3 cases. The decision "0011"
links a Verb Preposition to the verb of the clause. A Copula is not a verb
and does not take an object. If a Copula precedes the phrase, then the
phrase has one attachment.

## Decision

- A Copula takes a Noun Phrase. One Prepositional Phrase can follow the
  Noun Phrase. The phrase has a Verb Preposition but attaches to the noun.
  The sentence "the Lexicon is a bad tool for a Name" is one example.
- The rule covers the Complement but does not cover the subject. The
  sentence "the metrics for the load exist in the research" is a Ban,
  because the verb can take the phrase. The sentence is one example of the
  archetype "A5", so the maintainers keep the rewrite of the archetype "A5".
- The rule does not cover the object of a verb, so the decision "0011"
  keeps the attachment to the verb.

## Consequences

- If a sentence of the archetype "A17" is good, then the maintainers can
  revert the rewrite of the sentence.
- The Grammar gains a new Complement. The new Complement has one Parse,
  because a Copula does not take an object.
- The maintainer added one warning. The phrase "the wrong tool for a Name"
  is a bad sentence in the context, so a legal shape does not make a good
  sentence.
