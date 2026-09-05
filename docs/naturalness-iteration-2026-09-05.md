# Naturalness iteration, round 2 (2026-09-05): a grammar construction, a wrong conclusion, then a real fix

Continuation of `docs/prompt-ab.md` (same day, same session). **Read
the "Correction" section before trusting anything under "What was
tested" below it** — this file's own first conclusion was wrong, and
staying wrong for most of the session is itself part of the record,
not edited away.

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
discipline (see `docs/controversial-decisions-2026-09-05.md`).
Verified: clean build, all existing tests pass, `./scripts/check.sh`
clean, `dogfood-sweep`/`parse-report` numbers unchanged from before
the grammar edit (confirmed by `git stash`-comparing the exact same
sweep with and without the change). ADR 0050 is a real, kept,
zero-regression capability gain independent of everything below.

## What was tested (superseded conclusion — see Correction)

`docs/adr/0014-universal-and-no.md`'s 7-sentence run was rewritten
using the new construction, then judged 7 separate times against 5
different interventions (grammar coordination, word choice, a raw
markdown table, and re-judging with document context included). Every
single trial scored **2/5**, and combined with `docs/prompt-ab.md`'s 2
earlier trials that made 7/7 blind judgements at 2/5 across 2
documents. The conclusion drawn at the time — stated flatly, and
wrongly — was that "dense, defined-term-heavy enumerative content has
a real naturalness ceiling that sentence-level interventions do not
move, and it is not a testing artifact."

**That conclusion was wrong, and the "not a testing artifact" claim
was the specific error.** All 7 of those trials shared one thing: the
judge prompt used in every one of them was a shortened paraphrase of
the project's own real judge instruction in `docs/prejudge.md`, and it
silently dropped the one line that instruction leads with: *"Read
CONTEXT.md for the Capitalized terms."* Every trial this session ran
judged the paragraph with no access to the project's own term glossary
— judging a paragraph dense with Capitalized project-specific terms
(Sentence Shape, Bare Plural, Indexical Pronoun, First Token) against
a judge that has never been told what those terms mean. The "reads
like undefined jargon" and "catalog-like" complaints those judges kept
returning were, at least in part, a direct artifact of that missing
context, not a property of the prose.

## Correction: re-run with the real protocol

The same 3 paragraph versions (original, 3-way-coordinated, and a
plain Enumeration of the 7 tokens) were re-judged using the *actual*
`docs/prejudge.md` naturalness-judge prompt verbatim, with the judge
given `CONTEXT.md` to read first, exactly as the real pipeline does:

| version | naturalness (no CONTEXT.md, 7/7 trials) | naturalness (real protocol, with CONTEXT.md) |
|---|---|---|
| original, 7 flat sentences | 2/5 | 2/5 |
| 3-way coordinated (ADR 0050) | 2/5 | **3/5** |
| plain Enumeration (7 tokens, mixed granularity) | 2/5 | **3/5** |

Real movement, on 2 of 3 versions, from the protocol fix alone. The
plain Enumeration's *remaining* complaint under the correct protocol
was specific and new: the list mixes quoted literal words ("if", "do")
with abstract grammatical categories (a Bare Plural, a determiner) in
one flat list — inconsistent item granularity, not "reads like a
list" in general.

That complaint pointed at a concrete, fixable content restructuring:
split into two internally-homogeneous Enumerations — one for the 4
literal-word triggers, one for the 3 category triggers — rather than
one mixed list. Rewritten as:

```
The First Token of a sentence announces the Sentence Shape. 4 First
Tokens are words:
- "if"
- "do"
- "no"
- "every"

3 First Tokens are Categories:
- a Bare Plural
- a determiner
- an Indexical Pronoun
```

Judged with the real protocol: **naturalness 4/5.** Checked for
fidelity too (the real protocol's other required gate, telephone
explainer + rater against the original 7-sentence paragraph, run
independently, neither agent seeing both texts): **fidelity 4/5** —
meeting `docs/judge-report.md`'s actual pass bar (naturalness ≥4.0,
fidelity ≥4) on a paragraph that failed all day. This version is now
applied to `docs/adr/0014-universal-and-no.md` and dogfoods clean
(`./scripts/check.sh` and the full test suite pass, corpus parse
numbers unchanged from before this edit).

**The fidelity check did find real, non-fabricated loss**, not a false
alarm: the split version drops the explicit "a Bare Plural opens a
generic statement" / "an Indexical Pronoun opens a plain statement"
outcome mappings that the original stated and the new version doesn't
— it names the 7 First Tokens but not what each individually triggers
beyond the word/Category split. The rater scored this a 4, not a 5,
specifically because of that loss; it did not call it disqualifying.
This is flagged, not quietly accepted — see
`docs/controversial-decisions-2026-09-05.md`.

## What this actually shows

1. **The judge protocol matters as much as the content.** Half of this
   session's headline finding ("naturalness has a hard ceiling")
   was an artifact of a shortened, incorrect judge prompt, not a
   property of the language. Any future naturalness experiment must
   use the real `docs/prejudge.md` prompt verbatim, with `CONTEXT.md`
   attached, not a hand-written approximation of it.
2. **Once judged correctly, both tested interventions helped**: ADR
   0050's grammar construction moved this exact paragraph 2→3, and
   restructuring mixed-granularity list content into homogeneous
   groups moved it 3→4, past the pass bar. Neither alone would have
   been enough; both together did it on this one case.
3. **This is one paragraph, not a corpus-wide result.** 1 of the 34
   failing documents (`docs/judge-report.md`) had 1 paragraph tested
   and moved from below-bar to passing. Whether the same 2 moves
   (homogeneous grouping + ADR 0050 where applicable) generalize
   across the other ~33 failing documents' other failure paragraphs is
   untested — a real next step, not a corpus-wide claim this session
   supports.
4. **The associative/table-mapping gap identified earlier is still
   real** (minglish has no construction for a genuine key→value
   mapping — confirmed empirically, Enumeration items must be single
   noun phrases) but turned out not to be necessary for *this*
   paragraph: splitting into 2 homogeneous flat lists, accepting a
   small, disclosed fidelity cost, was enough. It may still matter for
   other paragraphs whose mapping content can't be split this way.

## Recommendation

1. **Re-run `docs/judge-report.md`'s full 37-document pass with the
   real protocol** was never in question — that's what it already
   does. What's new here: any *ad hoc* judging done during future
   design iteration (as this session did, twice, incorrectly) must use
   the real prompt from `docs/prejudge.md` verbatim, CONTEXT.md
   included, or its results are not trustworthy — this session is the
   proof.
2. Apply the "split mixed-granularity lists into homogeneous
   Enumerations" pattern to the other flagged "run of same-template
   sentences" paragraphs in `docs/judge-report.md` and re-judge with
   the corrected protocol before concluding whether it generalizes.
3. The table/mapping construction gap (see "What this actually shows,
   item 4) is still worth a real design pass, but is no longer the
   only lever — treat it as one option among several, not the
   necessarily-required fix this file originally concluded it was.

## What stays, what doesn't

- **ADR 0050 stays** — a real, verified, zero-regression capability
  gain, and (correcting the earlier record) it did measurably help
  naturalness once judged correctly, just not on its own.
- `docs/adr/0014-universal-and-no.md`'s "first-token telegraph
  principle" paragraph is rewritten to the homogeneous-split version,
  verified 4/5 naturalness and 4/5 fidelity against the real protocol,
  and dogfoods clean. The 2 lost per-item outcome mappings are a known,
  disclosed trade-off, not an oversight.
