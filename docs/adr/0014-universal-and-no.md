# 0014 — every / no, and the first-token telegraph principle

Date: 2026-09-01
Status: proposed (tentative)

## Context

Quantifiers block 5 of 8 dogfood sentences and top every corpus gap list.
"every X does not Y" is scope-ambiguous in English (∀¬ vs ¬∀) with split
reader priors — legislating a reading would violate least-surprise. And a
clause-level operator buried mid-sentence (e.g. a negative quantifier in
object position) forces readers to recover logical form from syntax.

## Decision

- **every** (QUANT_UNIV) + singular noun: exceptionless universal — distinct
  from the bare-plural generic (ADR 0013), which tolerates exceptions.
  Allowed in subject and object position. *all*/*each* stay out
  (one meaning, one form).
- **no** (QUANT_NEG) + singular noun: universal negative (¬∃). **Subject
  position only** — so every ∀¬ sentence begins with *no*. Object-position
  negation ("retries no request") is unwritable; its meaning lives at
  "does not retry requests".
- Quantified subjects take **positive predicates only**: no *not*, *does
  not*, *must not*, *cannot*. Each excluded combination has an unambiguous
  home (generic + negation; *no X can*; future *some*). *no + must* is
  additionally banned (deontically ambiguous in English). *no + can* stays:
  "no agent can delete the file" has one reading.
- Quantified subjects appear only in top-level statements for now — not
  inside conditionals or coordination tails (bound; revisit with evidence).
- Scope rule for the residue (e.g. *every* in object position): surface
  order = scope order.

## The first-token telegraph principle

Every sentence's first token announces its type with zero lookahead:
*if* → conditional; *do* → prohibition; *no* → universal negative;
*every* → universal; bare plural → generic; *the/a/my/i/you* → particular
statement. Every future sentence type (questions, existentials) must claim
a distinct first token. This serves readers (frame before content), LLMs
(maximal early constraint), and the LR parser (disjoint start sets) at once.

## Consequences

- Universal rules and prohibitions-of-ability become expressible in their
  most natural, densest English forms.
- The banned combinations force rephrasings that are all shorter or clearer
  than what they replace.
- More positive-predicate duplication in the grammar — the LALRPOP macro
  cleanup is now overdue.
