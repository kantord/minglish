# 0019 — Bare imperatives: verb-first is the command signature

Date: 2026-09-01
Status: proposed (tentative)

## Context

One case failed in the 4th run of the tool "agenttest". The case was the
Imperative "Remove the file" with an unknown verb. The first word of the
case was capitalized, so the error suggested a Name. The error did not
mention the command. The Prohibition "do not delete the file" was legal, but
the Imperative "delete the file" was not legal. The language cannot justify
the asymmetry. Agents read instructions, so a command is the core register
of the domain.

## Decision

The language allows the Imperative. A verb opens the Imperative, so the
Imperative is the positive twin of the Prohibition. The Imperative has 2
examples:
- "delete the file"
- "check the input of the user"

An Imperative does not need a marker, because the verb is the signature of
the command. The Lexicon gives one Form Tag to every Surface Form, so a verb
does not have a second Form Tag. If a verb opens a sentence, then the
sentence is a command. The verb is the First Token of the Imperative, so the
Imperative does not collide with a different Sentence Shape. The rule
matches the convention of English and does not cost a token.

The reader is the addressee of an Imperative. The decision "0002" gives the
indexical status to the Pronoun "you". The addressee has the indexical
status of the Pronoun "you", so an Imperative does not have a Reference
Ambiguity in the common case. A directed command names the doer and needs a
vocative. The vocative is a later design, so the decision does not cover
directed commands.

If the first word of a sentence is capitalized, then the error offers 2
Parses. The error suggests a Name or suggests an Imperative. The writer
introduces the Name or writes the quoted Name.

## Consequences

- The Imperative is the natural shape of an instruction, so a rewrite does
  not turn the instruction into an obligation. The phrase "you must" marks
  an obligation. The repairs of the showcase turned instructions into
  obligations. An obligation is stronger than an instruction, so the repairs
  changed the claim.
- The version "Tier-2" of the Grammar expected the Sentence Shape before the
  decision. The repairs of the showcase get honest targets. The case "Remove
  the file" gets an honest target.
- The maintainers defer the directed commands of a team until the design of
  the vocative.
