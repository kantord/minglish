# 0023 — One meaning per word: per-sense synonyms first, absolute findability floor

Date: 2026-09-01
Status: proposed (tentative). Amends ADR 0008 (supersedes its point 3 and
the relative frequency trigger). Decides *same*.

## Context

The queue item *same* forced the question. *same* has two senses in one
syntactic slot: that very thing ("reads the same file" — resolved only by
prior discourse, the reference problem ADR 0002 bans) and two things match
("have the same format"). The second sense has a well-defined synonym,
*identical*, but ADR 0008's trigger (suggestion more than 10× rarer than
the rejected word) flags it: zipf 5.80 → 4.21, a gap of 1.59.

The gap rule conflates two quantities. Sentence cost (ADR 0006, textcost:
a word's price is its improbability) falls when padding goes and precise
words stay, so dense text is *rarer* text on average, by construction.
Findability (would the writer produce this word unprompted?) is what ADR
0008 meant to protect, and it is a property of the suggested word alone,
not of its distance from the rejected one. *identical* (4.21) is common
knowledge; *necessitate* (the case ADR 0008 was written against) is not.

## Decision

1. **The goal is one meaning per word within a syntactic category**
   (homographs across categories count as the same word for this purpose).
2. **The first line of defense is the redirect table**: every rejected
   sense that has a well-defined synonym carries it, so a writer or agent
   can pick the word with the right level of specificity. A **ban** (no
   substitute; rephrase advice) is reserved for senses with no such
   synonym.
3. **The frequency guard becomes an absolute floor on the suggested word**:
   zipf ≥ 3.5, report-only (a review trigger, never a build error — the
   general-English table underrates technical vocabulary). The relative
   1.0 gap is dropped. Against the 36 existing redirects the floor flags
   *outcast* (3.09) and *emit* (3.25); both reviewed and kept.
4. ***same* is banned** with per-sense advice: that very thing → "the
   file" (definite *the* already carries identity); two things match →
   *identical* ("the copies are identical"). ***identical*** enters as an
   ordinary ADJ.
5. Deferred: the named-standard form "identical to the report" — the
   copular complement takes no prepositional phrase yet.

## Consequences

- Redirects grow rather than shrink as polysemous words are curated; the
  lexicon report lists the sub-floor ones for review.
- The redirect table becomes the substitution table for structured repair
  (docs/ideas.md): once a failed sentence's word roles are known, a
  rejected sense maps to its synonym mechanically, with the reject row as
  the explanation. A missing synonym is a hole in that transducer, which
  is why point 2 is a rule and not a preference.
- Anaphoric *same* joins *it/that/we* as a reference-class ban; the
  repeat-the-noun fix is unchanged.
