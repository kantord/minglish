# 0010 — Tense: simple past enabled; aspect and future stay out

Date: 2026-08-31
Status: proposed (tentative). Revises ADR 0003 (adds *was/were*) and
ADR 0005 (adds *did*).

## Context

The evidence of the past Tense is strong. The file
"corpus/untranslatable.tsv" marks 5 sentences for the Tense and marks 4
sentences for the aspect. 2 Translation Pairs flattened the past Tense into
the present Tense, so the loss of the meaning was silent.

The classic danger of a past Surface Form is the Reduced Relative. The
phrase "the file stored in the cache" shows the danger. The decision "0003"
bans the Passive and bans every Participle. The 2 Bans remove the danger,
so a past Surface Form has one Parse. Lexgen builds a full Paradigm for
every Lemma, so the Lexicon contains every past Surface Form.

## Decision

The language has the past Tense. A writer marks the past Tense with the
past Surface Form of the verb. The maintainers add 3 Function Words:
- "was"
- "were"
- "did"

The word "was" has the Form Tag "COPULA_SG_PAST", and the word "were" has
the Form Tag "COPULA_PL_PAST". The Auxiliary "did" carries the past
Negation, so the Form Tag of "did" is "NEG_AUX_PAST". The sentence "the
agent did not delete the file" shows the past Negation.

The maintainers deferred the word "will", so the language does not have
the future Tense. The perfect aspect needs a Participle, and the
Progressive needs a Participle. A Participle reopens the Reduced Relative
and reopens the Passive. The language excludes the 2 aspects, because the
2 aspects need a Participle. The language excludes 3 modals:
- "could"
- "might"
- "would"

The 3 modals mark a false condition. The language bans every contraction.

The past Function Words follow the rules of the present Function Words.
The word "was" is a Copula, and the word "were" is a Copula. The word
"did" carries a Negation. The language bans the emphatic Auxiliary and
bans the Auxiliary in a question.

## Consequences

- A writer can describe a past event with the past Tense, so the
  Translation Pairs do not cheat on the Tense.
- The language cannot say the phrase "has been failing", because the
  phrase needs an aspect. The Gap is acceptable. If the corpus adds an
  example of the Gap, then the maintainers revisit the Gap.
- The argument of the decision depends on the Ban of the Passive and
  depends on the Ban of the Participle. If a future decision admits a
  Participle, then the maintainers must revisit the decision "0010" before
  the future decision.
