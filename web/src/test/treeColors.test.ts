import { describe, expect, it } from 'vitest'
import type { TreeNode } from '../lib/types'
import { resolveColors, tagColor, withAlpha } from '../lib/treeColors'

const NODES: TreeNode[] = [
  { id: 'n0', parentId: null, name: 'Statement', kind: 'node', head: false },
  { id: 'n1', parentId: 'n0', name: 'Noun Phrase', kind: 'node', head: false },
  { id: 'n2', parentId: 'n1', name: 'the', kind: 'word', head: false, tag: 'DET' },
  { id: 'n3', parentId: 'n1', name: 'agent', kind: 'word', head: true, tag: 'NOUN_SG' },
  { id: 'n4', parentId: 'n0', name: 'Verb Phrase', kind: 'node', head: true },
  { id: 'n5', parentId: 'n4', name: 'reads', kind: 'word', head: true, tag: 'VERB_TRANS_3SG' },
  { id: 'n6', parentId: 'n4', name: 'the', kind: 'word', head: false, tag: 'DET' },
  { id: 'n7', parentId: 'n6', name: 'file', kind: 'word', head: false, tag: 'NOUN_SG' },
]

describe('tree colors', () => {
  it('maps form tags to a stable color', () => {
    expect(tagColor('NOUN_SG')).toBe(tagColor('NOUN_PL'))
    expect(tagColor('VERB_TRANS_3SG')).toBe(tagColor('VERB_INTRANS_ED'))
    expect(tagColor('NOUN_SG')).not.toBe(tagColor('VERB_TRANS_3SG'))
    expect(tagColor(undefined)).toBe('#94a3b8')
  })

  it('gives a phrase the color of its head child', () => {
    const colors = resolveColors(NODES)
    const verbColor = tagColor('VERB_TRANS_3SG')
    const nounColor = tagColor('NOUN_SG')
    const detColor = tagColor('DET')
    expect(colors.get('n0')).toBe(verbColor)  // Statement heads to the Verb Phrase
    expect(colors.get('n4')).toBe(verbColor)   // Verb Phrase heads to the verb
    expect(colors.get('n1')).toBe(nounColor)  // Noun Phrase heads to the noun
    expect(colors.get('n2')).toBe(detColor)
    expect(colors.get('n6')).toBe(detColor)   // object determiner
  })

  it('appends an alpha suffix for links', () => {
    expect(withAlpha('#22c55e')).toBe('#22c55e80')
  })
})