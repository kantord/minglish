---
name: minglish
description: Write minglish — a restricted, unambiguous subset of English. Every sentence you produce must parse in the minglish grammar. Use when asked to write, correct, or translate minglish sentences.
---

# Writing minglish

minglish is a small subset of English in which every sentence has exactly
one parse. Only the words and sentence shapes below exist. When repairing a
rejected sentence, keep its meaning; change only what the error requires.

## Orthography

- Everything is lowercase, except: sentence-start capitals are allowed;
  `I` is always the pronoun; a capitalized word mid-sentence is a proper
  name (e.g. `the tool Lexgen builds the lexicon`).
- A name may not start a sentence unquoted — introduce it (`the tool
  Lexgen …`) or quote it (`"lexgen" …`).
- Double quotes hold verbatim identifiers, treated as one thing:
  `the file "seed.json" is old`.
- No contractions (`don't` → `do not`). Sentences may end with a period.

## Sentence shapes (the only ones)

1. Statement: `the agent reads the file` / past: `the agent deleted the file`
2. Copular: `the queue is empty` · `the queue is not empty` ·
   `the tests were old` · `the parser is a program`
3. Negation: `the agent does not store the file` ·
   past `the user did not open a session`
4. Imperative: `delete the file` · `check the input of the user` — a
   sentence may start with a bare verb; that IS the command form. Prefer it
   over "you must …" when the source is an instruction, not an obligation.
   Prohibition: `do not delete my report`
5. Modal: `the agent must check the input` · `the agent must not delete
   your report` · `the user can open a session` · `you cannot delete the
   lexicon`
6. Conditional — comma and then are mandatory, condition first:
   `if the test fails, then the agent retries the request`
7. Coordination (binary only): `the server stores the message and returns
   the result` · `the test fails or the agent retries the request`
8. Quantified (first word signals the type):
   - generic (kind-level, tolerates exceptions): `machines find collisions`
   - universal: `every agent retries the request`
   - none: `no agent retries the request` (subject only; no second negation;
     never with must)
   - some / not-all: `some agents retry the request` ·
     `some agents do not retry the request` (some + plural, subject only)
9. Possession: `every word has one tag` (`one` means exactly one)
10. Counts are digits with a plural noun: `the agent deleted 3 files` ·
    `3 agents retry the request`. `one` stays a word (`one file`, never
    `1`). Never `0` or `zero`: write `no agent retries the request` or
    `the agent does not delete files`. Never number words (`three`).
    Shares: `43 percent of the swaps did not reduce the ambiguity` (digits +
    `percent` + `of` + the named set; plural agreement). Approximate counts:
    `about 10 rules` or `~10 rules`; `about` exists only before digits.

## Hard bans (rewrite instead)

- Third-person pronouns (`it, they, he, she, this, that`): repeat the noun.
- Passive (`is stored`): name the doer, use active voice.
- Progressive (`is running`) and perfect (`has deleted`): use simple
  present or simple past.
- Relative clauses (`the file that failed`, `the file stored in X`): split
  into two sentences.
- `every … not` and object-position `no`: use the quantified forms above.
- At most ONE verb-attaching prepositional phrase per clause; `of` attaches
  to the noun before it (`a copy of the report`), all other prepositions
  attach to the verb.
- Subjects and verbs agree; `a/an` take singular nouns.
- Only the words in the word list below exist (plus proper names and quoted
  identifiers). Do not invent or inflect new words.

## Word list

(appended automatically from the current lexicon)
