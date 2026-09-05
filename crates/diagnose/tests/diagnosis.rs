//! Tier-2 must be a strict superset of tier-1 (every accepted sentence gets
//! ≥1 tier-2 reading), and rejected sentences must get *named* diagnoses.

use diagnose::{diagnose, Diagnosis, Tier2};
use grammar::Lexicon;

fn repo(path: &str) -> String {
    format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"))
}

// diagnose() runs on arbitrary LLM-repair-proposal text (agenttest) and
// arbitrary document prose (lint-file.py), not just well-formed minglish —
// crash-freedom on any input is a real requirement.
proptest::proptest! {
    #[test]
    fn diagnose_never_panics(s in "\\PC{0,200}") {
        let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
        let _ = diagnose(&lexicon, &s);
    }
}

#[test]
fn tier2_is_a_superset_of_tier1() {
    let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
    let corpus = std::fs::read_to_string(repo("corpus/accept.txt")).unwrap();
    for line in grammar::units(&corpus) {
        if grammar::is_enumeration(&line) || grammar::is_step_block(&line) {
            continue; // blocks are checked by the tier-1 corpus test
        }
        let line = line.as_str();
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
        ("the test fails so the agent retries the request", "comma before \"so\""),
        ("because the test fails, the agent retries the request", "cannot start a sentence"),
        ("so the agent retries the request", "cannot start a sentence"),
        ("the project does not plan a discourse mechanism", "noun-noun"),
        ("the word i is a Pronoun", "must be quoted"),
        ("the Pronouns are not in the Lexicon", "not a prepositional phrase"),
        ("the frequencies are of Pronouns", "not a prepositional phrase"),
        ("the design is consistent with the project", "cannot take a prepositional phrase"),
        ("the mechanism stores a word and a message", "cannot be coordinated"),
        ("the speaker or the hearer reads the file", "cannot be coordinated"),
        ("the cost is big", "as a noun use"),
        ("the agent files the report", "as a verb use"),
        ("the Lexicon does not contain the banned Pronouns", "cannot modify a noun"),
        ("Pronouns for the speaker are indexical", "attaches to the verb"),
        ("the design prefers clarity", "needs a determiner"),
        ("the report shows Pronouns are big", "cannot be the object"),
        ("the i Pronoun is indexical", "follows the noun in quotes"),
        ("the file is more heavy than the report", "write \"heavier\""),
        ("the file is heavier", "needs the standard"),
        ("the Triage checks the tokens", "takes no determiner"),
        ("the file be old", "exists only after a modal"),
        ("Then the queue is empty", "inside a Step Block"),
        ("every decision about a structure uses 3 criteria", "exists only before digits"),
        ("the rule keeps the loss small", "carries the result"),
        ("the bound is 4", "needs its noun"),
        ("the bound of the Open Dependencies is about 4", "needs its noun"),
        ("every Grammar ADR uses the 3 criteria", "only after"),
        ("the writer prefers the more expressive one", "not a Pronoun"),
        ("the criterion of the Cognitive Load is first", "needs its noun"),
        ("the text must be cheap to the process", "cannot take a prepositional phrase"),
        ("if the test fails and the agent retries the request, then the queue is empty", "comma before"),
        ("the Grammar gives every sentence one Parse", "one object"),
        ("the rule keeps the Cognitive Load of the sentence small", "carries the result"),
        ("the Pronouns of a person not the speaker are anaphoric", "negates the verb only"),
        ("the prose is repetitive, the agent mentions the agent", "comma cannot join"),
        ("the language does not have a discourse layer", "write \"Discourse Layer\""),
        ("the Linter bans 3 Pronouns:\n- \"it\"\n- \"they\"", "counts 3 but"),
        ("the Linter bans the Pronoun:\n- \"it\"", "must be plural"),
        ("the Pronouns are \"it\", \"they\", and \"those\"", "inline list"),
        ("instructional text uses Indexical Pronouns", "needs a determiner"),
        ("the team defers the mechanism to the future", "is an adjective"),
        ("the Pronouns are about 2200 of the unknown tokens", "count needs its noun"),
        ("the document describes the finding", "verb form in minglish"),
        ("resolving the Pronoun requires a Discourse Layer", "cannot be the subject"),
        ("the language bans the possessive of every Anaphoric Pronoun", "of every"),
        ("Triage measures the Coverage against the UD-EWT corpora", "follows its noun"),
        ("the tool Lexgen allows my", "used as a word must be quoted"),
        ("the Pronouns of the speaker or of the hearer are indexical", "phrases cannot be coordinated"),
        ("the point of the copy of the report fails", "does not chain"),
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
    // same: banned, advice names the per-sense synonym (ADR 0023)
    match diagnose(&lexicon, "the copies are the same") {
        Diagnosis::Word(msg) => assert!(msg.contains("identical")),
        other => panic!("expected Word diagnosis, got {other:?}"),
    }
    // two verb-attaching PPs: genuinely multiple readings
    match diagnose(&lexicon, "the agent stores the report in the database with the tool") {
        Diagnosis::Ambiguous { readings, .. } => assert!(readings > 1),
        other => panic!("expected AMBIGUOUS, got {other:?}"),
    }
}

/// Advice gap #2 (docs/STATUS.md): the "as a verb use X" redirect was only
/// wired for the plain-SVO position (right after the subject). A modal,
/// negation, or sentence-initial imperative left the same word to fall
/// through to generic, sometimes actively wrong findings ("must be
/// quoted", "needs a determiner") instead of the correct redirect. Fixed
/// by widening the trigger and suppressing the now-superseded generic
/// findings for the same word.
#[test]
fn verb_redirect_fires_in_every_bare_verb_position() {
    let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
    let cases = [
        "the agent files the report",
        "file the report",
        "do not file the report",
        "the agent can file the report",
        "the agent must not file the report",
    ];
    for sentence in cases {
        match diagnose(&lexicon, sentence) {
            Diagnosis::Style(findings) => {
                assert!(
                    findings.iter().any(|f| f.contains("as a verb use \"submit\"")),
                    "{sentence}: expected the submit redirect, got {findings:?}"
                );
                assert!(
                    !findings.iter().any(|f| f.contains("must be quoted") || f.contains("needs a determiner")),
                    "{sentence}: a superseded generic finding survived: {findings:?}"
                );
            }
            other => panic!("{sentence}: expected STYLE, got {other:?}"),
        }
    }
}

/// ADR 0048 (grilled design): minglish converges to one canonical
/// construction per meaning by default; coexistence needs empirical
/// evidence, and none exists for same-verb-lemma coordination — this
/// session's own earlier analysis already found it reads worse than the
/// colon-list form. Banned via a semantic check on an already-successful
/// Tier-1 parse (a CFG can't compare two terminals' string payloads
/// against each other), narrowly scoped: different-verb coordination,
/// negated/modal predicates, and the colon-list shape itself all stay
/// untouched.
#[test]
fn same_verb_coordination_is_banned_but_only_the_plain_shape() {
    let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
    match diagnose(&lexicon, "the mechanism stores a word and stores a message") {
        Diagnosis::Style(findings) => {
            assert!(findings.iter().any(|f| f.contains("repeats across the coordination")), "{findings:?}");
        }
        other => panic!("expected STYLE, got {other:?}"),
    }
    // nested inside a Conditional's Clause, not just top-level S
    match diagnose(&lexicon, "if the agent saves the report and saves the file, then the writer reads the report") {
        Diagnosis::Style(findings) => {
            assert!(findings.iter().any(|f| f.contains("repeats across the coordination")), "{findings:?}");
        }
        other => panic!("expected STYLE, got {other:?}"),
    }
    let still_clean = [
        "the agent saves the report and deletes the file",
        "the agent does not save the report and does not save the file",
        "the mechanism stores: a word and a message",
    ];
    for sentence in still_clean {
        assert!(
            matches!(diagnose(&lexicon, sentence), Diagnosis::Clean(_)),
            "{sentence}: expected Clean, got {:?}",
            diagnose(&lexicon, sentence)
        );
    }
}
