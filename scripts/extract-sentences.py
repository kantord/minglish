#!/usr/bin/env python3
"""Extract prose sentences from a markdown file, one per line, for linting.
Backticked spans become minglish quoted identifiers; bullets are unwrapped;
list-numbering fragments and abbreviation splits are avoided."""
import re, sys

text = open(sys.argv[1]).read()
prose = []
for para in text.split("\n\n"):
    para = para.strip()
    if not para or para.startswith("#") or para.startswith(("Date:", "Status:")):
        continue
    lines = para.splitlines()
    # an Enumeration block (intro ending in ":" + "- item" lines) stays one unit
    if len(lines) > 1 and lines[0].rstrip().endswith(":") and all(l.lstrip().startswith("- ") for l in lines[1:]):
        block = "\n".join([lines[0].strip()] + [l.strip() for l in lines[1:]])
        block = re.sub(r"`([^`]+)`", r'"\1"', block)
        print(block.replace("\n", "\u23ce"))  # one line: ⏎ marks the item breaks
        continue
    # unwrap; strip bullet/number markers at line starts
    lines = [re.sub(r"^\s*(?:[-*]|\d+\.)\s+", "", l) for l in lines]
    prose.append(re.sub(r"\s+", " ", " ".join(lines)))
body = " ".join(prose)
# backticked spans are verbatim identifiers → minglish double quotes
body = re.sub(r"`([^`]+)`", r'"\1"', body)
# strip markdown emphasis markers
body = re.sub(r"\*\*?", "", body)
# protect abbreviations from the sentence splitter
body = body.replace("e.g.", "e~g~").replace("i.e.", "i~e~").replace("cf.", "cf~")
for s in re.split(r"(?<=[.!?]) ", body):
    s = s.strip().rstrip(".").strip().replace("e~g~", "e.g.").replace("i~e~", "i.e.").replace("cf~", "cf.")
    if not s or len(s.split()) < 3 or re.fullmatch(r"[\d\W]+", s):
        continue  # drop bare numbers / list fragments / punctuation shards
    print(s)
