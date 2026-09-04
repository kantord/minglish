//! ADR 0045: "afterward" — the bare, no-anchor twin of "after <NP>",
//! sharing the PPv slot (ADR 0011/0033) rather than a new position.

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
fn bare_clause_final() {
    ok("the agent saves the report afterward.");
}

#[test]
fn combines_with_negation() {
    ok("the agent does not save the report afterward.");
}

#[test]
fn not_sentence_initial() {
    // PPv is a trailing slot, same as every other PPv word — no fronting.
    rejects("afterward the agent saves the report.");
}

#[test]
fn one_ppv_per_clause_still_holds() {
    // ADR 0011/0033's existing limit: "afterward" competes for the same
    // single trailing slot as any other PPv word.
    rejects("the agent saves the report with the tool afterward.");
}
