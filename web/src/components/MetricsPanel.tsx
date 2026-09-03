import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import type { Metrics } from '@/lib/types'

const METRICS: {
  key: keyof Metrics
  label: string
  hint: string
  format: (m: Metrics) => string
}[] = [
  {
    key: 'peak_open_deps',
    label: 'Open Dependencies',
    hint: 'comfortable bound is 4',
    format: (m) => String(m.peak_open_deps),
  },
  {
    key: 'max_dep_len',
    label: 'Dependency Length',
    hint: 'longest dependency in words',
    format: (m) => String(m.max_dep_len),
  },
  {
    key: 'embedding_depth',
    label: 'Embedding Depth',
    hint: 'clauses inside a clause',
    format: (m) => String(m.embedding_depth),
  },
  {
    key: 'right_branching',
    label: 'Right-branching',
    hint: 'share of head-initial deps',
    format: (m) => `${Math.round(m.right_branching * 100)}%`,
  },
  {
    key: 'fronted',
    label: 'Fronted',
    hint: 'tokens before the main head',
    format: (m) => String(m.fronted),
  },
]

export function MetricsPanel({ metrics }: { metrics: Metrics }) {
  return (
    <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-5">
      {METRICS.map(({ key, label, hint, format }) => (
        <Card key={key}>
          <CardHeader className="pb-1">
            <CardTitle className="text-xs font-medium text-muted-foreground">
              {label}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-semibold tabular-nums">
              {format(metrics)}
            </div>
            <p className="mt-1 text-xs text-muted-foreground">{hint}</p>
          </CardContent>
        </Card>
      ))}
    </div>
  )
}