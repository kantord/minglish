export interface Example {
  label: string
  sentence: string
  note: string
}

export const EXAMPLES: Example[] = [
  {
    label: 'plain statement',
    sentence: 'the agent reads the file',
    note: 'one Parse, small Cognitive Load',
  },
  {
    label: 'conditional',
    sentence: 'if the test fails, then the agent retries the request',
    note: 'if … , then … (ADR 0007)',
  },
  {
    label: 'prohibition',
    sentence: 'do not delete my report',
    note: 'do not <verb> (ADR 0005)',
  },
  {
    label: 'imperative',
    sentence: 'delete the file',
    note: 'a verb opens an Imperative',
  },
  {
    label: 'negation',
    sentence: 'you cannot delete the Lexicon',
    note: 'modal + not (ADR 0009)',
  },
  {
    label: 'coordination',
    sentence: 'the agent reads the file and stores the result',
    note: 'and joins 2 predicates (ADR 0004)',
  },
  {
    label: 'counted noun',
    sentence: 'the agent deleted 3 files',
    note: 'counts are digits (ADR 0022)',
  },
  {
    label: 'of-phrase',
    sentence: 'the system stores a copy of the report in the database',
    note: 'of attaches to the noun (ADR 0011)',
  },
  {
    label: 'banned pronoun',
    sentence: 'it fails',
    note: 'an Anaphoric Pronoun is a Ban (ADR 0002)',
  },
  {
    label: 'scope ambiguity',
    sentence: 'every agent does not retry the request',
    note: 'every … not is ambiguous (ADR 0014)',
  },
  {
    label: 'reduced relative',
    sentence: 'the file stored in the database fails',
    note: 'a Participle after a noun is a Ban (ADR 0010)',
  },
  {
    label: 'redirect',
    sentence: 'the agent files the report',
    note: '"files" is a noun — as a verb use "submit"',
  },
  {
    label: 'number word',
    sentence: 'three files',
    note: 'Number Words are banned (ADR 0022)',
  },
  {
    label: 'fronted because',
    sentence: 'because the test fails, the agent retries the request',
    note: '"because" cannot start a sentence (ADR 0026)',
  },
  {
    label: 'bare singular',
    sentence: 'an agent retries request',
    note: 'a singular noun needs a determiner',
  },
]