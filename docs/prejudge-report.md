# Pre-judgement report

Blind sub-agent judgements of the best proposal per paragraph case
(protocol: docs/prejudge.md). Naturalness and fidelity are 1–5; a row
marked *stale* was judged on an earlier best proposal.

Naturalness mean 3.2 over 6; fidelity mean 4.0 over 6.

| case | natural | fidelity | human verdict | worst issue |
|---|---|---|---|---|
| dogfood-source-0006.md #2 | 3 | 3 | unreviewed | for every decision of a Sentence Shape — odd of-phrase; a human would write 'for |
| dogfood-source-0006.md #3 | 2 | 4 | unreviewed | The Grammar must prefer a structure. — incomplete on its own; the point (prefer  |
| dogfood-source-0006.md #4 | 4 | 4 | unreviewed | the ideal branching is not available — 'the ideal branching' is an odd nominal;  |
| dogfood-source-0006.md #5 | 3 | 4 | unreviewed | The Context Need of a sentence is the prior text of the sentence. — definition w |
| dogfood-source-0006.md #11 | 3 | 4 | unreviewed | The results of the research contain the metrics for the Cognitive Load. — of-cha |
| dogfood-source-0006.md #12 | 4 | 5 | unreviewed | "measurably easier" beats "natural sounding" — quoted adjective phrases used as  |

## dogfood-source-0006.md #2

**Original.** Ranked criteria for every syntactic-structure decision, in order:

**Proposal.** The Grammar defines 5 criteria for every decision of a Sentence Shape. The first criterion is the Cognitive Load. The second criterion is the familiarity of the construction. The third criterion is the Context Need. The 4th criterion is the density. The 5th criterion is the expressiveness. A later criterion does not override a prior criterion.

**Naturalness 3/5.**
- for every decision of a Sentence Shape — odd of-phrase; a human would write 'for deciding a Sentence Shape' or 'for every Sentence Shape decision'
- The first criterion is the Cognitive Load. The second criterion is the familiarity of the construction. The third criterion is the Context Need. The 4th criterion is the density. The 5th criterion is the expressiveness. — the same frame 'The Nth criterion is X' repeated five times; reads as a list forced into prose
- The 4th criterion — switches from spelled-out 'first/second/third' to digit '4th' mid-series
- the density — definite article on a bare abstract noun; 'density' alone is the natural form
- the expressiveness — definite article on a bare abstract noun; 'expressiveness' alone is the natural form

**Telephone.** This paragraph says the Grammar has a fixed set of five criteria that it applies whenever it decides something about a Sentence Shape. It lists them in a definite order: first the Cognitive Load (how much work a sentence demands of the reader), second how familiar the construction is, third the Context Need (how much prior text the sentence depends on), fourth density, and fifth expressiveness. The closing sentence states that a criterion lower in the list never overrides one higher up, so the list is a strict priority ranking rather than a set of factors to be weighed together. Nothing here defines density or expressiveness; the reader is expected to take those words at face value.

**Fidelity 3/5.**
- invented: The count 'five' criteria (the original paragraph gives no number; a later paragraph even speaks of 'these three criteria')
- invented: The named list of criteria (cognitive load, familiarity, context need, density, expressiveness) — the paragraph is only the intro line before the list
- invented: A closing sentence stating that a lower criterion never overrides a higher one (strict lexical priority) — the original says only 'ranked ... in order'
- invented: The remark that density and expressiveness are left undefined
- distorted: 'Ranked ... in order' (a priority ordering) is hardened into an explicit rule that a lower criterion can never override a higher one

## dogfood-source-0006.md #3

**Original.** Cognitive load first. Prefer the structure that minimizes open dependencies, dependency length, and embedding — branching rules exist to aid human and LLM understanding, per cognitive-science results, not per taste or tradition.

