import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Separator } from '@/components/ui/separator'
import { MetricsPanel } from '@/components/MetricsPanel'
import { ParseTreeChart } from '@/components/ParseTreeChart'
import { TokenChips } from '@/components/TokenChips'
import { KIND_META, type DiagnosisResult } from '@/lib/types'

const TONE_VARIANT: Record<string, 'default' | 'outline' | 'destructive' | 'secondary'> = {
  success: 'default',
  error: 'destructive',
  warn: 'secondary',
}

export function ResultPanel({ result }: { result: DiagnosisResult }) {
  const meta = KIND_META[result.kind]
  const clean = result.kind === 'Clean'

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-3">
        <Badge variant={TONE_VARIANT[meta.tone]} className="px-3 py-1 text-sm">
          {clean ? '✓' : '✗'} {result.kind}
        </Badge>
        <span className="text-sm text-muted-foreground">{meta.label}</span>
        {result.readings !== undefined && (
          <Badge variant="outline">{result.readings} readings</Badge>
        )}
      </div>

      {!clean && result.messages.length > 0 && (
        <Alert variant={meta.tone === 'error' ? 'destructive' : 'default'}>
          <AlertTitle>why the Linter rejects it</AlertTitle>
          <AlertDescription className="flex flex-col gap-2">
            {result.messages.map((m, i) => (
              <p key={i}>{m}</p>
            ))}
          </AlertDescription>
        </Alert>
      )}

      {clean && result.metrics && (
        <MetricsPanel metrics={result.metrics} />
      )}

      {clean && result.tree && (
        <Card>
          <CardHeader>
            <CardTitle>Parse tree</CardTitle>
          </CardHeader>
          <CardContent>
            <ParseTreeChart nodes={result.tree.nodes} />
          </CardContent>
        </Card>
      )}

      <Separator />

      <Card>
        <CardHeader>
          <CardTitle>Tokens</CardTitle>
        </CardHeader>
        <CardContent>
          <TokenChips tokens={result.tokens} />
        </CardContent>
      </Card>
    </div>
  )
}