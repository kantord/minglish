# 0034 — Step Block: ordered steps in the shape of Gherkin

Date: 2026-09-03
Status: proposed (tentative). Second Block-level structure after ADR 0028.

## Context

The rewrite of the decisions turned every list of the steps into the prose. The decision "0028" allows an item. The item is a Noun Phrase. A step is a clause. The
word "then" opens a step in English. The decision "0014" reserves the
First Token. The maintainer said one thing. A step needs an unambiguous structure. The structure has lines. The Enumeration is one example. Gherkin has
the structure. A file of Gherkin must be a document of Minglish.

## Decision

A Step Block is a Block. A keyword opens every line of a Step Block.
The language has 4 keywords:
- "Given"
- "When"
- "Then"
- "And"

The keyword "Given" marks a precondition. The keyword "When" marks an
event. The keyword "Then" marks a result. The keyword "And" marks a
continuation of the prior keyword. Every line holds one clause. The clause
does not carry a Coordination. The rule of the Conditional is identical.

A header opens a Step Block. The line "Feature:" is a header. The line
"Scenario:" is a header. A header carries a title. The title is a
sentence of Minglish or is a quoted Name. The Linter lints the title.

The word "Then" opens a line inside a Step Block. The word "then" does not open a sentence of the prose. The rule of the decision "0014" stays. The
keyword "But" of Gherkin is a Ban, because the word "but" joins 2 clauses
inside one sentence.

Every tool reads a Step Block. The Block is one unit. The corpus keeps the
lines of a Block. The tool "lint-file" keeps the lines of a Block. The
extractor keeps the lines of a Block. The repair of the paragraphs keeps
the lines of a Block. A file of Gherkin is a document. The
tool "lint-file" lints the file.

## Consequences

- The language can say a procedure. The steps have a shape.
- The Linter parses every file of Gherkin in the folder "features". The test
  "feature_files_parse" checks the folder.
- The Linter names the position of the word "then".
- A Block has a shape for the text. A spoken rendering needs a different shape. The file "docs/ideas.md" records the idea.
