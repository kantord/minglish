# Ideas (deferred, not designed)

Parking lot for directions we intend to explore but have deliberately not
committed to. Move an item into an ADR when it becomes a real decision.

## Vector embeddings as measurement tooling

Constraint: embeddings may serve *measurement and suggestion*, never the
language definition (ADR 0001's check-don't-choose discipline — model weights
are not auditable data). Candidate uses, ranked:

1. **Meaning-preservation guard for corpus/pairs.tsv** — sentence-embedding
   cosine between the English and minglish sides; catches silent meaning
   loss (e.g. dropped quantifiers) that lexicon-validity checks cannot see.
   Caveat (2026-09-01): pooled embeddings are close to order-blind — a
   subject/object swap lands almost on top of the original — so cosine
   cannot judge role changes. For the faithfulness gate proper, see
   "Structured repair", NLI.
2. **Redirect findability scoring** — rank substitute candidates by
   similarity-to-rejected-sense × frequency, operationalizing ADR 0008.
3. **Homonymy vs polysemy detection** — cluster a word's contextual
   embeddings across corpus uses; distant clusters = dangerous homonymy.
   Better instrument than WordNet sense counts, which overcount.
4. **Pair mining** — retrieve sentences semantically near already-translatable
   ones to raise sampling yield.

Licensing: GloVe vectors (PDDL) and sentence-transformers models (Apache)
are clean; CC BY-SA vector sets fit the existing fetch-not-vendor pattern.

## Core lexicon + jargon addon packs

Split the seed into a domain-neutral core (function words + general
vocabulary) and optional domain packs (tech, cooking, …). A minglish text
declares core + packs; lexgen lints the combined set.

Key principle if/when built: **packs add, never override** — a pack may not
shadow a core surface form or re-enable a sense the core rejects, so any
core+packs combination remains one unambiguous language. Core redirects may
only point at core words.

Deferred (2026-08-31) because the lexicon is ~50 lemmas and one layer; the
split earns its keep only once a second genuine domain exists. Justification
when built should cite our own triage data (function words dominate OOV;
current vocabulary is effectively a tech pack), not external CNL specs.

## Curation phasing (active decision, 2026-09-01)

