//! minglish linter compiled to wasm32-unknown-unknown for the static web
//! playground (see `web/`). The lexicon is embedded at compile time, so no
//! filesystem access is needed at runtime.

use diagnose::{diagnose, Diagnosis};
use grammar::{parse_text, Lexicon, Metrics, Tok, Tree};
use serde_json::{json, Value};
use std::sync::LazyLock;
use wasm_bindgen::prelude::*;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    Lexicon::from_tsv(include_str!("../../../lexicon.tsv")).expect("embedded lexicon.tsv")
});

/// Lint one sentence (or Enumeration / Step Block) and return a JSON
/// document:
///
/// ```json
/// {
///   "kind": "Clean" | "Word" | "Style" | "Ambiguous" | "Unknown",
///   "messages": ["..."],
///   "readings": 3,
///   "metrics": { ... },
///   "tree": { "nodes": [...] },
///   "tokens": [{ "pos": 0, "word": "the", "tag": "DET" }]
/// }
/// ```
#[wasm_bindgen]
pub fn diagnose_sentence(sentence: &str) -> String {
    let tokens = tokenize_json(sentence);
    let payload = match diagnose(&LEXICON, sentence) {
        Diagnosis::Clean(metrics) => {
            let tree = parse_text(&LEXICON, sentence).ok();
            json!({
                "kind": "Clean",
                "metrics": metrics_json(metrics),
                "tree": tree.map(|t| tree_json(&t)),
            })
        }
        Diagnosis::Word(msg) => json!({ "kind": "Word", "messages": vec![msg] }),
        Diagnosis::Style(findings) => json!({ "kind": "Style", "messages": findings }),
        Diagnosis::Ambiguous { readings, findings } => json!({
            "kind": "Ambiguous",
            "readings": readings,
            "messages": findings,
        }),
        Diagnosis::Unknown => json!({ "kind": "Unknown", "messages": [] }),
    };
    let mut out = payload.as_object().cloned().unwrap_or_default();
    out.insert("tokens".to_string(), tokens);
    serde_json::to_string(&Value::Object(out)).expect("json")
}

/// Split a sentence into tokens and return them as JSON:
/// `[{ "pos": 0, "word": "the", "tag": "DET" }, ...]`。
 /// A token that the lexicon does not know is returned with the tag `"?"`。
#[wasm_bindgen]
pub fn tokenize(sentence: &str) -> String {
    serde_json::to_string(&tokenize_json(sentence)).expect("json")
}

fn tokenize_json(sentence: &str) -> Value {
    let toks = LEXICON.tokenize(sentence);
    match toks {
        Ok(toks) => {
            let arr: Vec<Value> = toks
                .into_iter()
                .map(|(pos, t)| json!({ "pos": pos, "word": word_of(&t), "tag": tag_of(&t) }))
                .collect();
            Value::Array(arr)
        }
        Err(e) => Value::Array(vec![json!({ "pos": e.position, "word": e.word, "tag": "?" })]),
    }
}

fn word_of(t: &Tok) -> &str {
    use Tok::*;
    match t {
        Det(w) | DetSg(w) | Adj(w) | NounSg(w) | NounPl(w) | VtBase(w) | Vt3(w) | VtEd(w)
        | VtIng(w) | ViBase(w) | Vi3(w) | ViEd(w) | ViIng(w) | PrepN(w) | PrepV(w)
        | Pron1(w) | Pron2(w) | Poss(w) | CopSg(w) | CopPl(w) | CopSgPast(w)
        | CopPlPast(w) | Conj(w) | Neg(w) | DoBase(w) | Do3(w) | DoPast(w)
        | ModalMust(w) | ModalCan(w) | ModalCannot(w) | If(w) | Then(w) | Every(w)
        | No(w) | Num(w) | NumPl(w) | Percent(w) | Approx(w) | So(w) | Because(w)
        | Ord(w) | Than(w) | More(w) | Scale(w) | AdjCmp(w) | AdjLong(w) | Be(w)
        | BecomeSg(w) | BecomePl(w) | BecomePast(w) | Some_(w) | Name(w) => w,
        Comma => ",",
        Colon => ":",
    }
}

