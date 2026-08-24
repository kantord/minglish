# minglish

An intentionally minimal, formalized subset of English: every enabled word
has exactly one syntactic role, so text can be parsed deterministically.
Early stage — currently the lexicon layer and its measurement tooling.

## Layout

- `seed/seed.json` — hand-curated source of truth (the only hand-edited input)
- `crates/lexgen` — generates `lexicon.tsv` + `docs/lexicon-report.md`,
  enforcing the lexicon invariants (see `docs/adr/0001`)
- `crates/triage` — evaluates the lexicon against a gold-tagged corpus
- `corpus/accept.txt` — target sentences minglish must express
- `CONTEXT.md` — glossary · `docs/adr/` — decisions · `docs/research/` — findings

## Build

```
./scripts/fetch-data.sh     # once: fetch/derive non-vendored reference data
cargo run -p lexgen         # regenerate lexicon + report (lint-gated)
cargo run -p triage         # evaluate against UD-EWT
```

## License

Code and original content: MIT OR Apache-2.0, at your option
(see LICENSE-MIT, LICENSE-APACHE).

## Third-party data notices

- **WordNet 3.0** (vendored in `data/wordnet/`): © Princeton University,
  used under the WordNet License — see `data/wordnet/LICENSE`.
- **Moby Part-of-Speech** (vendored in `data/moby/`): Grady Ward, public
  domain.
- **UD_English-EWT r2.16** (fetched, not vendored): © the UD English-EWT
  contributors, CC BY-SA 4.0
  (<https://github.com/UniversalDependencies/UD_English-EWT>). Used
  unmodified as an evaluation corpus; triage reports quote sample sentences
  from it.
- **wordfreq 3.1.1 data** (derived table, fetched/regenerated, not vendored):
  Robyn Speer, data CC BY-SA 4.0
  (<https://github.com/rspeer/wordfreq>). Change: reformatted to a
  word/zipf TSV, truncated to the top 100k English words.

The generated `lexicon.tsv` is original work: reference data is used only to
*check* human word choices, never to select or copy content (ADR 0001).
