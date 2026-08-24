# 0010 — Tense: simple past enabled; aspect and future stay out

Date: 2026-08-31
Status: proposed (tentative). Revises ADR 0003 (adds *was/were*) and
ADR 0005 (adds *did*).

## Context

Tense is the best-evidenced structural gap: TENSE ×5 (+ASPECT ×4) in the
sampled-web-text inventory (corpus/untranslatable.tsv), and two translation
pairs had to flatten past to present — silent meaning loss. The classic
danger of *-ed* forms — the reduced-relative garden path ("the file stored
in the cache…") — is already structurally excluded: v0 has no passives and
no participle constructions (ADR 0003), so an *-ed* form can only be a
finite past verb. The `_ED` surfaces already exist in the lexicon via full
paradigms.

## Decision

- Simple past is part of minglish: lexical verbs use their `_ED` forms.
- Closed-class additions: *was* (COPULA_SG_PAST), *were* (COPULA_PL_PAST),
  *did* (NEG_AUX_PAST — past negation carrier: "did not delete").
- Still out: future (*will* deferred), perfect and progressive aspect
  (*have/be* + participle machinery would reopen the reduced-relative and
  passive doors), *could/might/would* (counterfactuals), contractions.
- Same fencing obligations as the present forms: *was/were* copular only,
  *did* negation-carrier only.

## Consequences

- Reports of past events become faithfully expressible; the pairs corpus no
  longer needs to cheat on tense.
- Epistemic/aspectual nuance ("has been failing") remains inexpressible —
  accepted, revisit only with corpus evidence.
- The *-ed*-unambiguity argument depends on passives/participles staying
  banned; if a future ADR admits them, this ADR must be revisited first.
