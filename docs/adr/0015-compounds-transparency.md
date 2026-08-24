# 0015 — Compounds: transparency-first resolution, hyphens as last resort

Date: 2026-09-01
Status: accepted (curation policy)

## Context

Noun-noun compounds block 4 of 8 dogfood sentences and pervade technical
prose. They carry two ambiguities: bracketing (three-noun chains) and the
unspecified noun-noun relation. Candidate fixes: single-word replacement,
hyphenated lexicon entries, of-genitive rephrasing. Head-noun shortening
("the seed" for *seed list*) is dense but opaque — it demands insider
context, violating minimal context need (ADR 0006 §3) at the vocabulary
level.

## Decision

The deciding criterion is **transparency**: can a first-time reader
reconstruct the meaning unaided? Preference order per compound:

1. **An existing word whose everyday meaning already is the concept**
   (*build output* → *binary*). Transparency is audience-relative — judged
   against the target audience; domain packs will each judge against their
   own readership.
2. **A transparent paraphrase from enabled words** (*seed list* → "the
   initial words"). Longer is fine: padding that removes comprehension cost
   is earned (ADR 0006 §4 ↔ §3 trade).
3. **A hyphenated single-token lexicon entry**, only when itself transparent
   (*temperature-sensor-cable*) and no option 1–2 exists. Curated like any
   noun: linter, redirects, note.
4. **Reject the term entirely** when no transparent form exists (*form-tag*):
   a name that requires hard thought is not renamed, it is unfolded into a
   stated sentence ("every word has one tag"). Opaque coinages never enter
   the lexicon.

Head-noun shorthand ("the seed") is rejected as a mechanism: short but
opaque, the wrong side of the transparency criterion.

## Consequences

- Compound resolution is per-word curation under existing machinery (the
  frequency guard measures findability; the `note` field records the
  choice); no new subsystem.
- Documents become slightly longer where paraphrases replace compounds, and
  need fewer term definitions — the trade is deliberate. This adopts as a
  design *value* the reader-cost claim our findings doc records as
  empirically unverified; the findings doc notes the promotion.
- Some technical terms will be stated rather than named; where a text needs
  a name repeatedly, that pressure is the signal to find an option-1/2 form,
  not to coin jargon.
