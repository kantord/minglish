# 0012 — Translation-pair validity: loss taxonomy, declared not silent

Date: 2026-09-01
Status: accepted (corpus methodology policy)

## Context

Auditing corpus/pairs.tsv showed three pairs changed propositional content
(a dropped universal quantifier; a recast causal claim; an invented agent) —
and the cost metric rewarded them, because text that sheds meaning is cheap.
Cheaper-by-omission is the failure mode a density goal invites; it must be
structurally impossible for it to pass silently.

## Decision

Three tiers of information loss, judged per pair:

1. **Propositional loss or change** — quantifiers, scope, causal structure,
   invented or dropped participants, tense/eventuality that alters the
   claim. **Never acceptable.** The pair is fixed, or moved to
   untranslatable.tsv with the blocking reason. A translation that drops
   "all" is wrong, not dense.
2. **Register/affect loss** — politeness markers, emphasis, emoji, discourse
   links. **Tolerated but regrettable** (per ADR 0006 §5 expressiveness is a
   subordinate goal): every such loss is declared in the pair's third
   column; undeclared loss found later is a corpus bug. Prefer translations
   that keep affect when criteria 1–4 permit.
3. **Information-structure change** — e.g. passive→active topic shift forced
   by v0's lack of passives. Acceptable, declared.

Format: `pairs.tsv` gains a third column, a comma-separated `drops` list
(empty/absent = lossless). textcost prints the declarations beside each
before/after entry, so the cost ratio is always read alongside what it cost
in meaning.

## Consequences

- Three pairs moved to untranslatable.tsv (QUANTIFIER; PASSIVE-agent ×2) —
  the aggregate cost ratio worsens and becomes honest.
- The impersonal→addressee narrowing ("it is not possible" → "you cannot")
  is classified tier 2 and declared, reviewable later.
- The parked embedding-similarity guard (docs/ideas.md) becomes the
  automation candidate for catching undeclared tier-1 loss.