/// The Form Tag of a token, e.g. `"NOUN_SG"` (matches the Lexicon).
fn tag_of(t: &Tok) -> &'static str {
    use Tok::*;
    match t {
        Det(_) => "DET",
        DetSg(_) => "DET_SG",
        Adj(_) => "ADJ",
        NounSg(_) => "NOUN_SG",
        NounPl(_) => "NOUN_PL",
        VtBase(_) => "VERB_TRANS_BASE",
        Vt3(_) => "VERB_TRANS_3SG",
        VtEd(_) => "VERB_TRANS_ED",
        VtIng(_) => "VERB_TRANS_ING",
        ViBase(_) => "VERB_INTRANS_BASE",
        Vi3(_) => "VERB_INTRANS_3SG",
        ViEd(_) => "VERB_INTRANS_ED",
        ViIng(_) => "VERB_INTRANS_ING",
        PrepN(_) => "PREP_N",
        PrepV(_) => "PREP_V",
        Pron1(_) => "PRON_1",
        Pron2(_) => "PRON_2",
        Poss(_) => "POSS",
        CopSg(_) => "COPULA_SG",
        CopPl(_) => "COPULA_PL",
        CopSgPast(_) => "COPULA_SG_PAST",
        CopPlPast(_) => "COPULA_PL_PAST",
        Conj(_) => "CONJ",
        Neg(_) => "NEG",
        DoBase(_) => "NEG_AUX_BASE",
        Do3(_) => "NEG_AUX_3SG",
        DoPast(_) => "NEG_AUX_PAST",
        ModalMust(_) => "MODAL_MUST",
        ModalCan(_) => "MODAL_CAN",
        ModalCannot(_) => "MODAL_CAN_NEG",
        If(_) => "SCONJ_COND",
        Then(_) => "THEN",
        Every(_) => "QUANT_UNIV",
        No(_) => "QUANT_NEG",
        Num(_) => "NUM_SG",
        NumPl(_) => "NUM_PL",
        Percent(_) => "PERCENT",
        Approx(_) => "APPROX",
        So(_) => "RESULT",
        Because(_) => "REASON",
        Ord(_) => "ORD",
        Than(_) => "THAN",
        More(_) => "MORE",
        Scale(_) => "SCALE",
        AdjCmp(_) => "ADJ_CMP",
        AdjLong(_) => "ADJ_LONG",
        Be(_) => "BE",
        BecomeSg(_) => "BECOME_SG",
        BecomePl(_) => "BECOME_PL",
        BecomePast(_) => "BECOME_PAST",
        Some_(_) => "QUANT_EXIST",
        Name(_) => "NAME",
        Comma => "COMMA",
        Colon => "COLON",
    }
}

fn metrics_json(m: Metrics) -> Value {
    json!({
        "peak_open_deps": m.peak_open_deps,
        "max_dep_len": m.max_dep_len,
        "embedding_depth": m.embedding_depth,
        "right_branching": m.right_branching,
        "fronted": m.fronted,
    })
}

/// Flatten a parse tree into rows for the tree chart:
/// `{ "nodes": [{ "id", "parentId", "name", "kind", "head" }] }`。
 /// `kind` is `"node"` for a phrase/sentence and `"word"` for a leaf; `head`
/// marks the child that is its parent's syntactic head.
fn tree_json(tree: &Tree) -> Value {
    let mut nodes: Vec<Value> = Vec::new();
    fn walk(tree: &Tree, parent: Option<&str>, head: bool, nodes: &mut Vec<Value>) -> String {
        let id = format!("n{}", nodes.len());
        match tree {
            Tree::Leaf { word, .. } => {
                nodes.push(json!({
                    "id": id, "parentId": parent, "name": word, "kind": "word", "head": head,
                }));
            }
            Tree::Node { label, head: h, children } => {
                nodes.push(json!({
                    "id": id, "parentId": parent, "name": label, "kind": "node", "head": head,
                }));
                for (i, c) in children.iter().enumerate() {
                    walk(c, Some(&id), i == *h, nodes);
                }
            }
        }
        id
    }
    walk(tree, None, false, &mut nodes);
    json!({ "nodes": nodes })
}