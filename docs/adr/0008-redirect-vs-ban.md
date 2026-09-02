# 0008 — Redirect only when the fix is findable; otherwise ban

Date: 2026-08-31
Status: accepted (curation policy); point 3 and the relative frequency
trigger superseded by ADR 0023

## Context

A Rejected Sense carries a Redirect or is a Ban. A Redirect names a
replacement. If a sense is a Ban, then the writer restructures the
sentence.

The research showed one result. A mechanical replacement optimizes the
rarity of the words. A mechanical replacement does not optimize the clarity
of the words. One example is the Redirect of the word "need". The
Redirect names the word "necessitate". The word "necessitate" is rare. A writer does not produce the word
"necessitate". A Language Model does not produce the word "necessitate".

A useless message is bad. A blunt message is acceptable. If the message is
useless, then the writer stalls. If the message is useless, then the
Language Model produces rare words. The rare words hurt the quality of the
text.

## Decision

The maintainers use 2 tools:
- a Redirect
- a Ban

A Redirect names a common word. A Redirect names a synonym of the Rejected
Sense. A writer chooses the word. If a word is rare, then the word is not a
Redirect.

If a Rejected Sense does not have a synonym, then the sense is a Ban. A Ban
gives one instruction to the writer. The writer restructures the sentence. A new
sentence beats a rare word.

The report of the Lexicon shows a rare Redirect. The maintainers review a
rare Redirect. The decision "0023" changed the rule of the guard.

The rule covers a whole word. If a sense does not have a common
replacement, then the sense is a Ban. If the Bans cover every sense of the
word, then the word is a Ban.

## Consequences

- The Redirects are rare. The Redirects are good.
  The writer trusts the messages of the Linter.
- A Ban has a message. A Redirect has a message. The 2 messages differ.
- An ambiguous modal is a Ban. The word "may" is one example. The word
  "may" has 2 senses. The senses do not have a common replacement.
