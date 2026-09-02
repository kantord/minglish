# Paragraph repair cases

One YAML per paragraph of a source document, written by
`just autofix-paragraphs <file>` (design: docs/ideas.md, "Paragraph
repair"). **Re-running spends API calls and the output needs ADR 0012
meaning review** — a milestone action, not CI.

Fields: `source`, `index`, `original`, `context_before/after` (the
neighbouring paragraphs the model saw) · `original_metrics` · `proposals`
(every distinct reply ever seen: text, declared `drops`, `valid`, metrics,
diagnosis when rejected, count) · `best` (highest-ranked valid proposal:
parse rate → topic continuity → cost; display order, never a gate) ·
`verdict` — your judgment of `best`: `ideal` | `needs-fix` | `unreviewed`
(auto-reset whenever `best` changes).

Review without opening the YAML: `just review-paragraphs` (one screen per
case), `just review-paragraphs N` (one case, all valid proposals), `just
verdict N ideal|needs-fix "note"`.

Accepted rewrites are applied to the source by hand. Metrics: parse =
sentences that parse; continuity = consecutive pairs whose second subject
noun appears in the first sentence; cost = Σ (9 − zipf) per word, as
textcost. `--dry-run` measures originals without API calls.
