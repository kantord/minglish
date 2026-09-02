#!/usr/bin/env python3
"""Print a domain term's definition (ADR 0027): define.py "Anaphoric Pronoun" """
import json, sys, os
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
model = json.load(open(os.path.join(ROOT, "domain/model.json")))
q = " ".join(sys.argv[1:]).strip().lower()
hits = [e for e in model if e["lemma"] == q or e["lemma"] + "s" == q or q.startswith(e["lemma"])]
if not hits:
    sys.exit(f"no term {q!r} — see CONTEXT.md")
for e in hits:
    shown = " ".join(w.capitalize() for w in e["lemma"].split()) if e["category"] in ("NOUN", "NAME") else e["lemma"]
    print(f"{shown} ({e['category'].lower()}) — {e['definition']}")
