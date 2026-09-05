//! End-to-end: tokenize a real (Tier-1-rejected) sentence with the real
//! lexicon, scan with every antiparser, check the right one fires with the
//! right repair category.

use antiparse::{anti::Repair, format_finding, rank_by_failure_position, scan, AntiFinding};
use grammar::Lexicon;

fn repo(path: &str) -> String {
    format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"))
}

fn lexicon() -> Lexicon {
    Lexicon::load(&repo("lexicon.tsv")).unwrap()
}

fn toks(lex: &Lexicon, s: &str) -> Vec<grammar::Tok> {
    lex.tokenize(s).unwrap().into_iter().map(|(_, t)| t).collect()
}

#[test]
fn bare_coord_full_conjuncts_gets_a_single_repair() {
    let lex = lexicon();
    let t = toks(&lex, "the mechanism stores the word and the message");
    let findings = scan(&lex, &t);
    let hit = findings.iter().find(|f| f.name == "AntiBareCoordObject").expect("should fire");
    match &hit.repair {
        Repair::Single(fix) => assert!(fix.contains(':'), "expected the colon-list repair, got {fix}"),
        other => panic!("expected Single, got {other:?}"),
    }
}

#[test]
fn bare_coord_elliptical_conjunct_gets_no_repair() {
    let lex = lexicon();
    // "the old files and reports" shape — second conjunct has no determiner
    let t = toks(&lex, "the mechanism stores the file and reports");
    let findings = scan(&lex, &t);
    let hit = findings.iter().find(|f| f.name == "AntiBareCoordObject").expect("should fire");
    match &hit.repair {
        Repair::None(why) => assert!(why.contains("not recoverable")),
        other => panic!("expected None, got {other:?}"),
    }
}

#[test]
fn noun_verb_after_negation_gets_the_redirect() {
    let lex = lexicon();
    let t = toks(&lex, "the agent must not file the report");
    let findings = scan(&lex, &t);
    let hit = findings.iter().find(|f| f.name == "AntiNounVerbSlot").expect("should fire");
    match &hit.repair {
        Repair::Single(fix) => assert!(fix.contains("submit"), "expected submit, got {fix}"),
        other => panic!("expected Single, got {other:?}"),
    }
}

#[test]
fn free_only_gets_a_menu_not_a_guess() {
    let lex = lexicon();
    let t = toks(&lex, "the mechanism only stores the report");
    let findings = scan(&lex, &t);
    let hit = findings.iter().find(|f| f.name == "AntiFreeOnly").expect("should fire");
    match &hit.repair {
        Repair::Menu(options) => assert_eq!(options.len(), 2),
        other => panic!("expected Menu, got {other:?}"),
    }
}

#[test]
fn ranking_prefers_the_span_touching_the_failure_position() {
    let lex = lexicon();
    // 2 candidate antiparser spans could plausibly exist in a longer
    // sentence; the failure position should pick the right one.
    let t = toks(&lex, "the agent must not file the report");
    let mut findings: Vec<AntiFinding> = scan(&lex, &t);
    // "file" sits at index 4 (the, agent, must, not, file, the, report)
    let failure_pos = 4;
    rank_by_failure_position(&mut findings, failure_pos);
    let top = &findings[0];
    assert!(
        top.span.0 <= failure_pos && failure_pos < top.span.1,
        "top-ranked span {:?} should cover the failure position {failure_pos}",
        top.span
    );
    println!("{}", format_finding(top));
}
