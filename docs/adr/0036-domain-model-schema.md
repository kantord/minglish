# 0036 — Domain model: unique terms, groups, examples, membership

Date: 2026-09-03
Status: proposed (tentative). Extends ADR 0027.

## Context

The rewrite of the decision "0006" produced 3 unnatural sentences. The
sentence "the Context Need of a sentence is the prior text of the sentence"
is one example. The unnatural sentences came from the definitions of the
model. A definition was a rule, so the reader of the definition cannot see
the meaning.

The maintainer gave the reason. The writer of a definition knows the
meaning but the reader of the definition does not know the meaning. An
example shows the meaning but a rule does not show the meaning.

## Decision

Every term of the model has one kind. The kind of a term has 2 values:
- "unique"
- "category"

A term of the kind "unique" names one thing. The term "Lexgen" names one
tool, so the kind of the term "Lexgen" is "unique". A term of the kind
"category" names a group. The term "Anaphoric Pronoun" names a group, so
the kind of the term is "category".

Every group has examples. An example is a member of the group or is a
sentence. If an example ends with the character ".", then the example is a
sentence. If an example has lines, then the example is a Block. The Linter
checks the sentence and parses the Block.

A term can be a member of one group. The field "member_of" names the group.
The term "Ban" is a member of the group "Rejection", so the field
"member_of" of the term names the group "Rejection". The tool Lexgen checks
the field, because the field must name a group of the model.

The first sentence of a definition names the group. The definition gives an
example before the first rule. The prompt of a repair shows the examples to
the writer. The file "CONTEXT.md" shows 3 fields of a term:
- the kind
- the examples
- the group

## Consequences

- The examples show the meaning, so a new reader can see every term.
- Every term shows the group of the term to a reader.
- If a group does not have an example, then the tool Lexgen rejects the
  group. If the field "member_of" names an unknown group, then the tool
  Lexgen rejects the field.
- The Seed does not carry the fields, because the fields belong to the model.
- 2 nouns became terms:
  - "Pronoun"
  - "Block"
