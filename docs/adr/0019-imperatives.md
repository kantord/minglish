# 0019 — Bare imperatives: verb-first is the command signature

Date: 2026-09-01
Status: proposed (tentative)

## Context

One case of the tool "agenttest" failed. The case was the sentence "Remove
the file". The verb was unknown. The error assumed a Name. The error did
not mention the command. The language had an asymmetry. The sentence "do
not delete the file" was legal. The sentence "delete the file" was not
legal. An agent reads instructions. The instructions use commands. A command is the core
Sentence Shape of the domain.

## Decision

The language enables the Imperative. The Imperative is the positive twin
of the Prohibition. The sentence "delete the file" is one example. The
sentence "check the input of the user" is one example.

An Imperative does not need a marker. The verb is the signature. A Surface Form has one Form Tag, so a verb is not a noun. If a verb opens a
sentence, then the sentence is a command. The First Token of an Imperative
is the verb. The rule matches the convention of English. The rule does not
add a token.

The reader is the addressee of an Imperative. The addressee has the status
of the Pronoun "you". The decision "0002" gives the status. A directed command names the agent. A directed command is a future design. The design is the vocative.

If a capital opens a sentence, then the error offers 2 Parses. The
error suggests a Name. The error suggests an Imperative.

## Consequences

- The language says an instruction in a natural way. A rewrite does not turn an instruction into an obligation. The phrase "you must" marks an obligation.
- The loose Grammar had the Sentence Shape. The showcase has the honest target. The case "Remove the file" has an honest target.
- A directed command waits for the vocative.
