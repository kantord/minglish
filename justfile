# minglish task runner — `just <target>`

# run every repo invariant (tests, regeneration, drift check)
check:
    ./scripts/check.sh

# fetch/derive the non-vendored reference data
fetch-data:
    ./scripts/fetch-data.sh

# regenerate the linter showcase
showcase:
    ./scripts/showcase.sh

# lint one or more sentences (quote each one: just lint "the agent reads the file")
[positional-arguments]
lint +SENTENCES:
    -cargo run -q -p diagnose -- "$@"

# document-level lint of one markdown file (parse rate, topic continuity, relation inventory)
lint-file FILE:
    python3 scripts/lint-file.py {{FILE}}

# regenerate docs/coherence-report.md over every ADR
coherence:
    ./scripts/coherence.sh

# MILESTONE: run the LLM repair-loop harness (needs OPENROUTER_API_KEY;
# see tests/agent-cases/README.md — results need human review)
agenttest concurrency="64":
    MINGLISH_TEST_CONCURRENCY={{concurrency}} cargo run -p agenttest

# MILESTONE: propose minglish fixes for a markdown file's prose (never edits
# the source; output needs ADR 0012 meaning review)
autofix file out="docs/autofix-report.md":
    python3 scripts/extract-sentences.py {{file}} > /tmp/autofix-sentences.txt
    cargo run -p agenttest -- fix /tmp/autofix-sentences.txt {{out}}
