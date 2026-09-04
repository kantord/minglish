//! ADR 0046: "yet" is a negative-polarity item — spliced only into
//! productions where a NEG is mandatory in the same alternative, so the
//! grammar itself makes an unnegated "yet" underivable (not a linter rule).

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
fn with_do_support() {
    ok("the agent does not yet save the report.");
}

#[test]
fn with_copula() {
    ok("the queue is not yet empty.");
}

#[test]
fn with_prohibition() {
    ok("do not yet delete the file.");
}

#[test]
fn with_modal() {
    ok("the agent must not yet save the report.");
}

#[test]
fn with_modal_copula() {
    ok("the agent must not yet be old.");
}

#[test]
fn bare_yet_is_impossible() {
    rejects("the agent yet saves the report.");
    rejects("the queue yet is empty.");
}

#[test]
fn yet_needs_do_support_like_any_negation() {
    // "the agent does yet save" — negation dropped but "yet" kept: still
    // not derivable, same reason bare "the agent not saves" already isn't.
    rejects("the agent does yet save the report.");
}
