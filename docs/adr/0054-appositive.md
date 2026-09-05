# 0054 — Appositive: a Noun Phrase after the subject

Date: 2026-09-05 (revised 2026-09-06: mandatory "namely" marker, fixing
a confirmed garden path — see docs/garden-paths-2026-09-06.md).
Status: proposed (tentative). Naturalness impact not yet proven at
scale — see Consequences.

## Context

A document names a term. A document defines the term in a second
sentence. The pattern repeats the subject. A sentence can carry the
definition inside the sentence. The Grammar does not allow a Noun
Phrase after the subject.

## Decision

The Grammar allows a Noun Phrase after the subject. A comma opens the
Noun Phrase. The predicate follows the second comma. The Noun Phrase
follows a singular subject. A plural subject does not take the Noun
Phrase. The Grammar allows the Noun Phrase inside the Conditional. the
word "namely" opens the Noun Phrase.

## Consequences

- A writer can define a term inside one sentence.
- A test compared 2 versions. the test did not change the score.
- the maintainers keep the decision, because the decision adds an
  ability.
- a test found a Gap. the first version of the decision did not allow
  the Noun Phrase inside the Conditional. the maintainers fixed the
  Gap on the day of the test.
- the corpus does not have an example of the Noun Phrase. a writer
  does not yet write a document with the Noun Phrase. no sentence of
  the corpus uses the shape. a test of the whole corpus does not
  verify the naturalness of the shape.
- a test found a second Gap. a bare noun is an empty word. the
  maintainers ban 11 empty nouns.
- a plain adjective did not fix the Gap. a Grammar does not judge the
  quality of a word. the test does not cover every adjective. a writer
  can choose an empty word.
- the Grammar rejected a second construction. the predicate opens the
  Noun Phrase of the second construction. the maintainers did not add
  the construction.
- a second test found a word without a Collision. the maintainers
  added the word. the word did not fix the Gap of the second
  construction. the maintainers removed the word. the maintainers
  keep the first construction.
- a real test compared 2 documents. the test changed the score of the
  first document. the test did not change the score of the second
  document. the maintainers kept the first document.
- a person confused the shape with a list. a person outranks a
  Language Model. the maintainers reverted the entry of the Linter.
- a test named the Gap. the shape of the Noun Phrase matches the shape
  of the list. a list of the language needs the word "and". the word
  "namely" removes the Gap. the Language Model reads the sentence.
  the Language Model did not confuse the shape with a list.
- a script finds the shape of the Gap inside the Grammar. a script
  found the Gap. the script does not find the Gap of the Noun Phrase.
