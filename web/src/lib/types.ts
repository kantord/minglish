export type Kind = 'Clean' | 'Word' | 'Style' | 'Ambiguous' | 'Unknown'

export interface Metrics {
  peak_open_deps: number
  max_dep_len: number
  embedding_depth: number
  right_branching: number
  fronted: number
}

export interface TokenRow {
  pos: number
  word: string
  tag: string
}

export interface TreeNode {
  id: string
  parentId: string | null
  name: string
  kind: 'node' | 'word'
  head: boolean
  lemma?: string
  tag?: string
  gloss?: string
  full?: string | null
}

export interface DiagnosisResult {
  kind: Kind
  messages: string[]
  readings?: number
  metrics?: Metrics
  tree?: { nodes: TreeNode[] }
  tokens: TokenRow[]
}

export const KIND_META: Record<Kind, { label: string; tone: 'success' | 'error' | 'warn' }> = {
  Clean: { label: 'parses uniquely', tone: 'success' },
  Word: { label: 'word-level rejection', tone: 'error' },
  Style: { label: 'style rejection', tone: 'warn' },
  Ambiguous: { label: 'ambiguous — several readings', tone: 'warn' },
  Unknown: { label: 'not recognizable', tone: 'error' },
}