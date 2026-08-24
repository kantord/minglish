# 0001 — Lexicon is a generated artifact from a curated seed list

Date: 2026-08-31
Status: accepted

## Context

minglish needs a lexicon: every allowed surface form, each with exactly one
form-tag, plus redirects for rejected uses. Three ways to produce it were
considered:

1. Derive it algorithmically from corpora/frequency lists and WordNet.
2. Hand-write `lexicon.tsv` directly and validate it.
3. Hand-curate a small seed list (lemma + category + irregulars + redirects);
   generate the lexicon from it mechanically.

Prior research (docs/research/cnl-design-findings.md) showed that algorithmic
word selection optimizes rarity, not clarity: a swap generator's apparent
disambiguation was fully explained by picking rarer words, and 43% of its
swaps had no benefit at all. Word choice must stay a human judgment. At the
same time, hand-writing every inflected surface form (full paradigms, ~5
forms per verb) is toil that invites typos and silent gaps, and the critical
safety property — no surface form with two tags, including cross-lemma
inflection collisions like *leaves* — is exactly the kind of check humans miss
and machines don't.

## Decision

Option 3. A hand-edited `seed/seed.json` is the single source of truth: one
entry per lemma with its category, irregular-form overrides, `reject`
redirects, optional `waive`, and a free-text `note` recording the rationale.
A Rust tool (`crates/lexgen`) expands paradigms with ~10 regular-morphology
rules, enforces the linter invariants (collision-free, cross-POS
completeness, no unattested forms) as hard build errors, and deterministically
emits `lexicon.tsv` and `docs/lexicon-report.md`. The maintainers commit the
generated files. People do not edit the generated files. The maintainers
vendor the reference data (WordNet 3.0 indices, mobypos, a zipf frequency
table) and pin the data by checksum. The linter checks the words with the data. The data
does not choose the words.

## Consequences

- People choose the words; each word costs one JSON entry. The machine
  handles inflection and the safety checks.
- Every curation commit shows its effect in the diffs of `lexicon.tsv` and
  the metrics report, so iteration history lives in git log.
- The committed-generated-file pattern requires discipline (and eventually a
  CI check) that `lexicon.tsv` matches `seed.json`; a hand-edit to the TSV is
  a build error waiting to happen.
- Reversing this later (e.g. moving to a database or to per-word files) means
  migrating accumulated curation, which gets more expensive as the seed grows
  — hence this ADR.