Initial dogfooding period: curate vocabulary for **our own jargon** (the
minglish project's ADRs, docs, software/linguistics register). Dogfood
coverage is the primary curation metric; EWT/general-register triage stays
reported but is telemetry, not a target. Later, **re-center** by testing on
different text types (Simple English Wikipedia, instructions, other
domains) and rebalance the lexicon — the core/packs split (above) is the
likely mechanism when that day comes.

## Compile parsed sentences to pseudocode (2026-09-01)

Mechanical reserialization of the tier-1 parse tree into pseudocode: the
head-annotated tree already carries subject/verb/object/PP roles, so
"if the test fails, then the agent retries the request" folds mechanically
into `if fails(the test): retries(the agent, the request)`. Precedent: the
ACE→first-order-logic tradition — minglish's version targets code-shaped
output instead of DRS/FOL. Motivating use: a self-healing feedback loop for
coding agents — instructions written in minglish are certified unambiguous,
then compiled to pseudocode the agent (or a checker) can execute or diff
against its actual behavior, closing the loop. Cheap to prototype: one
recursive function over `Tree` in the grammar crate.

Extension (2026-09-01): compile to *runnable* analysis code, not just
pseudocode display — each sentence becomes a small executable
check/assertion. Then multi-sentence texts get machine analysis for free:
consistency checking across sentences (two rules that contradict), coverage
("which corpus cases does this policy text actually decide?"), and the
self-healing agent loop (diff stated rules against observed behavior).

## Quotation as a construction (2026-09-01)

ADR 0018 quotes cover verbatim identifiers only (one opaque thing). True
quotation — mentioning sentences/phrases as language — is a separate future
construction: the quoted span is *mentioned*, not used, and could be parsed
recursively (a quoted minglish sentence validated as minglish; a quoted
foreign string held opaque). Needed for meta-prose ("the writer types
\"…\""), error-message docs, and eventually minglish talking about its own
rules. Distinguish delimiters or introduce a marker when designed.

## Linter advice gaps (found building the showcase, 2026-09-01)

1. Banned words with messages: "it fails" → generic WORD flag; ADR 0002
   promised "repeat the noun you mean". Needs the banned-word-with-message
   mechanism (ADR 0008 consequence) — a ban table beside the reject table.
2. Dormant redirects on inflected forms: "the agent files the report" —
   "files" lexes as the enabled NOUN_PL, so the file→submit VERB redirect
   never surfaces. The diagnose layer should notice a noun in verb position
   whose lemma has a rejected-VERB entry and surface the suggestion.

## Imperative-input advice gap (agenttest run 4, 2026-09-01)

"Remove the file" went 0/3 first-try: the sentence-initial-capital error
says "a name cannot start a sentence", but the writer meant an imperative
with an OOV verb. The message should offer both readings: "…or if this is a
command, minglish has no bare imperatives — write 'you must <verb> …' or
'do not <verb> …'". Also: whether bare positive imperatives deserve a
sanctioned form is now a data-backed design question.

## Structured repair: structure enumeration + table-driven rewrite (2026-09-01)

Design sketch for turning the linter from a rejecter into a corrector,
deterministic first, model-assisted last. Pruned to the parts that fit this
project; everything else was considered and dropped.

1. **Structure enumerator.** The tier-1 grammar is a cycle-free CFG and
   LR(1)-unique, so parse trees with N leaves ↔ accepted sequences of N
   form-tags, a finite set. Bottom-up DP over the expanded grammar (the
   same shape as diagnose's Tier2 counter) lists them all; brute force over
   38^N is not needed. ~2 h. Yields the complete inventory of sentence
   shapes per length — a testable artifact on its own.
2. **Role assignment on failure.** Each word has a small candidate tag set:
   an enabled word gets its form-tag plus every rejected sense in its
   redirect entry (*files* = NOUN_PL, or VERB via the reject row); an
   unknown word gets any open-class slot. Match the words' tag sets against
   the enumerated structures (permutations of the multiset). Every match is
   parse-valid by construction and names the role each word was playing.
   Gap messages become "needs a verb here", not "not a minglish word".
3. **Table-driven rewrite with explanation.** With roles known, a word in a
   rejected sense takes the redirect synonym inflected to the slot's form
   from its generated paradigm (*files* in a verb slot → *submits*); a
   banned sense leaves a hole with the rephrase advice; an unknown word
   keeps its assigned role. Output: a valid sentence plus one line per
   changed word — role chosen, reject row that fired, curation note. All
   data already lives in the seed. This retires advice gap #2 above rather
   than patching it, and it makes the per-sense-synonym policy (ADR 0023)
   structurally necessary: every rejected sense with a findable synonym must
   carry it, or the transducer has a hole where a suggestion belongs.
4. **Edit-distance repair** (later): add paradigm swaps and closed-class
   insert/delete (*the*, *then*, *does not*, comma), bounded at two edits.
   Covers the failure classes the agent cases actually show (dropped
   *then*, agreement, passive→active shape). Only here does the candidate
   set grow enough to need a ranker.
5. **Faithfulness gate: bidirectional NLI** (later, and its real value is
   not ranking). A candidate is a faithful rewrite only if input ⊨ candidate
   and candidate ⊨ input; a role swap fails one direction. This mechanizes
   the ADR 0012 loss review that is a human verdict on every dogfood pair
   and agent snapshot today. Local, deterministic given the model, explains
   itself as "does not entail". Dependency-parse role comparison (UD-scheme
   parser over the fetched EWT) is the fallback for unknown-word roles;
   marginal while gaps are being closed by curation.

Dropped: embeddings as a ranker (order-blind, see above; keep only as a
coarse gap detector), cross-encoders (duplicate NLI without the
explanation), LM log-probability ranking (spends API budget on a question
the redirect table already answers), late-interaction retrieval (a search
problem this project does not have).

## Paragraph repair: fix-and-compare in context (agreed design, 2026-09-01)

Sentence-level fixes can be right in isolation and wrong for the paragraph
(given-before-new, topic continuity). Agreed flow, to build after the
coherence measurement has run:

1. `just autofix` sends each **paragraph** to the repair loop with the
   per-sentence linter verdicts, the skill, and the neighbouring sentences.
2. The model returns a rewritten paragraph plus a declared-drops line
   (ADR 0012 contract, as in corpus/pairs.tsv).
3. The tool lints every sentence of the proposal, computes the document
   metrics (parse rate, topic continuity, relation inventory) for original
   and proposal, and writes one YAML per paragraph in `tests/paragraph-cases/`
   (original, every trial's proposal, metrics, `verdict`) — the agent-cases
   pattern at paragraph scale.
4. The report renders original and proposals side by side with metric
   deltas, ranked parse-rate → continuity → cost. Display order only, never
   a gate. Accepted rewrites are applied by hand until verdicts show the
   proposals are trustworthy.

