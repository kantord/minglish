//! ADR 0049: "other" — confined to the second conjunct of a colon-list,
//! and always paired with an explicit quantifier ("every"/"some") since
//! bare "other" is ambiguous once a category has 3+ members. Grammar-only
//! tests (position + agreement); the verification semantic check lives
//! in crates/diagnose (see diagnosis.rs).

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

fn rejects(s: &str) {
    let lex = lexicon();
    assert!(parse_text(&lex, s).is_err(), "expected a rejection: {s}");
}

#[test]
fn every_other_takes_singular() {
    ok("the mechanism names: the Ban and every other Rejection.");
}

#[test]
fn some_other_takes_plural() {
    ok("the mechanism names: the Ban and some other Rejections.");
}

#[test]
fn bare_other_is_ambiguous_and_banned() {
    rejects("the mechanism names: the Ban and other Rejections.");
}

#[test]
fn every_other_rejects_plural() {
    rejects("the mechanism names: the Ban and every other Rejections.");
}

#[test]
fn some_other_rejects_singular() {
    rejects("the mechanism names: the Ban and some other Rejection.");
}

#[test]
fn not_the_first_conjunct() {
    rejects("the mechanism names: every other Rejection and the Ban.");
}

#[test]
fn not_a_subject() {
    rejects("every other Rejection exists.");
}

#[test]
fn not_a_plain_object() {
    rejects("the mechanism deletes every other Rejection.");
}
