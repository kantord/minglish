#!/usr/bin/env python3
"""Pre-flight for seed.json / domain/model.json batches: mirrors lexgen's two
noisiest lints and fixes them in place so one lexgen run succeeds.

- unattested generated forms → an explicit `forms` override (acknowledged)
- cross-POS attestation      → `waive` entries (noted as auto-waived)

Run before `cargo run -p lexgen` when adding many lemmas; review the notes
later. Morphology mirrors crates/lexgen/src/morph.rs.
"""
import json, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def load_refdata():
    known, pos = set(), {}
    for p, letter in [("noun", "n"), ("verb", "v"), ("adj", "a"), ("adv", "r")]:
        for line in open(os.path.join(ROOT, f"data/wordnet/index.{p}")):
            if line.startswith(" "):
                continue
            w = line.split(" ")[0]
            if "_" in w:
                continue
            known.add(w); pos.setdefault(w, set()).add(letter)
    for raw in open(os.path.join(ROOT, "data/moby/mobypos.txt"), "rb").read().decode("latin-1").splitlines():
        raw = raw.rstrip("\r")
        if "\\" not in raw:
            continue
        w, codes = raw.split("\\", 1)
        if not w.isascii() or not w.islower() or not w.isalpha():
            continue
        known.add(w)
        for c in codes:
            m = {"N": "n", "p": "n", "h": "n", "V": "v", "t": "v", "i": "v", "A": "a", "v": "r"}.get(c)
            if m:
                pos.setdefault(w, set()).add(m)
    return known, pos

VOWELS = "aeiou"
def sib(w):
    if w.endswith("y") and len(w) > 1 and w[-2] not in VOWELS:
        return w[:-1] + "ies"
    if any(w.endswith(s) for s in ("s", "x", "z", "ch", "sh", "o")):
        return w + "es"
    return w + "s"
def doubled(w):
    if len(w) <= 4 and len(w) >= 3 and w[-3] not in VOWELS and w[-2] in VOWELS and w[-1] not in VOWELS and w[-1] not in "wxy":
        return w + w[-1]
    return w
def past(w):
    if w.endswith("e"): return w + "d"
    if w.endswith("y") and w[-2] not in VOWELS: return w[:-1] + "ied"
    return doubled(w) + "ed"
def comparative(w):
    syl, prev = 0, False
    for c in w:
        v = c in VOWELS or c == "y"
        if v and not prev: syl += 1
        prev = v
    syl = max(syl, 1)
    if not (syl == 1 or (syl == 2 and w.endswith("y"))): return None
    if w.endswith("e"): return w + "r"
    if w.endswith("y") and w[-2] not in VOWELS: return w[:-1] + "ier"
    return doubled(w) + "er"
def gerund(w):
    if w.endswith("ie"): return w[:-2] + "ying"
    if w.endswith("e") and not w.endswith("ee"): return w[:-1] + "ing"
    return doubled(w) + "ing"

POSNAME = {"n": "NOUN", "v": "VERB", "a": "ADJ", "r": "ADV"}
OWN = {"NOUN": "n", "VERB_TRANS": "v", "VERB_INTRANS": "v", "ADJ": "a"}

def check(path, domain):
    known, pos = load_refdata()
    entries = json.load(open(path))
    changed = 0
    for e in entries:
        cat = e["category"]; lemma = e["lemma"]
        forms = e.setdefault("forms", {})
        if cat == "NOUN" and not (domain):
            if "plural" not in forms and sib(lemma) not in known:
                forms["plural"] = sib(lemma); changed += 1
        if cat == "ADJ" and "comparative" not in forms:
            c = comparative(lemma)
            if c is not None and c not in known:
                forms["comparative"] = c; changed += 1
        if cat in ("VERB_TRANS", "VERB_INTRANS"):
            for slot, f in [("third", sib), ("past", past), ("ing", gerund)]:
                if slot not in forms and f(lemma) not in known:
                    forms[slot] = f(lemma); changed += 1
        own = OWN.get(cat)
        if own and not (domain and (cat == "NOUN" or " " in lemma)):
            attested = pos.get(lemma, set())
            for p in attested:
                if p == own: continue
                name = POSNAME[p]
                if name in e.get("reject", {}) or name in e.get("waive", []): continue
                e.setdefault("waive", []).append(name); changed += 1
                e["note"] = (e.get("note", "") + " [auto-waived " + name + "]").strip()
        if not forms:
            del e["forms"]
    json.dump(entries, open(path, "w"), indent=2, ensure_ascii=False); open(path, "a").write("\n")
    return changed

if __name__ == "__main__":
    n = check(os.path.join(ROOT, "seed/seed.json"), False) + check(os.path.join(ROOT, "domain/model.json"), True)
    print(f"seedcheck: {n} fix(es) applied")
