# Language gaps found by full-corpus rewriting (2026-09-03)

Every ADR (0001–0036) and the domain model were rewritten for naturalness
by blind sub-agents (docs/prejudge.md, "Whole documents"), each restricted
to the current lexicon and grammar. Each rewriter logged the words and
constructions it reached for and could not have. This is the ranked
tally over all 36 gap notes — the next real signal for what to add to the
language, sharper than the archetype review because it comes from writers
hitting a wall while trying to say a true thing, not from reading examples.

## Constructions (highest-value; block a whole shape, not one word)

- **Clause-level "but"** (10 rewrites wanted it). "but" only coordinates
  inside a VP (ADR 0021); a full contrastive clause ("X, but Y") has no
  shape and gets forced into 2 sentences or an inline-list misreading.
  The single most requested fix.
- **Coordination inside a causal or conditional clause** (8+ rewrites).
  "the test fails and the agent retries, so …" does not parse; the clause
  after "so"/"because"/"if" cannot itself coordinate. Forces artificial
  sentence splits inside what is naturally one reason or one consequent.
- **"without"** (7). No way to state an absence attached to a clause
  ("split the sentence without an ambiguity"); always rerouted through a
  negated clause or a Conditional, at a naturalness cost.
- **Coordinated noun phrases as subject/object** (ADR 0004 already fences
  this; still the top workaround-forcing gap after the two above).

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
