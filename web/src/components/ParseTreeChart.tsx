import { useMemo } from 'react'
import { defineChart, dot, link, text } from '@tanstack/charts'
import { treeLayout } from '@tanstack/charts/hierarchy/tree'
import { scaleLinear } from '@tanstack/charts/scales/linear'
import { Chart } from '@tanstack/charts/react'
import type { TreeNode } from '@/lib/types'

export function ParseTreeChart({ nodes }: { nodes: TreeNode[] }) {
  const definition = useMemo(() => {
    const hierarchy = treeLayout(nodes, { id: 'id', parentId: 'parentId' })
    const internal = hierarchy.nodes.filter((n) => n.data?.kind !== 'word')
    const words = hierarchy.nodes.filter((n) => n.data?.kind === 'word')
    const label = (n: { data: { name?: string } | null; name: string }) =>
      n.data?.name ?? n.name
    return defineChart({
      marks: [
        link(hierarchy.links, {
          x1: 'x1',
          y1: 'y1',
          x2: 'x2',
          y2: 'y2',
          key: 'id',
          stroke: 'var(--border)',
          strokeWidth: 1.5,
        }),
        dot(internal, {
          x: 'x',
          y: 'y',
          key: 'id',
          fill: 'var(--muted-foreground)',
          r: 2.5,
        }),
        dot(words, {
          x: 'x',
          y: 'y',
          key: 'id',
          fill: 'var(--primary)',
          r: 3.5,
        }),
        text(internal, {
          x: 'x',
          y: 'y',
          key: 'id',
          text: label,
          fill: 'var(--muted-foreground)',
          fontSize: 11,
          fontWeight: 600,
          anchor: 'end',
          dx: -7,
        }),
        text(words, {
          x: 'x',
          y: 'y',
          key: 'id',
          text: label,
          fill: 'var(--foreground)',
          fontSize: 13,
          fontWeight: 500,
          anchor: 'start',
          dx: 7,
        }),
      ],
      scales: {
        x: { scale: scaleLinear },
        y: { scale: scaleLinear },
      },
      guides: false,
      margin: { top: 20, right: 140, bottom: 20, left: 48 },
    })
  }, [nodes])

  const leafCount = useMemo(
    () => nodes.filter((n) => n.kind === 'word').length,
    [nodes],
  )

  return (
    <Chart
      definition={definition}
      height={Math.max(320, leafCount * 30 + 60)}
      ariaLabel="Parse tree of the sentence"
    />
  )
}