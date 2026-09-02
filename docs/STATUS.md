# Project status and handoff

Last updated: 2026-09-01. Everything a fresh agent needs that is not
derivable from the code, ADRs, or git history.

## Where everything lives

- `domain/model.json` — the domain model (ADR 0027): every project term with
  its minglish definition; `CONTEXT.md` is generated from it. Noun terms are
  written Capitalized in minglish text. `just define <Term>` looks one up.
- `docs/adr/0001–0028` — every language and policy decision, each citing its
  evidence. 0006 (comprehension-first + density + expressiveness-subordinate
  + enforcement hierarchy), 0008 (redirect vs ban) as amended by 0023
  (per-sense synonyms, absolute findability floor), 0012 (loss taxonomy),
  0015 (transparency-first vocabulary) are the governing ones.
- `docs/research/cnl-design-findings.md` — empirical findings, including five
  documented cases where a metric was optimized at the cost of meaning or
  quality; read the relevant entries before treating any metric as a target.
- `tests/dogfood-cases/` — the standard dogfood flow (README there) and open
  gap cases. `tests/agent-cases/` — LLM repair snapshots with human verdicts.
- `justfile` — check / showcase / lint / agenttest / autofix /
  autofix-paragraphs (paragraph repair flow, `tests/paragraph-cases/`) targets.
- `docs/review-checklist.md` — known failure types to check in every
  evaluation.
- `scripts/lint-file.py` / `just lint-file <md>` — document-level lint:
  per-sentence verdicts, topic continuity, relation inventory. `just
  coherence` regenerates `docs/coherence-report.md` over all ADRs.

## Open decision queue (in order, with standing recommendations)

1. **Coherence relations beyond cause** (exemplification, sequence,
   concession, purpose, elaboration) and cross-sentence relations: decide
   from `docs/coherence-report.md` (relation inventory + topic continuity,
   `just coherence`), not from word counts. Next: the paragraph repair flow
   (agreed design in docs/ideas.md, "Paragraph repair").
2. **only** (gap case adr0001-04) — real but rare (absent from sweep top-30);
   deprioritized by data.
3. Parked with design notes in `docs/ideas.md`: vocative (directed
   imperatives), quotation-as-mention, pseudocode/analysis-code compiler,
   embeddings-as-measurement, core+jargon-packs split, LM-based cost model,
   linter advice gap #2 (rejected-verb redirects dormant on inflected forms:
   "the agent files the report" gives no submit-suggestion).
4. Structured repair (docs/ideas.md, "Structured repair"): structure
   enumerator → role assignment → table-driven rewrite with explanations;
   NLI as the future faithfulness gate. Steps 1–3 need no model and no API
   spend; the first candidate for build work after the vocabulary queue.
5. Deferred by ADR 0022 (numbers, decided 2026-09-01): measurement values
   ("the exit code is 0" — the natural home for a future 0), ordinals,
   units, thousands separators, decimals. Needs a value slot distinct from
   the NUM_PL count slot.
6. Deferred by ADR 0023 (same): the named-standard form "identical to the
   report" needs an adjective + PP complement the copula lacks.
7. Deferred by ADR 0024 (percent): percent of a singular mass, percentages as
   predicates, decimals. Ambitransitives ("the process stops") remain a
   findings-level gap with no case attached; commit-as-verb (adr0001-05) was
   resolved by the linter's own redirect (*save*).

## Working process (session-established, not in any ADR)

- Language decisions go through a grilling interview (options + recommendation,
  user decides), then land as: tentative ADR + seed/grammar change + corpus
  sentences + reject tests + regenerated reports, all in one commit.
- `./scripts/check.sh` must pass before commits; it regenerates everything
  and drift-checks committed artifacts. It includes the self-lint tests:
  every example sentence in the skill, in linter advice, and in ban advice
  must itself parse — never feed the model an invalid example.
- The user runs all `git commit`s and all API-spending runs (agenttest,
  autofix) themselves; those runs are milestones, not routine.
- Between milestones, `just replay` is the proxy: it re-validates every
  stored paragraph proposal (tests/paragraph-cases/) against the current
  linter, no API. A linter change should raise valid counts and lower the
  no-advice count; a language change that lowers them needs a reason.
- Every review of linter output, rewrites, or agent snapshots goes through
  `docs/review-checklist.md` (known failure types; append, never delete).
- Dogfood flow: lint own docs → useful flag → rewrite in place (English
  quality first — parse-validity is NEVER the rewrite target; well-written
  but unparseable = gap evidence) → else file gap case.

## Metrics snapshot (2026-09-01)

- Lexicon 652 forms incl. 24 Capitalized term nouns (ADR 0027); corpus 70/70
  parse; grammar LR(1)-clean, zero
  precedence declarations (CI-enforced).
- agenttest first-try trajectory: 44 → 51 → 48 → 52 of 54 (run 5). Predictions registered for run 6: "please delete the file" should
  snapshot to the bare imperative (skill now teaches it); the no-must case
  should stop dropping the deontic (advice got a concrete example);
  autofix rerun on ADR 0001 should yield real proposals + GAP declarations
  (extractor fixed, refusal-routing strengthened).
- ADR sweep: see docs/dogfood-sweep.md (regenerated by `python3 scripts/dogfood-sweep.py`).
  Fully minglish: ADR 0001–0009 (docs/dogfood-adr-000N.md each). 0002 was rewritten
  by hand after nine paragraph-repair runs whose findings became ADR 0027 and 0028;
  0003–0009 with no model run at all (lint-file + fix-gap-or-follow-advice).
- Lexicon 1133 forms, 70 domain terms; `just replay`: see the last run below. Expected: ADRs are meta-linguistic
  register, and coverage of them is a non-target (see the reframe in
  docs/dogfood-adr-0001.md: coverage of arbitrary English is a non-goal).
- Dogfood ADR 0001: every sentence parses (in-place rewrite, 15 declared
  pairs in corpus/dogfood-pairs.tsv); cost ratio in docs/dogfood-cost-report.md
  (understated by the nominalization blind spot — see findings).

## Standing policies easy to violate accidentally

- No references to (or derivation from) non-open controlled-language
  specifications anywhere in the repo — the anonymous policy note in
  docs/resources.md is deliberate; do not name them even to warn.
- CC BY-SA data (UD-EWT, wordfreq table) is fetched, never vendored
  (scripts/fetch-data.sh); tracked content stays MIT/Apache + permissive.
- The dogfood curation phase is our-own-jargon-first (docs/ideas.md,
  "Curation phasing"); EWT triage numbers are telemetry, not targets.
