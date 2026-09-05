# 0051 — A table for one fact of every word (Mapping)

Date: 2026-09-05
Status: proposed (tentative). Extends ADR 0028.

## Context

The report "docs/judge-report.md" names the problem. the language did
not have a shape for the fact. a writer names one fact for one word,
and a writer repeats the pattern for every word. short sentences are
mechanical.

## Decision

A table holds the fact. the writer opens the table with a statement.
the statement ends with a colon. the table holds one row for every
word.

a row of the table has 2 columns. the first column names the word,
and the second column names the fact. every column is one Noun
Phrase.

the Grammar checks every column. the Grammar does not check the
shape of the table.

## Consequences

- the table names the fact for every word.
- the table needs the intro, and the intro needs the colon.
- the Grammar does not check every table. the writer marks the table
  with the intro. the Grammar checks the table.
