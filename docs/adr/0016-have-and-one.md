# 0016 — have (possession) and one (exactly-one numeral)

Date: 2026-09-01
Status: proposed (tentative)

## Context

"every word has one tag" — the ADR 0015 unfolding of the rejected *form-tag*
compound — was blocked on *has* and *one*. *have* is also ×335 in general
triage. English *have* is dangerous only as an auxiliary (perfect aspect);
English *one* is dangerous only as a pronoun ("a good one", generic "one
must"). Both dangerous halves are already unwritable in the grammar.

## Decision

- **have / has / had** — ordinary VERB_TRANS, possession/attribution only.
  The auxiliary reading needs no ban: with no participle constructions in
  the grammar, "the agent has deleted the file" fails to parse by
  construction (*has* wants an NP object). "have to" is likewise unwritable
  (no infinitives). Same safety-by-construction pattern as ADR 0010.
- **one** — NUM_SG, determiner position only, singular nouns; fiat meaning
  **exactly one**. Distinct from *a/an* (plain at-least-one indefinite).
  Pronoun *one* stays out. Opens the numeral pattern (*two/three* + plural
  as NUM_PL) without committing to it.

## Consequences

- "every word has one tag" parses; possession statements unlock a large
  class of attribute prose.
- The dogfood sentence about form-tags remains only *approximately*
  expressible: "surface form" vs "word" is a real distinction in our own
  glossary, and collapsing it is propositional loss under ADR 0012. The
  faithful version waits on a transparent rendering of "surface form".
- If participles or infinitives are ever admitted, the have-auxiliary and
  have-to readings must be re-fenced explicitly (same revisit trigger as
  ADR 0010's).
