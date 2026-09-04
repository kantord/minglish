//! ADR 0044: "still"/"already" as a medial adverb, one optional slot right
//! before the predicate — transparent when absent (same tree shape as
//! before 0044), wrapping in a "PredAdv" node only when used.

use grammar::{parse_text, Lexicon};

fn repo(path: &str) -> String {
    format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"))
}

fn lexicon() -> Lexicon {
    Lexicon::load(&repo("lexicon.tsv")).unwrap()
}

fn ok(s: &str) {
    let lex = lexicon();
    parse_text(&lex, s).unwrap_or_else(|e| panic!("expected to parse: {s} — {e}"));
}

#[test]
fn before_a_plain_verb() {
    ok("the agent already saves the report.");
}

#[test]
fn before_do_support() {
    ok("the agent still does not save the report.");
}

#[test]
fn before_a_modal() {
    ok("the agent still must save the report.");
}

#[test]
fn before_a_copula() {
    ok("the file already is old.");
}

#[test]
fn first_person_and_plural_subjects() {
    ok("I still save the report.");
    ok("the agents still save the report.");
}

#[test]
fn enumeration_intro_still_finds_its_noun_phrase() {
    ok("the agent already saves 3 reports:\n- \"a\"\n- \"b\"\n- \"c\"");
}
