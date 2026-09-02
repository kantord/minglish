# 0027 — Domain model: defined terms as a capitalized pack

Date: 2026-09-02
Status: proposed (tentative). Realizes the "core lexicon + jargon packs"
idea; supersedes the hand-written CONTEXT.md glossary.

## Context

Five paragraph-repair runs on ADR 0002 produced six valid proposals, all
judged needs-fix for unnatural wording. The pattern: a project concept
decomposed into general words ("the ambiguity of the reference", "the
layer of the discourse") because the language had no way to say *this is
one defined thing*. ADR 0015 already requires that an opaque coinage enter
only with a stated definition; nothing enforced or used the definitions.
Project terms are not only nouns (*waive*, *attest*, *anaphoric*), so a
noun-only list would not do.

## Decision

- **`domain/model.json`** is a second seed-shaped file: the same entry
  schema (lemma, category, forms, reject, waive, note) plus a required
  `definition` in minglish. Lexgen merges core + domain into one lexicon.
  The pack **adds, never overrides**: a domain lemma may not repeat a core
  lemma of the same category. `NAME` entries (Minglish, Lexgen, Triage,
  WordNet) carry a definition and no forms.
- **Noun terms are written Capitalized**, single or multi-word, and lex as
  one unit with number: "the Linter bans Anaphoric Pronouns", "every
  Surface Form has one Form Tag". A capitalized word not in the model is a
  proper name under ADR 0018's fail-loud rules. Verbs and adjectives from
  the pack stay lowercase. A defined term written lowercase gets advice.
- **Definitions are used**: the check script self-lints every definition;
  the repair-loop system prompt lists every term with its definition;
  `just define <Term>` prints one; **CONTEXT.md is generated** from the
  model and drift-checked.
- **Migration**: the glossary terms and the project jargon that had
  accumulated in the seed moved to the pack (24 nouns, 4 names, 5 verbs,
  4 adjectives); the corpus, the ADR 0001 rewrite, the skill examples, the
  showcase and the linter advice were recapitalized. A definition
  replaces reference-data attestation for a term (a coinage is by
  definition unattested); collision and cross-POS lints still apply.

## Consequences

- The model sees one unit with a meaning; the repair loop can stop
  paraphrasing project concepts. Readers get a visible signal that a word
  is defined here, and one place to look it up.
- Capitalization carries meaning, so the lowercase-only orthography of
  ADR 0018 now has two exceptions: names and terms. Sentence-initial
  terms are unambiguous because they are lexicon surfaces.
- Definitions are prose claims about the project and get the same ADR
  0012 review as translation pairs.
- Deferred: term verbs/adjectives with a visible signal; packs for other
  domains; a term used both as a name and as a noun (*Triage* the tool vs
  a triage).
