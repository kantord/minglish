# 0008 — Redirect only when the fix is findable; otherwise ban

Date: 2026-08-31
Status: accepted (curation policy); point 3 and the relative frequency
trigger superseded by ADR 0023

## Context

A Rejected Sense has a Redirect or is a Ban. A Redirect names a replacement
for the sense. A Ban does not offer a replacement, so the writer
restructures the sentence.

The research showed the problem of a mechanical replacement. The replacement
optimizes the rarity of a word but ignores the clarity of the word. The
Redirect of the word "need" was one example. The Redirect named the word
"necessitate". One usage of the word "necessitate" matches 1349 usages of
the word "need", so a writer does not produce the rare word. A Language
Model does not produce the rare word.

A blunt message is better than a useless message. If the message is useless,
then the writer stalls. A Language Model does not stall but produces rare
words. The whole system exists for the quality of the text, so the rare
words hurt the whole system.

## Decision

The policy has 3 points:
- the Redirect
- the Ban
- the guard

The word of a Redirect must be common and must be a real synonym of the
Rejected Sense. The word must be common, because a writer chooses a common
word. The word of the Redirect has a small distance from the word of the
Rejected Sense.

If a Rejected Sense does not have a common synonym, then the sense becomes
a Ban. A Ban does not offer a replacement, so the message says "restructure
the sentence". The maintainers prefer a Ban to a rare Redirect, because a
new sentence beats a rare word.

The guard of the report measures the distance of the 2 words on the scale
of Zipf. The bound of the guard is 1.0 points. If the distance is bigger
than the bound, then the guard flags the Redirect. The maintainers review
the Redirect. Maintainers turn the Redirect into a Ban or replace the word
of the Redirect. Maintainers do not keep a rare Redirect.

The maintainers apply the rule to a whole word. If the Bans cover every
sense of a word, then the maintainers ban the whole word. The maintainers do
not offer a Redirect for the word.

## Consequences

- The number of the Redirects is small. The Redirects are good, so the
  writer can trust every message of the Linter.
- The future Linter needs a distinct message for a Ban, because the message
  of a Redirect says "use the word X". The message of a Ban says
  "restructure the sentence". The current Linter marks a Ban with a Waiver.
- An ambiguous modal is a candidate for a Ban. The modal "may" marks a
  permission or marks a possibility. The 2 senses do not have a common
  replacement, so the modal becomes a candidate for a Ban.
