# 0020 — we/our/us banned: name the group

Date: 2026-09-01
Status: proposed (tentative)

## Context

*we/our* rank high among general-word blockers in the ADR sweep and the
filed gap case. But first-person plural is not an indexical in the ADR 0002
sense: *I* and *you* are resolved by the speech situation alone, while *we*
carries two unresolved ambiguities — inclusive vs. exclusive of the reader,
and group extent (the authors? the maintainers? the project? any
contributor?). "We commit the generated files" hides who acts the way a
passive hides its agent. A fiat reading ("we = the authors") would silently
change what technical *we* usually means.

## Decision

- **Banned**: *we, our, us*, with advice: name the group — "the
  maintainers", "the team", "the project" — or use an imperative/generic.
  Forcing the "who acts?" question is the product working: a policy document
  should answer it.
- Consequential rewrite applied to ADR 0001's own sentences ("We commit…" →
  "The maintainers commit…").
- Revisit trigger: evidence that a genuinely irreducible speaker-group
  reference exists in well-written target text (none seen yet — every
  observed *we* had a nameable group).

## Consequences

- Slight naturalness cost (ADR 0006 §5) accepted for referential precision.
- *maintainer* joins the lexicon as the canonical example replacement.
- The adr0001-05 gap case narrows to its other half (commit-as-verb).
