# 0018 — Proper names: capitalization mid-sentence, quotes for identifiers

Date: 2026-09-01
Status: proposed (tentative)

## Context

Self-hosting is impossible without naming things: "Minglish needs a
lexicon", the file "seed.json", the tools Lexgen and WordNet. Names are an
open class — curating them into the lexicon is the wrong tool. Quoting all
names works but is heavier than English needs.

## Decision

One open-class NAME token, produced two ways:

- **Capitalized token(s)** — the standard English proper-noun convention,
  **fail-loud**: an unquoted NAME requires capitalized ∧ mid-sentence ∧ not
  a lexicon word in lowercase. Consecutive capitalized tokens merge into one
  NAME. A sentence-initial capital folds to the lexicon or **errors** with
  guidance (introduce the name appositively or quote it) — a typo or OOV
  word must never silently become a name. A mid-sentence capitalized word
  whose lowercase is a lexicon word also errors (caps-typo, or a colliding
  name that needs quotes). *I* is always the pronoun, never a name.
- **A double-quoted span** — verbatim identifiers whose case/spelling cannot
  be distorted: "seed.json", code strings. Any characters allowed inside.
  Scope limit: a quoted span is a **single thing** (one opaque NP).
  *Quotation* — mentioning a sentence or phrase as language ("the writer
  types \"the agent retries the request\"") — is a different construction,
  deliberately NOT covered here; it needs its own future design (see
  docs/ideas.md), likely with recursive parsing of the quoted span.

Grammar: NAME is a singular NP by itself, and an appositive may follow a
noun ("the file \"seed.json\"", "the tool Lexgen"). Names never inflect, are
referentially opaque (repeat the name; no anaphora — consistent with
ADR 0002), and take a flat cost in the metric (frequency is meaningless for
names).

## Consequences

- (2026-09-01, dogfood ADR 0001) The appositive form is also allowed inside
  an of-PP: "every form of the file \"lexicon.tsv\"".

- minglish can finally talk about itself, its files, and its tools —
  several dogfood rows unblock.
- Coverage/cost tooling must treat NAME tokens specially (not lexicon
  misses; flat cost).
- A future lowercase brand name at sentence-initial position must be
  capitalized in minglish orthography or introduced appositively.
