# 0012 — Translation-pair validity: loss taxonomy, declared not silent

Date: 2026-09-01
Status: accepted (corpus methodology policy)

## Context

The maintainers audited the file "corpus/pairs.tsv" and found 3 pairs with
a Propositional Loss. The first pair dropped a universal quantifier. The
second pair changed a causal claim. The third pair invented a doer. The
tool "textcost" rewarded the 3 pairs, because a loss of the meaning is
cheap. A dense text can carry a silent loss, so the format of the corpus
must catch every silent loss.

## Decision

The maintainers judge every Translation Pair. A loss has 3 classes:
- the Propositional Loss
- the Register Loss
- the Structure Loss

A claim has 5 parts:
- the quantifier
- the scope
- the causal structure
- the doers
- the Tense

A Propositional Loss changes a part of the claim, so the loss is not
acceptable. If a Translation Pair has a Propositional Loss, then the
maintainers fix the pair. If the maintainers cannot fix the pair, then the
file "corpus/untranslatable.tsv" takes the pair with the reason. If a
translation drops the word "all", then the translation is wrong. A wrong
translation is not dense.

A Register Loss drops a marker of the register. The register has 3 markers:
- the politeness
- the emphasis
- the connectives

The maintainers tolerate a Register Loss but regret the loss. The 4
criteria of the decision "0006" outrank the expressiveness, so the loss is
acceptable. The Translation Pair declares every Register Loss in the column
"drops". If a later reader finds a silent Register Loss, then the corpus
has a bug. If the 4 criteria allow the affect, then the maintainers prefer
a translation with the affect.

A Structure Loss changes the order of the information. The language bans
every Passive, so the rewrite of a Passive changes the topic of the
sentence. The rewrite is a Structure Loss. A Structure Loss is acceptable
but needs a declaration.

The file "corpus/pairs.tsv" gains a third column. The column "drops" holds
a list of the Declared Losses. A comma separates the items. If the column
is empty, then the pair does not have a loss. The tool "textcost" shows the
declarations with every entry, so the reader sees the ratio of the expense
with the loss of the meaning.

## Consequences

- The maintainers moved 3 pairs into the file "corpus/untranslatable.tsv".
  One pair dropped a quantifier. 2 pairs invented the doer of a Passive.
  The ratio of the expense grew for the whole corpus but became honest.
- The translation "you cannot" narrows the sentence "it is not possible" to
  the addressee. The rewrite is a Register Loss, so the maintainers
  declared the loss. A later decision can revisit the rewrite.
- The file "docs/ideas.md" describes a guard with embeddings. The
  maintainers deferred the guard. The guard becomes the candidate for a
  future tool. The tool must catch every silent Propositional Loss.
