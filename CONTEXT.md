# minglish — Ubiquitous Language

Glossary of domain terms. Terms not listed here are undefined; propose an
entry before using one in code or docs.

## Terms

**Surface form** — an exact spelling as it appears in text (`reviews`,
`reviewed`). The unit the lexicon and lexer operate on. Distinct from lemma.

**Lemma** — the dictionary form of a word (`review`). Humans curate lemmas;
machines expand them into surface forms.

**Category** — the seed-level syntactic class a curator assigns to a lemma.
v0 open classes: `NOUN`, `VERB_TRANS`, `VERB_INTRANS`, `ADJ`, `PREP`, `DET`.
Closed-class function words carry individual fiat categories
(`COMPLEMENTIZER`, `RELATIVE`, `CONJ`, `NEG`, …) and do not inflect.
Ditransitive verbs are expressed as `VERB_TRANS` + prepositional phrase.

**Form-tag** — the tag a *surface form* carries in the lexicon, derived by the
generator from the category (`NOUN` → `NOUN_SG`/`NOUN_PL`; `VERB_TRANS` →
`VERB_TRANS_BASE`/`_3SG`/`_ED`/`_ING`). Every enabled surface form has exactly
one form-tag. Form-tags encode *shape*, not grammatical role: syncretic forms
(`reviewed` = past and participle) get one form-tag (`_ED`); role resolution
is a grammar-tier concern, not a lexicon concern.

**Seed list** — the hand-curated, human-edited source of truth: which lemmas
are enabled, each with its intended tag. The only file a curator edits.

**Lexicon** — the generated, committed artifact (`lexicon.tsv`): every enabled
surface form with its tag and redirects. A build output — never hand-edited.

**Paradigm** — the full set of inflected surface forms generated from one
lemma + tag (e.g. `review → review, reviews, reviewed, reviewing`).

**Collision** — a surface form that would carry more than one tag (within one
lemma's paradigm or across lemmas, e.g. `leaves` = leaf+PL and leave+3SG).
Collisions are build errors; the lexicon must contain none.

**Redirect** — a lexicon entry for a *rejected* use of an enabled surface
form: the banned syntactic category plus a suggested replacement word
(`present`: VERB accepted; NOUN → "gift"). Powers precise writer-facing
errors.

**Coverage** — v0: the share of seed-corpus sentences whose every token is in
the lexicon with the needed category. This is a proxy; the real target —
"when a word is rejected, the suggested alternative is clear and useful" —
is not yet mechanically measurable and must not be conflated with it.

**Rejection** — a sentence failing to lex or parse. Not a defect: minglish
is defined by what it rejects, and the one-parse guarantee exists because of
it. Three kinds, with different owners: **ban** (ambiguous or hostile
structure; permanent; the writer rephrases — the red flag is the product),
**gap** (fine sentence the language cannot say yet; the fix is curation or
an ADR), and **load warning** (future: parses but exceeds a calibrated
bound). A rejection message must say which kind and why; unexplained
rejection is the failure mode, not rejection itself.

**Translation pair** — an English sentence and its hand-made minglish
rendering (`corpus/pairs.tsv`). Valid only if the minglish side is
lexicon-legal, parses in tier 1, and preserves propositional content.

**Declared loss** — the pair's explicit record of what the rendering gave up
(politeness, emphasis, topic structure, …). Propositional loss is never
declarable — it invalidates the pair. Undeclared loss found later is a
corpus bug.

**Linter** — the set of checks the generator runs: paradigm collision
detection, cross-POS ambiguity lookup against reference data, and license-safe
data provenance.
