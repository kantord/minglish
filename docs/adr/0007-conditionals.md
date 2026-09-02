# 0007 — Conditionals: if … , then … (one template, condition first)

Date: 2026-08-31
Status: proposed (tentative — same review point as ADR 0002–0005; rewritten in minglish 2026-09-02)

## Context

Triage counts about 87 tokens of the word "if" in the corpus. Instructions
need Conditionals. The sentence "if the test fails, then the agent retries
the request" is a Conditional. The initial design had the Conditional.

If a Conditional does not have a marker, then the boundary of the 2 clauses
is ambiguous. English allows the consequent at the front of the sentence.
The sentence "the agent retries the request if the test fails" is one
example. The reader holds the consequent in the memory. The condition is
the last clause of the sentence.

## Decision

The language enables 2 Function Words:
- "if"
- "then"

The Form Tag of "if" is "SCONJ_COND". The Form Tag of "then" is "THEN".

The Conditional has one Sentence Shape. The Sentence Shape is the phrase
"if <clause>, then <clause>". The Sentence Shape puts the condition at the
front of the sentence. The reader reads the condition, so the reader has
the context for the consequent. The condition is one clause, so the front of
the sentence has a bound.

The comma is mandatory. The word "then" is mandatory. The 2 markers show
the boundary of the consequent. The markers remove every wrong Parse at the
seam of the clauses.

The language does not enable the consequent at the front of the sentence.
The language does not enable a nested Conditional inside the condition.

The language does not enable 4 words:
- "unless"
- "when"
- "whenever"
- "else"

## Consequences

- The language can say a rule. The language can say a reactive instruction.
  The Conditional has one Sentence Shape.
- A writer restructures the sentence "X if Y" into the sentence "if Y, then
  X". The rewrite is mechanical. The Linter suggests the rewrite.
- The Grammar enforces the Sentence Shape. Triage does not see the Sentence
  Shape, because Triage checks the tokens.
