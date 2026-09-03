---
title: Kitchen sink
date: 2026-09-04
---

# Kitchen sink — every markdown feature `mdblocks.py` handles

The agent reads the file.

## A blockquote

> The agent stores the file. The file is old.

## Code

```python
def add(a, b):
    return a + b
```

## A table

| a | b |
|---|---|
| 1 | 2 |

## An Enumeration

The language has 3 conjunctions:
- "and"
- "or"
- "but"

## A Step Block

Given the agent stores the file
When the test fails
Then the agent retries the request

## Lists

- the agent stores the file
- the agent deletes the report

The agent has 2 reports:
- [ ] the old report
- [x] the new report

## Nested lists

- the agent has 2 reports:
  - the old report
  - the new report
- the agent has 2 reports with no colon
  - the old report
  - the new report

## Links and a standalone image

The agent reads [the guide](https://example.com/guide).

![A diagram of the pipeline](diagram.png)

## An inline image

The report has a chart ![a chart](chart.png) attached.

## Raw HTML

<details>
<summary>Click to expand</summary>

The agent stores the report.

</details>

## Footnotes

The agent reads the file[^1]. The file is old.

[^1]: A footnote about the agent.

## Text formatting

Bold **text** and italic *text* and ~~struck~~ text and `inline code`.

## A horizontal rule

---

The agent stores the file after the rule.
