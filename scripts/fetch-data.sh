#!/usr/bin/env bash
# Fetch the CC BY-SA 4.0 reference-data files that are not vendored in git
# (see data/README.md), then verify every data file against the committed
# SHA-256 pins. Run once after cloning; needs network + python3.
set -euo pipefail
cd "$(dirname "$0")/.."

UD_TAG="r2.16"
WORDFREQ_VERSION="3.1.1"

mkdir -p data/ud data/freq

if [ ! -f data/ud/en_ewt-ud-test.conllu ]; then
    echo "fetching UD_English-EWT ${UD_TAG} test split (CC BY-SA 4.0)..."
    curl -sfL -o data/ud/en_ewt-ud-test.conllu \
        "https://raw.githubusercontent.com/UniversalDependencies/UD_English-EWT/${UD_TAG}/en_ewt-ud-test.conllu"
fi

if [ ! -f data/freq/en_zipf.tsv ]; then
    echo "deriving zipf table from wordfreq ${WORDFREQ_VERSION} (data CC BY-SA 4.0)..."
    venv="$(mktemp -d)/venv"
    python3 -m venv "$venv"
    "$venv/bin/pip" -q install "wordfreq==${WORDFREQ_VERSION}"
    "$venv/bin/python" - <<'EOF'
from wordfreq import iter_wordlist, zipf_frequency
import itertools
with open("data/freq/en_zipf.tsv", "w") as f:
    f.write("# word\tzipf — derived from wordfreq 3.1.1 'best' English list\n")
    for w in itertools.islice(iter_wordlist("en", "best"), 100000):
        z = zipf_frequency(w, "en")
        if z >= 1.0:
            f.write(f"{w}\t{z:.2f}\n")
EOF
fi

echo "verifying checksums..."
(cd data && sha256sum -c checksums.sha256)
echo "data ready."
