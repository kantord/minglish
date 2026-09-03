#!/usr/bin/env python3
"""Pre-judge paragraph-repair cases with blind sub-agents, before the human review.

Two judgements per case, each made by an agent that sees only what a reader
would see (docs/prejudge.md has the protocol and the prompts):

  naturalness  one agent reads the best proposal and scores how natural it
               sounds (1–5) and names the unnatural spans.
  telephone    one agent reads ONLY the best proposal and explains its meaning
               in its own words; a second agent reads ONLY the original and
               that explanation and scores fidelity (1–5), listing what was
               lost, invented, or distorted.

  prejudge.py bundle [SOURCE-FILTER] > bundles.json   cases with a current best
  prejudge.py merge NAT.json EXPL.json FID.json > results.json   join the judges' outputs
  prejudge.py record RESULTS.json                      write judgements into the cases
  prejudge.py report                                   docs/prejudge-report.md + summary

RESULTS.json is a list of {index, source, naturalness: {score, issues: [..]},
telephone: {explanation, fidelity: {score, lost: [..], invented: [..],
distorted: [..]}}}; any key may be missing and is then left untouched.
"""
import glob, json, os, sys, yaml

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CASES = os.path.join(ROOT, "tests/paragraph-cases")
REPORT = os.path.join(ROOT, "docs/prejudge-report.md")


def load():
    out = []
    for f in sorted(glob.glob(os.path.join(CASES, "*.yaml"))):
        d = yaml.safe_load(open(f))
        d["_path"] = f
        out.append(d)
    return sorted(out, key=lambda d: (d["source"], int(d["index"])))


def best(d):
    return next((p for p in d.get("proposals", []) if p["text"] == d.get("best")), None)


def bundle(filt):
    out = []
    for d in load():
        if filt and filt not in d["source"]:
            continue
        b = best(d)
        if not b or not b.get("valid"):
            continue
        out.append({
            "index": int(d["index"]), "source": d["source"],
            "context_before": d.get("context_before", ""),
            "original": d["original"],
            "context_after": d.get("context_after", ""),
            "proposal": b["text"],
        })
    json.dump(out, sys.stdout, indent=2, ensure_ascii=False)
    print()


def merge(nat_path, expl_path, fid_path):
    """Join the three judge outputs (each a list keyed by index+source)."""
    key = lambda r: (int(r["index"]), r["source"])
    nat = {key(r): r for r in json.load(open(nat_path))}
    expl = {key(r): r for r in json.load(open(expl_path))}
    fid = {key(r): r for r in json.load(open(fid_path))}
    out = []
    for k in sorted(set(nat) | set(expl) | set(fid)):
        r = {"index": k[0], "source": k[1]}
        if k in nat:
            r["naturalness"] = nat[k]["naturalness"]
        if k in expl:
            r["telephone"] = {"explanation": expl[k]["explanation"]}
            if k in fid:
                r["telephone"]["fidelity"] = fid[k]["fidelity"]
        out.append(r)
    json.dump(out, sys.stdout, indent=2, ensure_ascii=False)
    print()


def record(path):
    results = json.load(open(path))
    cases = load()
    n = 0
    for r in results:
        d = next((c for c in cases if int(c["index"]) == int(r["index"]) and c["source"] == r["source"]), None)
        if not d:
            print(f"no case {r['source']} #{r['index']}", file=sys.stderr)
            continue
        pj = d.setdefault("prejudge", {})
        pj["proposal"] = d["best"]  # the text the judgement is about; stale if best changes
        for key in ("naturalness", "telephone"):
            if key in r:
                pj[key] = r[key]
        p = d["_path"]
        out = {k: v for k, v in d.items() if not k.startswith("_")}
        yaml.safe_dump(out, open(p, "w"), allow_unicode=True, sort_keys=False, width=88)
        n += 1
    print(f"recorded {n} judgement(s)")


def report():
    rows, nat, fid = [], [], []
    for d in load():
        pj = d.get("prejudge")
        if not pj:
            continue
        stale = pj.get("proposal") != d.get("best")
        ns = pj.get("naturalness", {}).get("score")
        fs = pj.get("telephone", {}).get("fidelity", {}).get("score")
        if ns is not None and not stale:
            nat.append(ns)
        if fs is not None and not stale:
            fid.append(fs)
        rows.append((d, pj, stale, ns, fs))
    lines = ["# Pre-judgement report", "",
             "Blind sub-agent judgements of the best proposal per paragraph case",
             "(protocol: docs/prejudge.md). Naturalness and fidelity are 1–5; a row",
             "marked *stale* was judged on an earlier best proposal.", ""]
    if nat:
        lines.append(f"Naturalness mean {sum(nat)/len(nat):.1f} over {len(nat)}; "
                     f"fidelity mean {sum(fid)/len(fid):.1f} over {len(fid)}.")
        lines.append("")
    lines += ["| case | natural | fidelity | human verdict | worst issue |", "|---|---|---|---|---|"]
    for d, pj, stale, ns, fs in rows:
        issues = pj.get("naturalness", {}).get("issues", [])
        lost = pj.get("telephone", {}).get("fidelity", {}).get("lost", [])
        worst = (issues[0] if issues else (lost[0] if lost else ""))
        if isinstance(worst, dict):
            worst = worst.get("span", "") + " — " + worst.get("why", "")
        tag = f"{os.path.basename(d['source'])} #{d['index']}" + (" *stale*" if stale else "")
        lines.append(f"| {tag} | {ns if ns is not None else '–'} | {fs if fs is not None else '–'} | {d.get('verdict','unreviewed')} | {str(worst)[:80]} |")
    lines.append("")
    for d, pj, stale, ns, fs in rows:
        lines.append(f"## {os.path.basename(d['source'])} #{d['index']}")
        lines.append("")
        lines.append(f"**Original.** {d['original']}")
        lines.append("")
        lines.append(f"**Proposal.** {pj.get('proposal','')}")
        lines.append("")
        nat_ = pj.get("naturalness")
        if nat_:
            lines.append(f"**Naturalness {nat_.get('score')}/5.**")
            for i in nat_.get("issues", []):
                lines.append(f"- {i['span']} — {i['why']}" if isinstance(i, dict) else f"- {i}")
            lines.append("")
        tel = pj.get("telephone")
        if tel:
            lines.append(f"**Telephone.** {tel.get('explanation','')}")
            lines.append("")
            f_ = tel.get("fidelity", {})
            lines.append(f"**Fidelity {f_.get('score')}/5.**")
            for k in ("lost", "invented", "distorted"):
                for x in f_.get(k, []):
                    lines.append(f"- {k}: {x}")
            lines.append("")
    open(REPORT, "w").write("\n".join(lines))
    print(f"prejudge: {len(rows)} judged; naturalness mean {sum(nat)/len(nat) if nat else 0:.1f}, "
          f"fidelity mean {sum(fid)/len(fid) if fid else 0:.1f}; report in docs/prejudge-report.md")


if __name__ == "__main__":
    a = sys.argv[1:]
    if not a or a[0] == "bundle":
        bundle(a[1] if len(a) > 1 else None)
    elif a[0] == "merge":
        merge(*a[1:4])
    elif a[0] == "record":
        record(a[1])
    elif a[0] == "report":
        report()
    else:
        sys.exit(__doc__)
