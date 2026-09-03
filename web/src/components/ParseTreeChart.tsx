import { useMemo } from 'react'
import { defineChart, dot, link, text } from '@tanstack/charts'
import { treeLayout } from '@tanstack/charts/hierarchy/tree'
import { scaleLinear } from '@tanstack/charts/scales/linear'
import { Chart } from '@tanstack/charts/react'
import type { TreeNode } from '@/lib/types'
import { LEGEND, resolveColors, withAlpha } from '@/lib/treeColors'

const WORD_DX = 8
const GLOSS_DY = 15
const ROW_DEPTH = 54

export function ParseTreeChart({ nodes }: { nodes: TreeNode[] }) {
  const colors = useMemo(() => resolveColors(nodes), [nodes])

  const definition = useMemo(() => {
    const hierarchy = treeLayout(nodes, {
      id: 'id',
      parentId: 'parentId',
      orientation: 'top',
    })

const colorOf = (n: { data: TreeNode | null }) =>
      colors.get(n.data?.id ?? '') ?? 'var(--muted-foreground)'

    const internal = hierarchy.nodes
      .filter((n) => n.data?.kind !== 'word')
      .map((n) => ({ ...n, fill: colorOf(n) }))
    const words = hierarchy.nodes
      .filter((n) => n.data?.kind === 'word')
      .map((n) => ({ ...n, fill: colorOf(n) }))
    const glosses = words.map((n) => ({
      ...n,
      key: `${n.id}:gloss`,
      text: n.data?.gloss ?? '',
      dy: GLOSS_DY,
      fill: 'var(--muted-foreground)',
    }))

    const byColor = <T extends { fill: string }>(rows: readonly T[]): { fill: string; rows: T[] }[] => {
      const groups = new Map<string, T[]>()
      for (const r of rows) {
        groups.set(r.fill, [...(groups.get(r.fill) ?? []), r])
      }
      return [...groups.entries()].map(([fill, rows]) => ({ fill, rows }))
    }
    const internalGroups = byColor(internal)
    const wordGroups = byColor(words)

    const links = hierarchy.links.map((l) => {
      const c = colors.get(l.targetNode.data?.id ?? '')
      return { ...l, color: c ? withAlpha(c) : 'var(--border)' }
    })

    const label = (n: { data: { name?: string } | null; name: string }) =>
      n.data?.name ?? n.name

    const internalMark = (g: { fill: string; rows: { x: number; y: number; id: string }[] }) => [
      dot(g.rows, { x: 'x', y: 'y', key: 'id', fill: g.fill, r: 2.5 }),
      text(g.rows, {
        x: 'x',
        y: 'y',
        key: 'id',
        text: label,
        fill: g.fill,
        fontSize: 12,
        fontWeight: 600,
        anchor: 'start',
        dx: WORD_DX,
      }),
    ]
    const wordMark = (g: { fill: string; rows: { x: number; y: number; id: string }[] }) => [
      dot(g.rows, { x: 'x', y: 'y', key: 'id', fill: g.fill, r: 3.5 }),
      text(g.rows, {
        x: 'x',
        y: 'y',
        key: 'id',
        text: label,
        fill: g.fill,
        fontSize: 13,
        fontWeight: 600,
        anchor: 'start',
        dx: WORD_DX,
      }),
    ]

    return defineChart({
      marks: [
        link(links, {
          x1: 'x1',
          y1: 'y1',
          x2: 'x2',
          y2: 'y2',
          key: 'id',
          stroke: (link) => link.color ?? 'var(--border)',
          strokeWidth: 1.5,
        }),
        ...internalGroups.flatMap(internalMark),
        ...wordGroups.flatMap(wordMark),
        text(glosses, {
          x: 'x',
          y: 'y',
          key: 'key',
          text: 'text',
          fill: 'var(--muted-foreground)',
          fontSize: 10.5,
          fontWeight:   400,
          anchor: 'start',
          dx: WORD_DX,
          dy: GLOSS_DY,
        }),
      ],
      scales: {
        x: { scale: scaleLinear },
        y: { scale: scaleLinear },
      },
      guides: false,
      focus: false,
      margin: { top:   28, right:     320, bottom:   16, left:   12 },
    })
  }, [nodes, colors])

  const maxDepth = useMemo(() => {
    const depth = new Map<string, number>()
    for (const n of nodes) {
      depth.set(n.id, n.parentId === null ? 0 : (depth.get(n.parentId) ?? 0) + 1)
    }
    return Math.max(0, ...depth.values())
  }, [nodes])

  return (
    <div className="flex flex-col gap-3">
      <div className="overflow-x-auto">
        <Chart
          definition={definition}
          height={maxDepth * ROW_DEPTH + 120}
          ariaLabel="Parse tree of the sentence"
        />
      </div>
      <div className="flex flex-wrap items-center gap-x-4 gap-y-1.5 px-1">
        {LEGEND.map(({ color, label }) => (
          <span key={label} className="inline-flex items-center gap-1.5 text-xs text-muted-foreground">
            <span className="size-2 rounded-full" style={{ background: color }} />
            {label}
          </span>
        ))}
      </div>
    </div>
  )
}