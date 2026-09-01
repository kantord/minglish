#!/usr/bin/env python3
"""Document-level lint (measurement only, never gating).

For a markdown file: per-sentence verdicts and parse rate; topic continuity
(does the subject noun of each sentence appear in the sentence before it?);
and a relation inventory — English coherence connectives in the source,
grouped by relation type, marked by whether minglish has a form for them.

Usage: lint-file.py FILE.md [--brief]   (brief: no per-sentence listing)
"""
import re, subprocess, sys, collections, os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# relation type -> (connectives, minglish form or None)
RELATIONS = {
    "result":         (["so", "therefore", "thus", "hence", "consequently", "as a result"], ", so"),
    "reason":         (["because", "since", "for"], ", because"),
    "contrast":       (["but", "however", "yet", "whereas", "while", "instead"], "but"),
    "concession":     (["although", "though", "even though", "despite", "nevertheless", "still"], None),
    "condition":      (["if", "unless", "when", "whenever", "otherwise"], "if …, then"),
    "sequence":       (["then", "after", "before", "next", "finally", "first", "later", "once"], None),
    "exemplification":(["for example", "e.g.", "such as", "for instance"], None),
    "purpose":        (["so that", "in order to"], None),
    "elaboration":    (["in other words", "i.e.", "that is", "also", "moreover", "furthermore", "specifically", "namely"], None),
}
SKIP_FOR = True  # "for" is mostly a preposition; counted only as "for" + clause-ish is unreliable → skip

def paragraphs(text):
    out = []
    for para in text.split("\n\n"):
        para = para.strip()
        if not para or para.startswith("#") or para.startswith(("Date:", "Status:")) or para.startswith("|"):
            continue
        lines = [re.sub(r"^\s*(?:[-*]|\d+\.)\s+", "", l) for l in para.splitlines()]
        out.append(re.sub(r"\s+", " ", " ".join(lines)))
    return out

def sentences(para):
    body = re.sub(r"`([^`]+)`", r'"\1"', para)
    body = re.sub(r"\*\*?", "", body)
    body = body.replace("e.g.", "e~g~").replace("i.e.", "i~e~").replace("cf.", "cf~")
    out = []
    for s in re.split(r"(?<=[.!?]) ", body):
        s = s.strip().rstrip(".").strip().replace("e~g~", "e.g.").replace("i~e~", "i.e.").replace("cf~", "cf.")
        if not s or len(s.split()) < 3 or re.fullmatch(r"[\d\W]+", s):
            continue
        out.append(s)
    return out

def lint(sents):
    """Batch the sentences through diagnose; return [(ok, first line of detail)]."""
    if not sents:
        return []
    r = subprocess.run(["cargo", "run", "-q", "-p", "diagnose", "--", *sents],
                       cwd=ROOT, capture_output=True, text=True)
    verdicts, cur = [], None
    for line in (r.stdout + r.stderr).splitlines():
        if line.startswith(("✓", "✗")):
            cur = [line.startswith("✓"), ""]
            verdicts.append(cur)
        elif cur is not None and line.strip() and not cur[1]:
            cur[1] = line.strip()
    return [(ok, d) for ok, d in verdicts]

def load_lexicon():
    lemma, tag = {}, {}
    for line in open(os.path.join(ROOT, "lexicon.tsv")):
        if line.startswith("#"):
            continue
        f = line.rstrip("\n").split("\t")
        if len(f) == 4 and f[1] == "form":
            tag[f[0]] = f[2]; lemma[f[0]] = f[3]
    return lemma, tag

def content_lemmas(sent, lemma, tag):
    """Nouns (by lemma) and names in a sentence; subject = first noun/name."""
    toks = re.findall(r'"[^"]*"|[A-Za-z][\w\-\']*', sent)
    nouns, subj = [], None
    for i, t in enumerate(toks):
        if t.startswith('"') or (t[0].isupper() and i > 0):
            key = t.strip('"').lower()
        else:
            w = t.lower()
            if tag.get(w, "").startswith("NOUN"):
                key = lemma[w]
            else:
                continue
        nouns.append(key)
        if subj is None:
            subj = key
    return subj, set(nouns)

def relation_inventory(text):
    low = " " + re.sub(r"\s+", " ", text.lower()) + " "
    counts = collections.OrderedDict()
    for rel, (words, form) in RELATIONS.items():
        n = 0
        for w in words:
            if w == "for" and SKIP_FOR:
                continue
            pat = r"(?<![\w-])" + re.escape(w) + r"(?![\w-])"
            if w == "then":
                pat = r"(?<!, )" + pat  # ", then" is the conditional's marker, not sequence
            n += len(re.findall(pat, low))
        counts[rel] = (n, form)
    return counts

def main():
    path = sys.argv[1]
    brief = "--brief" in sys.argv
    text = open(path).read()
    lemma, tag = load_lexicon()
    paras = paragraphs(text)
    all_sents = [(pi, s) for pi, p in enumerate(paras) for s in sentences(p)]
    verdicts = lint([s for _, s in all_sents])
    ok = sum(1 for v, _ in verdicts if v)
    print(f"# Document lint: {os.path.relpath(path, ROOT)}\n")
    print(f"- Sentences: {len(all_sents)}; parse: {ok}/{len(all_sents)}"
          f" ({100*ok/max(1,len(all_sents)):.0f}%)")
    # topic continuity within paragraphs
    pairs, breaks = 0, []
    prev = None
    for (pi, s), (v, _) in zip(all_sents, verdicts):
        subj, nouns = content_lemmas(s, lemma, tag)
        if prev is not None and prev[0] == pi and subj is not None:
            pairs += 1
            if subj not in prev[2]:
                breaks.append((subj, s))
        prev = (pi, subj, nouns)
    cont = pairs - len(breaks)
    print(f"- Topic continuity: {cont}/{pairs} consecutive pairs share the subject with the sentence before"
          f" ({100*cont/max(1,pairs):.0f}%)")
    inv = relation_inventory(text)
    print("- Relation inventory (source connectives, indicative counts — hand-check before deciding; ✓ = minglish has a form):")
    for rel, (n, form) in inv.items():
        if n:
            print(f"  - {rel}: {n}" + (f" ✓ {form}" if form else " ✗ no form"))
    if not brief:
        print("\n## Sentences\n")
        for (pi, s), (v, d) in zip(all_sents, verdicts):
            mark = "✓" if v else "✗"
            print(f"- {mark} {s}" + ("" if v else f"\n  - {d}"))
        if breaks:
            print("\n## Topic breaks (subject not in the previous sentence)\n")
            for subj, s in breaks:
                print(f"- *{subj}*: {s}")

if __name__ == "__main__":
    main()
