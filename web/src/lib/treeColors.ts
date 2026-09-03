import type { TreeNode } from './types'

const FALLBACK = '#94a3b8'

export const TAG_COLOR: Record<string, string> = {
  DET: '#a855f7',
  DET_SG: '#a855f7',
  ADJ: '#eab308',
  ADJ_LONG: '#eab308',
  ADJ_CMP: '#eab308',
  THAN: '#eab308',
  MORE: '#eab308',
  SCALE: '#eab308',
  NOUN_SG: '#3b82f6',
  NOUN_PL: '#3b82f6',
  VERB_TRANS_BASE: '#22c55e',
  VERB_TRANS_3SG: '#22c55e',
  VERB_TRANS_ED: '#22c55e',
  VERB_TRANS_ING: '#22c55e',
  VERB_INTRANS_BASE: '#22c55e',
  VERB_INTRANS_3SG: '#22c55e',
  VERB_INTRANS_ED: '#22c55e',
  VERB_INTRANS_ING: '#22c55e',
  COPULA_SG: '#ec4899',
  COPULA_PL: '#ec4899',
  COPULA_SG_PAST: '#ec4899',
  COPULA_PL_PAST: '#ec4899',
  BE: '#ec4899',
  BECOME_SG: '#ec4899',
  BECOME_PL: '#ec4899',
  BECOME_PAST: '#ec4899',
  PREP_N: '#14b8a6',
  PREP_V: '#14b8a6',
  PRON_1: '#f97316',
  PRON_2: '#f97316',
  POSS: '#fb923c',
  MODAL_MUST: '#6366f1',
  MODAL_CAN: '#6366f1',
  MODAL_CAN_NEG: '#6366f1',
  CONJ: '#6b7280',
  NEG: '#ef4444',
  NEG_AUX_BASE: '#ef4444',
  NEG_AUX_3SG: '#ef4444',
  NEG_AUX_PAST: '#ef4444',
  SCONJ_COND: '#8b5cf6',
  THEN: '#8b5cf6',
  RESULT: '#8b5cf6',
  REASON: '#8b5cf6',
  QUANT_UNIV: '#f59e0b',
  QUANT_NEG: '#f59e0b',
  QUANT_EXIST: '#f59e0b',
  NUM_SG: '#0ea5e9',
  NUM_PL: '#0ea5e9',
  PERCENT: '#0ea5e9',
  APPROX: '#0ea5e9',
  ORD: '#0ea5e9',
  NAME: '#d946ef',
  COMMA: '#94a3b8',
  COLON: '#94a3b8',
}

export const LEGEND: { color: string; label: string }[] = [
  { color: '#a855f7', label: 'Determiner' },
  { color: '#eab308', label: 'Adjective / comparative' },
  { color: '#3b82f6', label: 'Noun' },
  { color: '#22c55e', label: 'Verb' },
  { color: '#ec4899', label: 'Copula' },
  { color: '#6366f1', label: 'Modal' },
  { color: '#14b8a6', label: 'Preposition' },
  { color: '#f97316', label: 'Pronoun / possessive' },
  { color: '#f59e0b', label: 'Quantifier' },
  { color: '#0ea5e9', label: 'Number / percent' },
  { color: '#6b7280', label: 'Conjunction' },
  { color: '#ef4444', label: 'Negation / auxiliary' },
  { color: '#8b5cf6', label: 'Connective (if / then / so / because)' },
  { color: '#d946ef', label: 'Name' },
]

export function tagColor(tag?: string): string {
  return (tag && TAG_COLOR[tag]) ?? FALLBACK
}

/** Every node (phrase) takes the color of its head child, so a Verb
 *  Phrase is the same color as the verb itself, a Noun Phrase as its noun.
 */
export function resolveColors(nodes: readonly TreeNode[]): Map<string, string> {
  const colors = new Map<string, string>()
  const resolve = (n: TreeNode): string => {
    const cached = colors.get(n.id)
    if (cached) return cached
    if (n.kind === 'word') {
      const c = tagColor(n.tag)
      colors.set(n.id, c)
      return c
    }
    const headChild = nodes.find((c) => c.parentId === n.id && c.head)
    const c = headChild ? resolve(headChild) : FALLBACK
    colors.set(n.id, c)
    return c
  }
  for (const n of nodes) resolve(n)
  return colors
}

/** Hex + alpha suffix, for link strokes etc. */
export function withAlpha(hex: string, alpha: string = '80'): string {
  return hex + alpha
}