# 0031 — A prepositional phrase after a copular complement

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0003; ADR 0011 unchanged.

## Context

The maintainers found 12 sentences of the archetype "A17" in the rewrite
of the 30 decisions. The archetype "A17" names the pattern of the
sentence "the Lexicon is a bad tool for a Name". The
rewrite replaced the phrase with the Noun Preposition or wrote 2 sentences.
The Noun Preposition caused a Propositional Loss in 3 cases. The decision "0011"
links a Verb Preposition to the verb of the clause. A Copula is not a verb
and does not take an object. If a Copula precedes the phrase, then the
phrase has one attachment.

## Decision

- A Copula takes a Noun Phrase. One Prepositional Phrase can follow the
  Noun Phrase. The phrase has a Verb Preposition but attaches to the noun.
  The sentence "the Lexicon is a bad tool for a Name" is one example.
- The rule covers the Complement but does not cover the subject. The
  sentence "the metrics for the load exist in the research" is a Ban,
  because the phrase can attach to the verb or can attach to the noun. The archetype "A5" names the pattern of the sentence "the metrics for
  the load exist in the research". The maintainers keep the rewrite of
  the archetype "A5".
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
