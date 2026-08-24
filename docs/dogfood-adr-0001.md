# Dogfood: translating ADR 0001 into minglish

Sentence-by-sentence attempt to render our own first ADR in minglish
(2026-09-01, lexicon at 47+ lemmas, tier-1 grammar). Faithful translations
join `corpus/pairs.tsv`; blockers carry the reason codes from
`corpus/untranslatable.tsv`. Retroactive rewriting of the ADR itself is
sanctioned only when lossless — which, as the tally shows, is not yet
reachable for most of it.

Before/after previews with costs and parse guarantees for the faithful
pairs: **docs/dogfood-cost-report.md** (source: corpus/dogfood-pairs.tsv).

## Sentence audit

| ADR 0001 sentence (abridged) | verdict |
|---|---|
| "minglish needs a lexicon: every allowed surface form, each with exactly one form-tag…" | BLOCKED — QUANTIFIER (*every, each, exactly one*), COMPOUND (*form-tag*) |
| "Word choice must stay a human judgment." | ✓ translated (post-ADR 0013) — "people must choose words", drops: nominalization |
| "…exactly the kind of check humans miss and machines don't." | ✓ translated (post-ADR 0013) — "people do not find collisions. machines find collisions.", drops: nominalization |
| "A hand-edited seed/seed.json is the single source of truth." | ✓ translated (post-ADR 0018) — "people edit the file \"seed.json\". the file \"seed.json\" is the source of the truth.", drops: emphasis |
| "Both generated files are committed; they are…never hand-edited." | ✓ resolved by in-place rewrite (dogfood case adr0001-01): the flag was correct about the prose; the rewrite is itself valid minglish |
| "Reference data is used only for checking — never for choosing words." | ✓ resolved by in-place rewrite (case adr0001-03); only/never expressed as affirmative + generic-negation pair |
| "Curation stays human and cheap: one JSON entry per word…" | ✓ resolved by in-place rewrite (case adr0001-02); all rewrite sentences valid minglish |
| "Every curation commit shows its effect in the diffs." | ✓ translated (post-ADR 0014/0015) — "every commit shows the effect of the commit in the diffs", drops: nominalization |

## What technical prose needs, by blocking frequency

0. **GENERIC** — generic reference (bare plurals: *humans, machines*;
   process nominals: *word choice*). Mandatory determiners removed English's
   genericity device, so every translation attempt forces a false definite
   or an ambiguous indefinite. Caught only by human review — no metric sees
   semantic reference ambiguity. Kin to QUANTIFIER (both are quantification).
1. **QUANTIFIER** (*every, each, both, only, single, exactly one*) — in 5 of 8
   sentences. Now the top-blocking feature across every corpus we have.
2. **COMPOUND** (noun-noun: *form-tag, seed list, reference data, build
   output*) — 4 of 8. Technical prose is compound-dense; the research
   options were ban-and-rephrase (of-genitives) or hyphenate-as-lexical-entry.
3. **NOMINALIZATION** (*curation, word choice, judgment*) — verbal recast is
   the right direction, but recasting a generic nominal drags in the GENERIC
   problem above; safe only once genericity has a sanctioned form.
4. **PASSIVE / agentless statements**, **ANAPHORA** (*they, its*),
   **adverbs of quantification** (*never, only*) — the familiar residue.

## Verdict

**7 of 8 sentences resolved** (4 translated, 3 by in-place source rewrite) (progression: 2 → 0 after the
generic-reference retraction → 2 after ADR 0013 → 4 after names (ADR 0018),
every/one, and the transparency policy unblocked the seed.json and
curation-commit rows). **Reframe (2026-09-01): the goal is NOT to translate arbitrary English.**
minglish parses well-written text and flags problems in text that is not —
so each blocked row must first be judged: is the rejection the linter being
*right* about the prose? Verdicts on the remaining 4: the both/they/passive
semicolon splice and the "curation stays human and cheap" compression are
**legitimate style flags — the source sentences should be rewritten in
English first**, then translated. *never* already has a sanctioned
equivalent (generic negation: "people do not edit these files"). The only
residue that survives as a genuine gap candidate is ***only*** (a precision
device — though placement-ambiguous in English, so its design needs its own
interview). New language features require a *well-written* rejected
sentence as evidence, not coverage of the original prose. Self-hosting also exposed a blind
spot in the tooling itself: structural-ambiguity guarantees and cost metrics
are silent about semantic reference ambiguity — that check is human (or a
future semantic layer), and ADR 0012 review is where it happens.
