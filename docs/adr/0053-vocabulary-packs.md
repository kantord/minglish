# 0053 — Vocabulary packs: a general pack, not only a domain pack

Date: 2026-09-05
Status: proposed (tentative). Generalizes ADR 0027.

## Context

The decision "0027" adds one pack. The pack is the file
"domain/model.json". The pack adds a term of the project. A writer of a
report is not a writer of a letter.

An old document is not a document of the project. The maintainers do
not write the old document. A test judged a rewrite of the old
document. The test found a Gap. The Seed does not have the word "joy".
The Seed does not have the word "fear". The Gap is not a Gap of one
document. The Gap is a Gap of one register.

A report has one register. The register needs different words. A
letter has one register. The register needs different words. The
report does not need the word "breeze". A pack does not add the word
"breeze" to the Seed.

## Decision

Lexgen adds a pack after the Seed. A pack has the shape of the Seed. A
pack has the 6 fields of the Seed:
- "lemma"
- "category"
- "forms"
- "reject"
- "waive"
- "note"

The pack follows one rule of the pack "0027". The pack adds a word.
The pack does not override a word of the Seed. The pack does not
override a word of the file "domain/model.json".

Lexgen checks a word of a pack against WordNet. Lexgen already checks a
word of the Seed against WordNet. An entry stops Lexgen without a
Waiver. An entry stops Lexgen without a Redirect. A Collision stops
Lexgen.

## Consequences

- A document does not yet choose a pack. Every document adds every
  pack.
- A future pack can cover a different register.
- A writer of a pack chooses only a strong word.
