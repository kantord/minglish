# Project status and handoff

Last updated: 2026-09-01. Everything a fresh agent needs that is not
derivable from the code, ADRs, or git history.

## Where everything lives

- `CONTEXT.md` — glossary (authoritative vocabulary; "Rejection" defines the
  ban/gap/load-warning triage that governs all linting work).
- `docs/adr/0001–0021` — every language and policy decision, each citing its
  evidence. 0006 (comprehension-first + density + expressiveness-subordinate
  + enforcement hierarchy), 0008 (redirect vs ban), 0012 (loss taxonomy),
  0015 (transparency-first vocabulary) are the governing ones.
- `docs/research/cnl-design-findings.md` — empirical findings, including five
  documented cases where a metric was optimized at the cost of meaning or
  quality; read the relevant entries before treating any metric as a target.
- `tests/dogfood-cases/` — the standard dogfood flow (README there) and open
  gap cases. `tests/agent-cases/` — LLM repair snapshots with human verdicts.
- `justfile` — check / showcase / lint / agenttest / autofix targets.

## Open decision queue (in order, with standing recommendations)

1. **Numbers** (pending mid-interview): recommend digits as an open lexer
   class (NUM_PL + plural noun: "3 files"), *one* stays word-form
   (exactly-one fiat), digit-0 banned in favor of *no*, ordinals/units
   deferred. Alternatives (number words / both forms) rejected for treadmill
   and one-meaning-one-form reasons. Awaiting user confirmation.
2. **same** — CAUTION from discussion: "the same X" is anaphoric-adjacent
   (same as *what*?), so it is NOT a clean vocabulary add; needs its own
   interview with the reference problem front and center.
3. **commit-as-verb** (gap case adr0001-05) — the one-tag noun/verb tension;
   same family as ambitransitives (see findings: "the process stops"
   inexpressible because *stop* is VERB_TRANS only).
4. **only** (gap case adr0001-04) — real but rare (absent from sweep top-30);
   deprioritized by data.
5. Parked with design notes in `docs/ideas.md`: vocative (directed
   imperatives), quotation-as-mention, pseudocode/analysis-code compiler,
   embeddings-as-measurement, core+jargon-packs split, LM-based cost model,
   linter advice gap #2 (rejected-verb redirects dormant on inflected forms:
   "the agent files the report" gives no submit-suggestion).

## Working process (session-established, not in any ADR)

- Language decisions go through a grilling interview (options + recommendation,
  user decides), then land as: tentative ADR + seed/grammar change + corpus
  sentences + reject tests + regenerated reports, all in one commit.
- `./scripts/check.sh` must pass before commits; it regenerates everything
  and drift-checks committed artifacts.
- The user runs all `git commit`s and all API-spending runs (agenttest,
  autofix) themselves; those runs are milestones, not routine.
- Dogfood flow: lint own docs → useful flag → rewrite in place (English
  quality first — parse-validity is NEVER the rewrite target; well-written
  but unparseable = gap evidence) → else file gap case.

## Metrics snapshot (2026-09-01)

- Lexicon 224 forms; corpus 60/60 parse; grammar LR(1)-clean, zero
  precedence declarations (CI-enforced).
- agenttest first-try trajectory: 44 → 51 → 48 → 52 of 54 (run 5). Predictions registered for run 6: "please delete the file" should
  snapshot to the bare imperative (skill now teaches it); the no-must case
  should stop dropping the deontic (advice got a concrete example);
  autofix rerun on ADR 0001 should yield real proposals + GAP declarations
  (extractor fixed, refusal-routing strengthened).
- ADR sweep: 3/287 sentences parse. Expected: ADRs are meta-linguistic
  register, and coverage of them is a non-target (see the reframe in
  docs/dogfood-adr-0001.md: coverage of arbitrary English is a non-goal).
- Dogfood ADR 0001: 7 of 8 audited sentences resolved (4 translated, 3 by
  in-place rewrite); dogfood pairs cost ratio ≈1.08 (understated by the
  nominalization blind spot — see findings).

## Standing policies easy to violate accidentally

- No references to (or derivation from) non-open controlled-language
  specifications anywhere in the repo — the anonymous policy note in
  docs/resources.md is deliberate; do not name them even to warn.
- CC BY-SA data (UD-EWT, wordfreq table) is fetched, never vendored
  (scripts/fetch-data.sh); tracked content stays MIT/Apache + permissive.
- The dogfood curation phase is our-own-jargon-first (docs/ideas.md,
  "Curation phasing"); EWT triage numbers are telemetry, not targets.
