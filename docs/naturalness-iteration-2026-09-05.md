# Naturalness iteration, round 2 (2026-09-05): a grammar construction, tested and mostly falsified

Continuation of `docs/prompt-ab.md` (same day, same session). That
round showed naturalness is not moved by system-prompt wording. This
round asks the next question in `docs/readiness.md`'s plan: is it
moved by a *grammar* construction instead? Short answer: **no, not by
itself** — a real, valuable finding, but not the win originally hoped
for. Full method and every judged trial are in the session transcript;
this file is the durable record.

## What was built

`docs/judge-report.md`'s failure notes were read directly (not
re-guessed) to rank failure shapes by real frequency. The dominant
shape, by a wide margin, is a **run of 3+ consecutive sentences that
share a subject-verb template but not the actual subject** — e.g.
`docs/adr/0014-universal-and-no.md`'s "The word 'if' opens a
Conditional. The word 'do' opens a Prohibition. The word 'no' opens a
universal Negation..." (7 sentences in a row, same template).

The grammar had no way to combine 3+ *different-subject* clauses into
one sentence — confirmed empirically (`just lint` rejects both the
Oxford-comma and pure `and`-chain forms). ADR 0037 gave a binary
(exactly-2) shape; extending it seemed like the obvious fix.

**ADR 0050** (`docs/adr/0050-nary-clause-coordination.md`, itself
100% dogfooded minglish) adds a 3-clause Oxford-comma shape to
`crates/grammar/src/minglish.lalrpop`'s `CoordClause` rule. Built as
explicit bounded alternatives (2-ary, 3-ary), not a generic `+`
repetition macro — the macro version produced a genuine LALR(1)
shift/reduce conflict with the existing Causal/Conditional lookahead
discipline (see "Controversial and unvalidated decisions" below).
Verified: clean build, all existing tests pass, `./scripts/check.sh`
clean, `dogfood-sweep`/`parse-report` numbers unchanged from before
the grammar edit (confirmed by `git stash`-comparing the exact same
sweep with and without the change).

## What was tested, and what it showed

`docs/adr/0014-universal-and-no.md`'s 7-sentence run was rewritten
using the new construction (7 sentences → 3, one of them a genuine
3-way coordination). A blind naturalness judge scored **the original
2/5** and **the rewrite 2/5** — no change. The judge's complaint just
moved: from "seven consecutive sentences, same template" to "three-item
list with identical template... reads like an enumerated spec table
flattened into prose."

A third version, varying the verb per clause (opens/marks/starts/
announces — no new grammar, pure word choice) *also* scored 2/5: "feels
like deliberate synonym-cycling rather than natural word choice."

A fourth attempt tried reformatting the same content as a real
Enumeration block (ADR 0028, zero new grammar) instead of any kind of
sentence. It doesn't fit: Enumeration items must be a single noun
phrase, and this content is a token→Sentence-Shape *mapping* — there
is no construction in minglish for an associative list, only flat
membership lists. That gap is real but a materially bigger project
than this round's scope (a genuine 2-column/table-shaped block, not a
one-line grammar tweak).

**Combined with `docs/prompt-ab.md`'s two scenario-passage trials
(also both 2/5, prompt held constant, content varied), this is 5
independent blind-judged trials, 2 different documents, 3 different
intervention types (grammar, prompt, word choice), all landing at
2/5.** That is a strong, convergent signal, not a coincidence:
**dense, defined-term-heavy enumerative content has a real naturalness
ceiling that sentence-level interventions do not move.** The
likely actual fix is structural — either a genuine table/mapping
construction, or accepting that this content shape shouldn't be held
to a prose-naturalness bar at all (see Recommendation).

## Recommendation

Do not keep iterating on sentence-combining constructions for this
failure shape — the evidence above says that lever is close to
exhausted. Two real options remain, both bigger than a single-session
grammar tweak:

1. Design an actual associative/table block (a construction genuinely
   new to minglish, not a variant of Enumeration) for token→category,
   term→definition, and similar mapping content. This is a real
   language-design project with the same weight as the original
   Enumeration/Step-Block work — budget it as such.
2. Revisit `docs/judge-report.md`'s pass bar itself for content that
   is inherently list-shaped: judging a lookup table against a
   "reads like narrative prose" bar may be measuring the wrong thing.
   This is a methodology question, not a language gap, and should be
   decided by a human, not inferred from the data — see the
   controversial-decisions note below.

## What stays, what doesn't

- **ADR 0050 stays.** It is a real, verified, zero-regression
  capability gain (a writer can now say something that previously had
  no valid sentence form at all) even though it did not, by itself,
  close the naturalness gap it was motivated by. Keeping a good
  grammar feature and reporting that its original motivating payoff
  didn't materialize are two separate calls — see the
  controversial-decisions report for why this is flagged, not just
  quietly kept.
- `docs/adr/0014-universal-and-no.md`'s rewrite (7 sentences → 3) is
  applied and dogfoods clean. It is a strict readability improvement
  in cadence even though the blind score didn't move — worth keeping
  on its own terms, not worth over-selling as a naturalness fix.
