import { Badge } from '@/components/ui/badge'
import type { TokenRow } from '@/lib/types'

export function TokenChips({ tokens }: { tokens: TokenRow[] }) {
  return (
    <div className="flex flex-wrap items-center gap-1.5">
      {tokens.map((t) => (
        <span
          key={`${t.pos}-${t.word}`}
          className="inline-flex items-center gap-1.5 rounded-md border border-border bg-muted/50 px-2 py-1 text-sm"
        >
          <span className="font-medium">{t.word}</span>
          <Badge variant="outline" className="px-1 py-0 text-[10px] font-mono">
            {t.tag}
          </Badge>
        </span>
      ))}
    </div>
  )
}