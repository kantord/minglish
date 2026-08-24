# 0008 — Redirect only when the fix is findable; otherwise ban

Date: 2026-08-31
Status: accepted (curation policy)

## Context

A rejected word sense can either carry a redirect ("use X instead") or be
banned outright (the writer must rephrase). Our research showed mechanical
substitution optimizes rarity, not clarity: suggestions like
*need*→*necessitate* (1,349× rarer) are fixes no human or LLM would
naturally produce. An unactionable lint message is worse than a blunt one —
the writer stalls, or the LLM emits stilted low-frequency vocabulary, and
the output quality the whole system exists for degrades.

## Decision

When curating a rejected sense:

1. **Redirect** only when the suggestion is a word the writer would
   plausibly have chosen themselves — common (small zipf gap to the rejected
   word), and a genuine synonym of the rejected sense.
2. **Ban** (no substitute offered; error says to rephrase) when no such word
   exists. Forcing a sentence-level rephrase is more productive than
   coercing an unfindable word.
3. The existing report-level frequency guard (zipf delta > 1.0) is the
   trigger for review: a flagged redirect should usually be demoted to a ban
   or given a better suggestion, not waved through.

Same logic applies to whole words: a word whose every sense would need an
unfindable substitute is banned entirely rather than redirected.

## Consequences

- Redirects stay few and high-quality; the error channel stays trustworthy.
- The future validator needs a "banned — rephrase" message type distinct
  from "use X instead" (currently approximated by `waive`).
- Ambiguous modals (e.g. *may*: permission vs possibility) become ban
  candidates, since neither sense has a common one-word substitute.
