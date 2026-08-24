# Agent repair cases

One YAML per case: the harness (`crates/agenttest`) runs the repair loop
against an LLM and updates each file in place. **Re-running is a milestone
action, not routine CI** — it spends API calls and produces outputs that
need human review; this is a snapshot-boosted manual review flow, not a
cheap replicable snapshot test.

Fields: `input` (the rejected sentence) · `snapshot` (latest valid repair)
· `verdict` — your review of the snapshot: `ideal` | `needs-fix` |
`unreviewed` (auto-reset whenever a run changes the snapshot) ·
`unique_outputs` (every output ever seen, failures included — the mining
corpus) · `runs` (full per-trial logs).

Run: `OPENROUTER_API_KEY=… cargo run -p agenttest`
(env: MINGLISH_TEST_MODEL, MINGLISH_TEST_TRIALS, MINGLISH_TEST_TEMP)
