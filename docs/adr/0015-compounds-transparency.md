# 0015 — Compounds: transparency-first resolution, hyphens as last resort

Date: 2026-09-01
Status: accepted (curation policy)

## Context

Compounds broke 4 sentences in the Dogfood of 8 sentences. Every technical
text carries Compounds. A Compound carries 2 ambiguities:
- the bracketing
- the relation

A chain of 3 nouns has an ambiguous bracketing. A Compound does not name
the relation of the 2 nouns. The maintainers considered 3 candidates:
- a single word
- a hyphenated word
- a paraphrase

The third candidate uses the word "of".

A writer can replace a Compound with the Head Noun. The phrase "the seed"
replaces the Compound "seed list". The Head Noun is dense but is opaque. A
reader needs the context of the project, so the Head Noun has a big
Context Need. The decision "0006" requires a small Context Need. The rule
covers a word, so the Head Noun violates the rule.

## Decision

The transparency of a term is the criterion of the remedy. If a new
reader rebuilds
the meaning of a term, then the term is transparent. A transparent term
does not need a definition. The maintainers rank 4 remedies:
- an ordinary word
- a transparent paraphrase
- a hyphenated word
- a Rejection of the term

The maintainers choose the first transparent remedy of the list.

If the ordinary meaning of a word is the concept, then the word is the
first remedy. The word "binary" replaces the Compound "build output". The
transparency of a term depends on the readers. The maintainers judge the
transparency against the readers of the text. A pack holds the terms of
one domain, so the pack judges the transparency against the readers of
the domain.

If the Lexicon does not have a transparent word, then the maintainers
write a transparent paraphrase. The paraphrase uses the words of the
Lexicon. The phrase "the initial words" replaces the Compound "seed
list". A paraphrase can be longer than the
Compound. If the padding reduces the work of the reader, then the padding
earns the length. The Context Need outranks the density in the decision
"0006".

If a transparent paraphrase does not exist, then the maintainers add a
hyphenated word. A hyphenated word must be transparent. A hyphenated word
is one token of the Lexicon. The word "temperature-sensor-cable" is one
example. A hyphenated word is a noun of the Lexicon, so the Linter checks
the word. The word gets the Redirects and gets a note.

If the hyphenated word is opaque, then the maintainers reject the term.
The Compound "form-tag" is one example. An opaque Name costs the reader.
The maintainers do not rename the Name but unfold the Name into a
sentence. The sentence "every word has one tag" unfolds the Compound
"form-tag". No opaque coinage enters the Lexicon.

The Head Noun is not a remedy. The Head Noun is short but is opaque. The
criterion rejects an opaque word.

## Consequences

- The remedy of a Compound is the curation of one word. The curation uses
  the current tools. The guard of the frequency measures the findability
  of the word. The field "note" records the choice. The remedy does not
  need a new subsystem.
- A paraphrase adds words to the document but removes a definition. The
  choice is deliberate. The decision depends on one claim. An opaque
  word costs the reader. The research did not verify the claim. The decision
  adopts the claim, so the claim is a value of the design. The file
  "cnl-design-findings.md" records the promotion of the claim.
- Some technical terms do not get a Name. The maintainers unfold the terms
  into sentences. If the sentences of a text need one Name, then the
  pressure is a signal for a remedy. The maintainers find an ordinary
  word or find a paraphrase. The maintainers do not coin a Name.
