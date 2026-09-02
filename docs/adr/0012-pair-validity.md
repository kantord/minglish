# 0012 — Translation-pair validity: loss taxonomy, declared not silent

Date: 2026-09-01
Status: accepted (corpus methodology policy)

## Context

The maintainers audited the file "corpus/pairs.tsv". 3 Translation Pairs
changed a claim. One pair dropped a universal quantifier. One pair changed a causal claim. One pair invented a doer. The metric of the expense rewarded the 3 pairs, because a short text is cheap. A dense text can cover a loss. The
system must catch the loss.

## Decision

The maintainers judge every Translation Pair. The language names 3 classes
of a loss:
- the Propositional Loss
- the Register Loss
- the Structure Loss

A Propositional Loss is not acceptable. If a Translation Pair has a
Propositional Loss, then the maintainers fix the pair. If the maintainers
cannot fix the pair, then the maintainers move the pair into the file
"corpus/untranslatable.tsv". If a translation drops the word "all", then
the translation is wrong.

A Register Loss is acceptable. The maintainers regret every Register Loss.
The Translation Pair declares the loss in the column "drops". If a loss
does not have a declaration, then the corpus has a bug. If a translation
keeps the affect, then the maintainers prefer the translation.

A Structure Loss is acceptable. The Translation Pair declares the
Structure Loss.

The file "corpus/pairs.tsv" has the column "drops". The column contains
every Declared Loss of the pair. The tool "textcost" shows the declarations with the expense, so the reader sees the expense with the loss.

## Consequences

- The maintainers moved 3 pairs into the file "corpus/untranslatable.tsv".
  The ratio of the expense grew. The ratio is honest.
- The translation "you cannot" narrows the sentence "it is not possible" to
  the reader. The rewrite is a Register Loss. The maintainers declared the
  loss.
- The file "docs/ideas.md" describes a guard. The guard uses embeddings.
  The guard can catch a Propositional Loss.
