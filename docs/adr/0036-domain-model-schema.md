# 0036 — Domain model: unique terms, groups, examples, membership

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0027.

## Context

The repair of the decision "0006" produced 3 unnatural sentences. The
sentence "the Context Need of a sentence is the prior text of the sentence"
is one example. The definitions of the model caused the sentences. A
definition was a rule. The reader of the rule cannot see the thing.

The maintainer named the reason. The writer of a definition knows the
thing. The reader of the definition does not know the thing. An example shows
the thing. A rule does not show the thing.

## Decision

Every term of the model has a kind. The kind of a term is "unique" or is
"category". If the kind of a term is "unique", then the term names one
thing. The term
"Lexgen" is one example. If the kind of a term is "category", then the term names a group.
The term "Anaphoric Pronoun" is one example.

Every group has examples. An example is a member of the group or is a
sentence. If an example carries the character ".", then the example is a sentence. The Linter checks
the sentence. If an example has lines, then the example is a Block.
The Linter parses the Block.

A term is a member of one group. The field "member_of" names the group of
a term. The term "Ban" is a member of the group "Rejection". The tool
Lexgen checks the field. The field must name a group of the model.

The definition of a term names the group in the first sentence. The
definition names one example before a rule. The file "CONTEXT.md" shows 3
fields of a term:
- the kind
- the examples
- the group

The prompt of the repair shows the examples.

## Consequences

- A new reader can see every term. The examples show the meaning.
- Every term names the group of the term. A reader can read the group
  of a term.
- If a group does not have an example, then the tool Lexgen rejects the
  group. If the field "member_of" names an unknown group, then the tool
  Lexgen rejects the field.
- The Seed does not carry the fields. The fields belong to the model.
- The noun "Pronoun" became a term. The noun "Block" became a term.
