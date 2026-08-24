# 0019 — Bare imperatives: verb-first is the command signature

Date: 2026-09-01
Status: proposed (tentative)

## Context

agenttest run 4's only failing case (0/3 first-try) was an imperative with
an OOV verb ("Remove the file"): the sentence-initial-capital error assumed
a name and never mentioned the command reading, and the language itself had
an indefensible asymmetry — "do not delete the file" was legal while
"delete the file" was not. Commands are the core register of the
agent-instruction domain.

## Decision

- Sanctioned: **`<VERB_BASE> …`** as a sentence form (the positive twin of
  the prohibition): "delete the file", "check the input of the user".
- No explicit marker is needed: **the verb is the signature.** The
  one-tag-per-surface invariant guarantees no first token is both a verb
  and anything else, so verb-first can only be a command — the first-token
  telegraph gains `VERB_BASE → imperative` with zero possible collisions,
  matching the standard English convention at zero token cost.
- Addressee, by fiat: **the reader** — the same indexical status as *you*
  (ADR 0002), so no reference ambiguity in the normal case. Directed
  commands (naming which agent acts) are a deferred **vocative** design,
  not covered here.
- The sentence-initial-capital error message now offers both readings:
  name introduction/quoting, or the imperative forms.

## Consequences

- Instructions read naturally instead of being laundered into "you must"
  obligations (a deontic strengthening the repair pairs kept committing).
- Tier-2 already anticipated the shape; the showcase repair pairs and the
  agenttest "Remove the file" case get honest target forms.
- Multi-agent directed commands wait on the vocative design.
