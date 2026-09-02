# 0027 — Domain model: defined terms as a capitalized pack

Date: 2026-09-02
Status: proposed (tentative). Realizes the "core lexicon + jargon packs"
idea; supersedes the hand-written CONTEXT.md glossary.

## Context

The maintainers used the repair on the decision "0002". The runs produced 6
valid proposals. The maintainers judged every proposal. No proposal was ideal. The wording was unnatural. The proposals decomposed a
concept of the project into general words. The phrase "the ambiguity of
the reference" is one example. The phrase "the layer of the discourse" is
one example. The language did not mark a term. The decision
"0015" requires a definition for an opaque coinage. The system did not
enforce the definitions. The system did not use the definitions. A term can be a verb. The word "waive" is one example. The word
"attest" is one example. The word "anaphoric" is an adjective. A list of
the nouns does not cover the terms.

## Decision

The file "domain/model.json" has the shape of the Seed. An entry has the
fields of the Seed. An entry has a definition. The definition is Minglish.
Lexgen merges the model into the Lexicon. The model adds words. The model does
not override a word of the Seed. If a Lemma of the model repeats a Lemma of the Seed in one Category, then Lexgen shows an error. An entry of a
Name has a definition. The entry does not have Surface Forms. The model
has 4 Names:
- Minglish
- Lexgen
- Triage
- WordNet

A writer writes a term with a capital. A term can have 2 words. The Lexer gives one
token to a term. The token has a number. The sentence "the Linter bans
Anaphoric Pronouns" is one example. The sentence "every Surface Form has
one Form Tag" is one example. If the model does not have a capitalized
word, then the word is a Name. The rules of the decision "0018" cover the
Name. A verb of the model does not have a capital. An adjective of the model does not have a capital. If a term does not have a capital, then the Linter shows an advice.

The script "check.sh" lints every definition. The prompt of the repair
contains every term. The prompt contains every definition. The command
"just define" shows a definition. Lexgen writes the file "CONTEXT.md" from
the model. The script "check.sh" checks the file "CONTEXT.md".

The maintainers moved the terms of the glossary into the model. The maintainers moved the jargon of the project into the model. The model got 24 nouns. The model got 4 Names. The
model got 5 verbs. The model got 4 adjectives. The maintainers put capitals on the terms of the corpus. The maintainers put capitals on the terms of the decision "0001". The maintainers put capitals on the terms of the skill. A
definition replaces the attestation for a term. The data does not attest a
coinage. Lexgen checks the Collisions of a term. Lexgen checks the
Categories of a term.

## Consequences

- The Language Model sees one unit. The unit has a meaning. The repair does not decompose a concept of the project. A reader sees a signal.
  The signal marks a term. The file "CONTEXT.md" defines every
  term.
- The capital carries a meaning. The orthography of the decision "0018"
  has 2 exceptions. A Name is one exception. A term is one exception. A term can open a sentence. The term is not ambiguous, because the term is a Surface Form of the Lexicon.
- A definition is a claim. The claim covers a fact of the project. The decision "0012" covers
  every definition.
- The maintainers deferred 3 questions. One question is the signal. A verb of the model does not have a signal. One question is a pack. A different domain needs a pack. One
  question is the word "Triage". The word "Triage" is a Name. The word
  "triage" is a noun.