**Proposal.** The Cognitive Load is the first criterion. The Grammar must prefer a structure. The structure reduces every Open Dependency. The structure reduces every Dependency Length. The structure reduces every Embedding Depth. The rules of the branching exist. The rules help the reader. The rules help the Language Model. The results of the research ground the rules. The rules do not come from the taste. The rules do not come from the tradition.

**Naturalness 2/5.**
- The Grammar must prefer a structure. — incomplete on its own; the point (prefer structures that reduce load) is split off into three following sentences
- The structure reduces every Open Dependency. The structure reduces every Dependency Length. The structure reduces every Embedding Depth. — same subject and verb three sentences in a row; should be one sentence with a coordinated object
- reduces every Open Dependency — 'every' is the wrong quantifier; one reduces the number of Open Dependencies, not each one
- reduces every Dependency Length — same wrong quantifier; a length is shortened, not 'every length reduced'
- The rules of the branching exist. — a bare existence claim nobody would write; 'the branching' is also an odd definite noun
- The rules help the reader. The rules help the Language Model. — same subject and verb repeated; should be one sentence
- The results of the research ground the rules. — of-chain plus 'ground' in a sense rarely used in plain technical prose
- The rules do not come from the taste. The rules do not come from the tradition. — definite article on 'taste' and 'tradition' is wrong; two sentences that should be one

**Telephone.** This paragraph elaborates the top-ranked criterion, Cognitive Load. It instructs the Grammar to prefer whichever structure reduces the Open Dependencies a reader must hold in memory, shortens the Dependency Lengths between related words, and lowers the Embedding Depth of clauses inside clauses. One sentence in the middle is hard to read: it only asserts that rules about branching exist, without saying what they are; I take it to mean the Grammar has rules governing how sentence structure branches, and the following sentences are about those rules. Those rules are said to serve two audiences, the human reader and a Language Model. Finally, the paragraph claims the rules are justified by research findings, and explicitly denies that they come from personal taste or from tradition. A minor ambiguity: reducing every Open Dependency could mean reducing the number of them or reducing the burden of each; either reading fits.

**Fidelity 4/5.**
- lost: The justification is specifically 'cognitive-science results'; the summary generalizes to 'research findings'

## dogfood-source-0006.md #4

**Original.** Fallback: the most common, least confusing construction. When the ideal branching is unavailable, choose the most frequent standard-English pattern — familiarity is itself a processing aid, and surprise is a cost.

**Proposal.** If the ideal branching is not available, then the writer chooses the common structure of English. The familiarity of the structure helps the reader. The surprise of a new structure costs the reader.

**Naturalness 4/5.**
- the ideal branching is not available — 'the ideal branching' is an odd nominal; 'the ideal branching structure' or 'ideal branching' would read naturally
- The surprise of a new structure costs the reader. — 'costs the reader' with no object is a stiff idiom; stiff but tolerable

**Telephone.** This paragraph gives a fallback rule. When the ideal branching structure cannot be used, the writer should fall back to whatever structure is common in ordinary English. The justification is that a familiar structure helps the reader, while an unfamiliar or novel structure imposes a cost through surprise. It presupposes that an ideal branching has been identified elsewhere; this paragraph alone does not say what it is.

**Fidelity 4/5.**
- lost: 'least confusing' as part of the fallback target
- distorted: 'the most frequent standard-English pattern' (a superlative: pick the single most common) is softened to 'whatever structure is common in ordinary English'

## dogfood-source-0006.md #5

**Original.** Minimal context need. A sentence should be interpretable with as little prior context as possible. This is why anaphora is banned (ADR 0002), why nouns are repeated, and why scope rules are fixed rather than context-dependent.

**Proposal.** The Context Need of a sentence is the prior text of the sentence. A good sentence has a small Context Need. The language bans every Anaphoric Pronoun, so the writer repeats every noun. The language fixes the rules of the scope, so the reader does not depend on the prior context.

**Naturalness 3/5.**
- The Context Need of a sentence is the prior text of the sentence. — definition written as an equation with a repeated of-phrase on both sides
- the rules of the scope — definite article on 'scope'; 'the scope rules' or 'scoping rules' is the natural form
- so the writer repeats every noun — 'every noun' is an overstatement for 'repeats the noun instead of using a pronoun'; reads mechanical

