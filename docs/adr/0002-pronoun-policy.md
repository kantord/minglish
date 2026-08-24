# 0002 — Pronouns: third person banned, indexicals allowed

Date: 2026-08-31
Status: proposed (tentative — revisit once real corpus experience exists)

## Context

Triage against UD-EWT (docs/triage-report.md) shows pronouns are ~2,200 of
the OOV tokens — the second-largest closed-class gap. But third-person
pronouns are anaphoric: resolving "it failed" requires a discourse layer, and
our research findings identify cross-sentence reference as exactly where
ambiguity reappears after sentences are split
(docs/research/cnl-design-findings.md, "Ambiguity migrates"). First- and
second-person pronouns are indexical (speaker/hearer), never anaphoric, and
carry no reference ambiguity; instructional text uses them constantly.

## Decision

- Banned: all anaphoric pronouns and pronominal demonstratives (*it, they,
  he, she, this, that, these, those*, and their object/possessive forms).
  The writer repeats the noun instead.
- Allowed: *i* (PRON_1SG), *you* (PRON_2), *my* (POSS_1SG), *your* (POSS_2).
- No anaphora/discourse machinery exists or is planned for v0.

## Consequences

- Zero reference ambiguity by construction; no discourse layer to build.
- Prose is repetitive ("the agent … the agent …") — accepted cost, consistent
  with the project's clarity-over-naturalness stance.
- Banned pronouns currently just fall out of the lexicon (OOV). A
  writer-facing "repeat the noun you mean" error message needs a
  banned-word-with-message mechanism in the future validator; deferred.
