//! Tier-2 must be a strict superset of tier-1 (every accepted sentence gets
//! ≥1 tier-2 reading), and rejected sentences must get *named* diagnoses.

use diagnose::{diagnose, Diagnosis, Tier2};
use grammar::Lexicon;

fn repo(path: &str) -> String {
    format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn tier2_is_a_superset_of_tier1() {
    let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
    let corpus = std::fs::read_to_string(repo("corpus/accept.txt")).unwrap();
    for line in corpus.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let toks: Vec<_> = lexicon
            .tokenize(line)
            .unwrap()
            .into_iter()
            .map(|(_, t)| t)
            .collect();
        let n = Tier2::new(&toks).count();
        assert!(n >= 1, "tier-2 rejects a tier-1 sentence: {line}");
    }
}

#[test]
fn rejections_get_named_diagnoses() {
    let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
    let style_cases = [
        ("if the test fails, the agent retries the request", "then"),
        ("the agent retries the request if the test fails", "condition must come first"),
        ("every agent does not retry the request", "scope-ambiguous"),
        ("no agent must check the input", "must not"),
        ("the agent retries no request", "subject-only"),
        ("some agent retries the request", "plural"),
        ("a files are big", "singular noun"),
        ("the agent has deleted the file", "perfect aspect"),
        ("the agent is opening the file", "progressive"),
        ("the file stored in the database fails", "reduced relative"),
        ("the agent stopped", "needs an object"),
        ("the agent deleted 3 file", "plural noun"),
        ("the user has one sessions", "singular noun"),
    ];
    for (sentence, expect) in style_cases {
        match diagnose(&lexicon, sentence) {
            Diagnosis::Style(findings) | Diagnosis::Ambiguous { findings, .. } => {
                assert!(
                    findings.iter().any(|f| f.contains(expect)),
                    "{sentence}: expected a finding containing {expect:?}, got {findings:?}"
                );
            }
            other => panic!("{sentence}: expected STYLE/AMBIGUOUS, got {other:?}"),
        }
    }
    // banned words go through the word-level channel with suggestions
    match diagnose(&lexicon, "it fails") {
        Diagnosis::Word(msg) => assert!(msg.contains("it")),
        other => panic!("expected Word diagnosis, got {other:?}"),
    }
    // digits: 0 and number words redirect through the word-level channel (ADR 0022)
    match diagnose(&lexicon, "the agent deleted 0 files") {
        Diagnosis::Word(msg) => assert!(msg.contains("\"no <noun>")),
        other => panic!("expected Word diagnosis, got {other:?}"),
    }
    match diagnose(&lexicon, "the agent deleted three files") {
        Diagnosis::Word(msg) => assert!(msg.contains("digits")),
        other => panic!("expected Word diagnosis, got {other:?}"),
    }
    // two verb-attaching PPs: genuinely multiple readings
    match diagnose(&lexicon, "the agent stores the report in the database with the tool") {
        Diagnosis::Ambiguous { readings, .. } => assert!(readings > 1),
        other => panic!("expected AMBIGUOUS, got {other:?}"),
    }
}