**Telephone.** This paragraph defines and motivates the Context Need criterion. The Context Need of a sentence is the earlier text a reader has to have seen to understand it, and a good sentence should need as little of that as possible. It then names two design choices that keep Context Need small. First, because the language bans Anaphoric Pronouns such as it or they, the writer repeats the noun each time instead of pointing back to it. Second, because the language fixes its scope rules in advance, the reader does not have to lean on surrounding context to work out what a word applies to. The phrase about repeating every noun is slightly loose: it presumably means restating the referent wherever a pronoun would otherwise appear, not literally duplicating every noun in the text.

**Fidelity 4/5.**
- lost: The reference to ADR 0002 as the source of the anaphora ban
- invented: The gloss 'Anaphoric Pronouns such as it or they' narrows 'anaphora' to pronouns and adds examples the original does not give
- distorted: The original lists three parallel consequences of the criterion (anaphora ban, repeated nouns, fixed scope rules); the summary merges the first two into a cause-effect chain (nouns are repeated because anaphora is banned) and presents two design choices instead of three

## dogfood-source-0006.md #11

**Original.** Metrics for (1) already exist in the research findings and can be implemented over parse output when the grammar tier arrives.

**Proposal.** The results of the research contain the metrics for the Cognitive Load. A future Grammar parses the sentences, so the Linter uses the metrics on the output of the Parse.

**Naturalness 3/5.**
- The results of the research contain the metrics for the Cognitive Load. — of-chain plus 'contain' where 'the research provides metrics for Cognitive Load' is what a human would write
- A future Grammar parses the sentences, so the Linter uses the metrics — 'so' asserts a consequence where a temporal 'once a future Grammar parses sentences' was meant; reads as a non sequitur
- uses the metrics on the output of the Parse — 'uses X on Y' plus another of-phrase is stiff; 'applies the metrics to the Parse' is natural

**Telephone.** This paragraph makes two points. The research findings the project relies on include concrete metrics for Cognitive Load. And the plan is that once a future version of the Grammar actually parses sentences, the Linter will be able to compute those metrics from the parse output. The wording uses present tense, so it could be misread as describing something that already works, but the word future signals this is a projected capability, not current behavior. It also implies the current Grammar does not yet produce parses the Linter can measure in this way.

**Fidelity 4/5.**
- invented: 'the Linter' as the component that will compute the metrics — the original names no implementer, only that metrics 'can be implemented over parse output'
- distorted: 'when the grammar tier arrives' (a not-yet-existing tier) is rendered as 'once a future version of the Grammar actually parses sentences' — close, but shifts from 'the tier does not exist' to 'the Grammar exists but does not yet parse'

## dogfood-source-0006.md #12

**Original.** "Natural-sounding" loses to "measurably easier" whenever they conflict — but criterion 2 keeps the loss small.

**Proposal.** If "natural sounding" conflicts with "measurably easier", then "measurably easier" beats "natural sounding". The second criterion limits the loss.

**Naturalness 4/5.**
- "measurably easier" beats "natural sounding" — quoted adjective phrases used as nouns four times in one sentence; readable but visibly a workaround
- The second criterion limits the loss. — stray reference; 'the loss' has no antecedent in this paragraph and 'the second criterion' is unnamed here

**Telephone.** This paragraph resolves a specific conflict. When making a sentence sound natural conflicts with making it measurably easier to read, the measurable ease wins. The second sentence then says the second criterion limits the loss; read on its own, the paragraph does not say what the second criterion is, so the reader has to know from elsewhere that it is the familiarity of the construction. The most natural reading is that the familiarity criterion acts as a brake: it caps how much naturalness may be sacrificed in pursuit of measurable ease. A less likely reading is that the loss is some other cost that the second criterion bounds; the paragraph does not say which loss it means.

**Fidelity 5/5.**
