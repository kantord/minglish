# 0017 — some: the existential, completing the quantification square

Date: 2026-09-01
Status: proposed (tentative)

## Context

With generic (bare plural, ADR 0013), universal (*every*) and universal
negative (*no*, ADR 0014) in place, ¬∀ ("not all") still had no home — it
was deferred from ADR 0014 precisely to here.

## Decision

- **some** (QUANT_EXIST) + **plural** noun, **subject position only**, first
  token = existential signature: "some agents retry the request" (at least
  one, possibly more).
- **Negated predicates are allowed** under *some* — "some agents do not
  retry the request" is unambiguous in English (¬ can only scope under ∃),
  and it is exactly ¬∀. The quantification square is complete:
  ∀ *every* · ¬∃ *no* · ∃ *some* · ¬∀ *some … not* · generic bare plural.
- Fences: *some* + singular banned (English "some agent" drifts to the
  unknown-identity reading — a different meaning); the "approximately"
  reading ("some twenty files") never enters; object position banned (same
  buried-operator argument as *no*, ADR 0014).

## Consequences

- All four corners of quantification have exactly one form each, and each
  quantified sentence type announces itself in its first token.
- *some* takes the full plural predicate set including negation — unlike
  *every*/*no*, whose positive-only restriction stays justified by their
  genuine scope ambiguity with negation.
