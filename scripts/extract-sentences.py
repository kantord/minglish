#!/usr/bin/env python3
"""Extract prose sentences from a markdown file, one per line, for linting.
Backticked spans become minglish quoted identifiers; bullets are unwrapped;
list-numbering fragments and abbreviation splits are avoided."""
import re, sys

text = open(sys.argv[1]).read()
prose = []
blocks = []
for para in text.split("\n\n"):
    para = para.strip()
    if not para or para.startswith("#") or para.startswith(("Date:", "Status:")):
        continue
    lines = [l.strip() for l in para.splitlines()]
    if lines and all(l.startswith(("Given ", "When ", "Then ", "And ", "Feature:", "Scenario:")) for l in lines):
        blocks.append(re.sub(r"`([^`]+)`", r'"\1"', "\n".join(lines)).replace("\n", "\u23ce"))  # Step Block
        continue
    # an Enumeration block (intro line ending in ":" + "- item" lines) stays one
    # unit, whether it is the whole paragraph or ends it
    k = next((i for i, l in enumerate(lines) if l.endswith(":") and i + 1 < len(lines) and all(x.startswith("- ") for x in lines[i + 1:])), None)
    if k is not None:
        text = re.sub(r"\s+", " ", " ".join(re.sub(r"^(?:[-*]|\d+\.)\s+", "", l) for l in lines[:k + 1]))
        cut = text.rfind(". ")
        before, intro = (text[:cut + 1], text[cut + 2:]) if cut >= 0 else ("", text)
        block = re.sub(r"`([^`]+)`", r'"\1"', "\n".join([intro] + lines[k + 1:]))
        blocks.append(block.replace("\n", "\u23ce"))  # one line: ⏎ marks the item breaks
        lines = [before] if before.strip() else []
        if not lines:
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
for b in blocks:
    print(b)
