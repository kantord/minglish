//! Every accept-corpus sentence parses (LR(1) guarantees uniqueness);
//! trees + metrics are snapshotted; sanctioned-structure violations reject.

use grammar::{metrics, parse, Lexicon};

fn repo(path: &str) -> String {
    format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn corpus_parses_with_snapshots() {
    let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
    let corpus = std::fs::read_to_string(repo("corpus/accept.txt")).unwrap();
    let mut snap = String::new();
    for line in corpus.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tree = parse(&lexicon, line)
            .unwrap_or_else(|e| panic!("corpus sentence failed to parse: {line} — {e}"));
        let m = metrics(&tree);
        snap.push_str(&format!(
            "### {line}\nopen={} maxdep={} depth={} right={:.0}% fronted={}\n",
            m.peak_open_deps,
            m.max_dep_len,
            m.embedding_depth,
            m.right_branching * 100.0,
            m.fronted
        ));
        tree.render(&mut snap, 0);
        snap.push('\n');
    }
    insta::assert_snapshot!("corpus_trees", snap);
}

#[test]
fn banned_structures_reject() {
    let lexicon = Lexicon::load(&repo("lexicon.tsv")).unwrap();
    let banned = [
        // determiner–number agreement
        "a files are big",
        // two verb-attaching PPs in one clause (ADR 0011 bound)
        "the agent stores the report in the database with the tool",
        // banned third-person pronoun (ADR 0002)
        "it fails",
        // banned modal (ADR 0009)
        "the agent may fail",
        // reduced relative shape cannot arise (ADR 0010's precondition)
        "the file stored in the database fails",
        // transitive verb without object
        "the agent stopped",
        // conditional without mandatory then (ADR 0007)
        "if the test fails, the agent retries the request",
        // consequent-first conditional is out (ADR 0007)
        "the agent retries the request if the test fails",
        // -ing forms have no sanctioned structure yet
        "the agent is opening the file",
        // quantified subjects take positive predicates only (ADR 0014)
        "every agent does not retry the request",
        "no agent must check the input",
        "no agent cannot open a session",
        // no is subject-only, first-token signature (ADR 0014)
        "the agent retries no request",
        // auxiliary have is unparseable by construction (ADR 0016)
        "the agent has deleted the file",
        // some: plural + subject-only (ADR 0017)
        "some agent retries the request",
        "the agent checks some requests",
        // digits: 0 and 1 redirect, leading zeros reject, number words
        // are banned, digit + singular / one + plural disagree (ADR 0022)
        "the agent deleted 0 files",
        "the agent deleted 1 file",
        "the agent deleted 03 files",
        "the agent deleted three files",
        "3 agent retries the request",
        "one files are big",
    ];
    for s in banned {
        assert!(
            parse(&lexicon, s).is_err(),
            "should NOT parse but did: {s}"
        );
    }
}
