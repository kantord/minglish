# 0001 — Lexicon is a generated artifact from a curated seed list

Date: 2026-08-31
Status: accepted (rewritten in minglish 2026-09-01; meaning preserved, see docs/dogfood-adr-0001.md)

## Context

The language needs a lexicon. The lexicon contains every form of the
words. Every form has one tag. A word can have an unused sense. The
lexicon can contain a replacement for the unused sense.

The maintainers considered 3 options.

1. The option "algorithm" derives the lexicon from the data. A machine
   chooses the words.
2. The option "handwriting" uses a handwritten file. People write every
   form of the file `lexicon.tsv`. A tool checks the file.
3. The option "seed" uses a small seed. People write the seed. A machine
   builds the lexicon from the seed.

The file `docs/research/cnl-design-findings.md` describes the prior
research. The algorithms optimized the rarity of the words. The
algorithms did not optimize the clarity of the words. One generator
replaced common words with rare words. The rarity of the rare words
explained the apparent improvement. 43 percent of the swaps did not
reduce the ambiguity. People must choose the words. Machines must not
choose the words.

Every verb has about 5 forms. If people write every form, then the
lexicon has typos. If people write every form, then the lexicon has
hidden gaps. No form can have 2 tags. The noun "leaves" collides with the
verb "leaves". People cannot find every collision. Machines can find
every collision.

## Decision

The maintainers chose the option "seed". People edit the file
`seed/seed.json`. The file `seed/seed.json` is the source of the truth.
Every lemma has one entry. An entry has a category. An entry can contain
irregular forms. An entry can contain a replacement for an unused sense.
The maintainers can waive the replacement of a sense. An entry can have a
note. The note records the rationale.

The tool Lexgen expands the paradigms with about 10 rules. The tool Lexgen
checks the invariants. No form has 2 tags. An unused sense has a
replacement or has a waiver. The data attests every form. If a test
fails, then the tool Lexgen returns an error. The tool Lexgen writes the
file `lexicon.tsv` and writes the file `docs/lexicon-report.md`. The tool
Lexgen is deterministic. The maintainers save the generated files in the
repository. People do not edit the generated files.

The repository stores the database WordNet. The repository stores the
file `mobypos`. The script `scripts/fetch-data.sh` fetches the table
`en_zipf.tsv`. The file `data/checksums.sha256` pins the data. The
database WordNet attests the senses of the words. The file `mobypos`
attests the categories of the words. The table `en_zipf.tsv` contains the
frequency of the words. The linter checks the words with the data. The
data does not choose the words.

## Consequences

- People choose the words. Every word costs one entry. The machine writes
  the inflections and finds the collisions.
- Every commit shows the effect of the commit in the diffs. The tool Git
  stores the history of the lexicon.
- The repository stores the generated files. The generated files can
  drift from the seed. The script `scripts/check.sh` finds the
  differences. If a person edits the file `lexicon.tsv`, then the script
  `scripts/check.sh` fails.
- A reversal migrates every entry. The expense of a reversal grows with
  the seed, so the maintainers record the decision.
