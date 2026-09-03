#!/usr/bin/env python3
"""Review paragraph-repair cases without opening the YAML.

  paragraph-review.py                 list every case: original, best, drops, verdict
  paragraph-review.py N               one case in full detail (all valid proposals)
  paragraph-review.py N ideal         set the verdict (ideal | needs-fix | unreviewed)
  paragraph-review.py N needs-fix "note"   verdict plus a free-text note
Cases are matched by their index across tests/paragraph-cases/*.yaml.
"""
import glob, sys, os, textwrap, yaml

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CASES = os.path.join(ROOT, "tests/paragraph-cases")

def load():
    out = []
    for f in sorted(glob.glob(os.path.join(CASES, "*.yaml"))):
        d = yaml.safe_load(open(f))
        d["_path"] = f
        out.append(d)
    return sorted(out, key=lambda d: (d["source"], d["index"]))

def wrap(s, indent="    "):
    return textwrap.fill(s, width=88, initial_indent=indent, subsequent_indent=indent)

def best_proposal(d):
    return next((p for p in d.get("proposals", []) if p["text"] == d.get("best")), None)

def show(d, full=False):
    m = d["original_metrics"]
    print(f"\n[{d['index']}] {os.path.basename(d['source'])} — original parse {m['parsed']}/{m['sentences']} — verdict: {d.get('verdict','unreviewed')}")
    print(wrap(d["original"]))
    b = best_proposal(d)
    if not b:
        print("    (no valid proposal)")
        return
    print("  → best:")
    print(wrap(b["text"]))
    if b.get("drops"):
        print(f"    drops: {b['drops']}")
    if d.get("note"):
        print(f"    note: {d['note']}")
    pj = d.get("prejudge")
    if pj:
        stale = " (stale)" if pj.get("proposal") != d.get("best") else ""
        ns = pj.get("naturalness", {}).get("score", "–")
        fs = pj.get("telephone", {}).get("fidelity", {}).get("score", "–")
        print(f"    prejudge{stale}: natural {ns}/5 · fidelity {fs}/5")
        for i in pj.get("naturalness", {}).get("issues", [])[:3]:
            print(f"      - {i['span']} — {i['why']}" if isinstance(i, dict) else f"      - {i}")
    if full:
        others = [p for p in d["proposals"] if p["valid"] and p["text"] != d["best"]]
        for k, p in enumerate(others, 1):
            print(f"  other valid {k}:")
            print(wrap(p["text"]))
            if p.get("drops"):
                print(f"    drops: {p['drops']}")

def main():
    cases = load()
    args = sys.argv[1:]
    if not args:
        for d in cases:
            show(d)
        print(f"\n{len(cases)} cases · set a verdict: paragraph-review.py N ideal|needs-fix [\"note\"]")
        return
    n = int(args[0])
    d = next((c for c in cases if c["index"] == n), None)
    if d is None:
        sys.exit(f"no case with index {n}")
    if len(args) == 1:
        show(d, full=True)
        return
    verdict = args[1]
    if verdict not in ("ideal", "needs-fix", "unreviewed"):
        sys.exit("verdict must be ideal | needs-fix | unreviewed")
    d["verdict"] = verdict
    if len(args) > 2:
        d["note"] = args[2]
    path = d.pop("_path")
    with open(path, "w") as f:
        yaml.safe_dump(d, f, allow_unicode=True, sort_keys=False, width=1000)
    print(f"[{n}] verdict: {verdict}" + (f" — {d.get('note')}" if d.get("note") else ""))

if __name__ == "__main__":
    main()
