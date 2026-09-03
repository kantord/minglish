# Language gaps found by full-corpus rewriting (2026-09-03)

Every ADR (0001–0036) and the domain model were rewritten for naturalness
by blind sub-agents (docs/prejudge.md, "Whole documents"), each restricted
to the current lexicon and grammar. Each rewriter logged the words and
constructions it reached for and could not have. This is the ranked
tally over all 36 gap notes — the next real signal for what to add to the
language, sharper than the archetype review because it comes from writers
hitting a wall while trying to say a true thing, not from reading examples.

## Constructions (highest-value; block a whole shape, not one word)

- ~~**Clause-level "but"**~~ (10 rewrites wanted it). **Closed
  2026-09-03, ADR 0037.** "but" used to only coordinate inside a VP (ADR
  0021); a full contrastive clause ("X, but Y") now has a shape (a comma
  marks the new clause), the single most requested fix.
- ~~**Coordination inside a causal or conditional clause**~~ (8+
  rewrites). **Closed 2026-09-04, ADR 0038.** "the test fails and the
  agent retries, so …" used to reject; a Conditional and a causal
  sentence now allow a Coordination in every clause, both shapes: comma-
  free for a shared subject, comma-mandatory for a new one, same rule as
  ADR 0037's top-level Coordination. Zero new LALR conflicts — the
  decision point sits behind the unique "if"/"so"/"because" markers, not
  the shared `Clause` reduction that caused ADR 0037's conflict.
- ~~**"without"**~~ (7). **Closed 2026-09-04, ADR 0039.** A plain
  `PREP_V` add, the exact slot "with" already had (ADR 0011) — no grammar
  change. All 7 original wanted sentences now parse verbatim.
- **Coordinated noun phrases as subject/object** (ADR 0004 already fences
  this; the top open construction gap now that "but", the causal/
  conditional gap, and "without" are closed).

## High-frequency missing words

- **"only"** (5) — deferred by design (ADR queue item); this data adds
  weight to prioritizing it.
- **"other"** (4), **"or"** as a full alternative in more positions (3).
- **Temporal/hedge adverbs**: "yet", "still", "already", "later", "never"
  (3 each) — writers reach for these constantly when describing an ADR's
  history ("not yet allowed", "still applies", "already deferred").
- **"at most"** (2) — "most" is banned (ADR 0029 superlative deferral);
  "at most N" has no phrasing and gets rewritten around the count instead.
- **Superlatives** ("largest", "shortest", "biggest") — known deferral
  (ADR 0029); this run shows it costing real content (a dropped ranking
  in ADR 0002, a lost "second-largest" in the same file).
- **"change"/"tier"/"trade"/"toil"/"trigger" as nouns** — several
  ADR-specific technical nouns exist only as verbs or not at all; each
  rewrite improvised a paraphrase (see the per-ADR notes in git history
  of this file's companion runs, not kept — request a rerun of `just
  judge-docs failing` if the detail is needed again).

## Reading this list

This is a frequency count from one rewrite pass, not a design decision.
Treat it as the input to the next `/grill-with-docs` round: clause-level
"but" and causal/conditional-internal coordination are the two items
most likely to unblock the largest number of paragraphs at once.
