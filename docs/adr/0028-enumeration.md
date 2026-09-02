# 0028 — Enumeration: a block that lists things

Date: 2026-09-02
Status: proposed (tentative). First block-level structure.

## Context

Paragraph repair on ADR 0002 turned "Banned: … (it, they, he, she, this,
that, these, those)" into nine sentences, one per pronoun. The proposal was
valid and unreadable. Lists are how technical prose enumerates members of
a set; the language had only sentences, so a list became repetition.

## Decision

- An **Enumeration** is a block, not a sentence: one plain statement
  ending in a colon, then one line per item starting with `- `.
- By fiat the items enumerate the statement's **last noun phrase**. That
  phrase must be plural, counted with digits, or `every <noun>`; it must
  end the statement (no trailing prepositional phrase or adjective, no
  coordination tail). A digit count must equal the item count.
- An **item is one noun phrase**: a quoted word, a Capitalized term, a
  name, or `the <noun>`. Clauses are not items; nested lists do not exist.
- The tools treat a block as one unit: `just lint` takes it as one argument
  with newlines; the corpus, the document lint, the ADR extractor and the
  paragraph repair flow group an intro line and its items. In markdown the
  dashes render as bullets.
- Deferred: ordered (numbered) lists whose items are clauses (steps,
  rules); a colon inside a sentence; an intro that is a question.

## Consequences

- "The language allows 4 pronouns: - "I" - "you" - "my" - "your"" replaces
  four sentences, and the count is checked.
- The intro's syntactic role is fixed narrowly on purpose: only the object
  (or copular complement) can be enumerated, so a reader never has to
  guess which phrase the list expands.
- The first construction that spans lines; the extractor's line-based
  contract gains one exception, marked with ⏎ in its output.
