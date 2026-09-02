# 0010 — Tense: simple past enabled; aspect and future stay out

Date: 2026-08-31
Status: proposed (tentative). Revises ADR 0003 (adds *was/were*) and
ADR 0005 (adds *did*).

## Context

The evidence of the Tense is strong. The file "corpus/untranslatable.tsv"
counts 5 sentences. The sentences need the past Tense. 4 sentences need an
aspect. 2 Translation Pairs flattened the past Tense into the present
Tense. The rewrite dropped a claim.

A past verb can cause a Reduced Relative. The sentence "the file stored in
the cache fails" is one example. The language bans the Passive. The
language bans the Reduced Relative. A past verb has one Parse, because the
Passive is a Ban. The Lexicon contains every past Surface Form.

## Decision

The language has the past Tense. A verb uses the past Surface Form. The
language enables 3 Function Words:
- "was"
- "were"
- "did"

The Form Tag of "was" is "COPULA_SG_PAST". The Form Tag of "were" is
"COPULA_PL_PAST". The Form Tag of "did" is "NEG_AUX_PAST". The word "did"
carries a past Negation. The sentence "the agent did not delete the file"
is one example.

The language does not have the future Tense. The maintainers deferred the
word "will". The language does not have the perfect aspect. The language
does not have the Progressive. The perfect aspect needs a Participle. A
Participle reopens the Reduced Relative and reopens the Passive. The
language bans every contraction.

The language does not have 3 words:
- "could"
- "might"
- "would"

The past Surface Forms have the rules of the present Surface Forms. The
word "was" is a Copula. The word "were" is a Copula. The word "did" carries
a Negation. The word "did" has one role.

## Consequences

- The language can say a report of a past action. The Translation Pairs do
  not cheat on the Tense.
- The language cannot say the phrase "has been failing". The Gap is
  acceptable. If the corpus needs the aspect, then the maintainers revisit
  the Gap.
- The argument depends on the Ban of the Passive. If a future decision
  allows the Passive, then the maintainers revisit the decision.
