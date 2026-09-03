# 0005 — Negation: not with do-support, fixed predicate scope

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0004; rewritten in minglish 2026-09-02)

## Context

Triage counts about 204 tokens of the word "not" in the corpus. Instructions
need Prohibitions and the sentence "do not delete the file" is a Prohibition.

A Negation of a verb needs the Auxiliary "do" in English but the language
avoids Auxiliaries. English allows a Negation of a Constituent, so the phrase
"not all users" has a Scope Ambiguity. The phrase "not old" has a Scope
Ambiguity.

## Decision

The language enables 3 Function Words:
- "not"
- "do"
- "does"

The word "not" has the Form Tag "NEG". The word "do" has the Form Tag
"NEG_AUX_BASE" and the word "does" has the Form Tag "NEG_AUX_3SG".

The 2 Surface Forms of the Lemma "do" carry a Negation, so the Grammar must
allow the Auxiliary in one pattern. The word "not" follows the Auxiliary and
comes before a base verb. A Negation uses the pattern and a Prohibition uses
the bare pattern. The emphatic Auxiliary is a Ban, so the sentence "the
parser does accept the file" is a Rejection. A question cannot use the
Auxiliary.

A Negation of a Copula does not need an Auxiliary, so the word "not" follows
the Copula.

The Grammar limits the scope of "not" to the main predicate of the clause,
so a Negation of a Constituent is a Ban. The Linter rejects the phrase "not
all users" and rejects the phrase "a not old file".

The decision does not enable "did" but the later decision "0010" added
"did". The orthography of the language bans every contraction. The decision
does not enable 2 contractions:
- "doesn't"
- "don't"

## Consequences

- The language can say a Negation and can say a Prohibition in natural
  sentences of English.
- The language gains one Auxiliary but the Auxiliary appears in one pattern.
- The Grammar limits the Auxiliary to one pattern and fixes the scope of
  "not". Triage does not see the 2 rules, because Triage counts the tokens
  of a sentence. Triage accepts every token of the 3 Function Words.
