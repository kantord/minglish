# 0001 — Lexicon is a generated artifact from a curated Seed list

Date: 2026-08-31
Status: accepted (rewritten in minglish 2026-09-01; meaning preserved, see docs/dogfood-adr-0001.md)

## Context

The language needs a Lexicon. The Lexicon contains every Surface Form of the
words. Every Surface Form has one Form Tag. A word can have an unused sense. The
Lexicon can contain a replacement for the unused sense.

The maintainers considered 3 options.

1. The option "algorithm" derives the Lexicon from the data. A machine
   chooses the words.
2. The option "handwriting" uses a handwritten file. People write every
   Surface Form of the file `lexicon.tsv`. A tool checks the file.
3. The option "Seed" uses a small Seed. People write the Seed. A machine
   builds the Lexicon from the Seed.

The file `docs/research/cnl-design-findings.md` describes the prior
research. The algorithms optimized the rarity of the words. The
algorithms did not optimize the clarity of the words. One generator
replaced common words with rare words. The rarity of the rare words
explained the apparent improvement. 43 percent of the swaps did not
reduce the ambiguity. People must choose the words. Machines must not
choose the words.

Every verb has about 5 Surface Forms. If people write every Surface Form, then the
Lexicon has typos. If people write every Surface Form, then the Lexicon has
hidden holes. No Surface Form can have 2 Form Tags. The noun "leaves" collides with the
verb "leaves". People cannot find every Collision. Machines can find
every Collision.

## Decision

The maintainers chose the option "Seed". People edit the file
`seed/seed.json`. The file `seed/seed.json` is the source of the truth.
Every Lemma has one entry. An entry has a Category. An entry can contain
irregular Surface Forms. An entry can contain a replacement for an unused sense.
The maintainers can waive the replacement of a sense. An entry can have a
note. The note records the rationale.

The tool Lexgen expands the Paradigms with about 10 rules. The tool Lexgen
checks the invariants. No Surface Form has 2 Form Tags. An unused sense has a
replacement or has a Waiver. The data attests every Surface Form. If a test
fails, then the tool Lexgen returns an error. The tool Lexgen writes the
file `lexicon.tsv` and writes the file `docs/lexicon-report.md`. The tool
Lexgen is deterministic. The maintainers save the generated files in the
repository. People do not edit the generated files.

The repository stores the database WordNet. The repository stores the
file `mobypos`. The script `scripts/fetch-data.sh` fetches the table
`en_zipf.tsv`. The file `data/checksums.sha256` pins the data. The
database WordNet attests the senses of the words. The file `mobypos`
attests the Categories of the words. The table `en_zipf.tsv` contains the
frequency of the words. The Linter checks the words with the data. The
data does not choose the words.

## Consequences

- People choose the words. Every word costs one entry. The machine writes
  the inflections and finds the Collisions.
- Every commit shows the effect of the commit in the diffs. The tool Git
  stores the history of the Lexicon.
- The repository stores the generated files. The generated files can
  drift from the Seed. The script `scripts/check.sh` finds the
  differences. If a person edits the file `lexicon.tsv`, then the script
  `scripts/check.sh` fails.
- A reversal migrates every entry. The expense of a reversal grows with
  the Seed, so the maintainers record the decision.
