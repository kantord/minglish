//! Tier-1 minglish grammar: LALRPOP over form-tags, lexicon-backed lexer,
//! and cognitive-load metrics derived from head-annotated parse trees.
//!
//! The LR(1) build is the ambiguity gate: a grammar conflict is a compile
//! error, so every accepted sentence has exactly one parse by construction.
//! Metrics follow ADR 0006: report peak open dependencies, dependency
//! length, embedding depth, branching direction, and fronted weight —
//! measurement only, no gating yet.

use lalrpop_util::lalrpop_mod;
use std::collections::BTreeMap;

lalrpop_mod!(pub minglish);

// ---------------------------------------------------------------- tokens --

#[derive(Clone, Debug)]
pub enum Tok {
    Det(String),
    DetSg(String),
    Adj(String),
    NounSg(String),
    NounPl(String),
    VtBase(String),
    Vt3(String),
    VtEd(String),
    VtIng(String),
    ViBase(String),
    Vi3(String),
    ViEd(String),
    ViIng(String),
    PrepN(String),
    PrepV(String),
    Pron1(String),
    Pron2(String),
    Poss(String),
    CopSg(String),
    CopPl(String),
    CopSgPast(String),
    CopPlPast(String),
    Conj(String),
    Neg(String),
    DoBase(String),
    Do3(String),
    DoPast(String),
    ModalMust(String),
    ModalCan(String),
    ModalCannot(String),
    If(String),
    Then(String),
    Every(String),
    No(String),
    Num(String),
    NumPl(String),
    Some_(String),
    Name(String),
    Comma,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub word: String,
    pub position: usize,
    /// From the lexicon's reject table, when the unknown word is a known
    /// rejected use: "use \"X\" instead".
    pub suggestion: Option<String>,
    /// The word is deliberately banned (ban table), not merely unknown.
    pub banned: bool,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.banned {
            write!(f, "\"{}\" is banned in minglish", self.word)?;
        } else {
            write!(f, "\"{}\" is not a minglish word", self.word)?;
        }
        if let Some(s) = &self.suggestion {
            write!(f, " — {s}")?;
        }
        Ok(())
    }
}

// --------------------------------------------------------------- lexicon --

pub struct Lexicon {
    forms: BTreeMap<String, String>,
    rejects: BTreeMap<String, Vec<(String, String)>>,
    bans: BTreeMap<String, String>,
}

impl Lexicon {
    pub fn load(path: &str) -> Result<Lexicon, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let mut forms = BTreeMap::new();
        let mut rejects: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
        let mut bans: BTreeMap<String, String> = BTreeMap::new();
        for line in text.lines().filter(|l| !l.starts_with('#')) {
            let f: Vec<&str> = line.split('\t').collect();
            let [surface, kind, tag, value] = f[..] else { continue };
            match kind {
                "form" => {
                    forms.insert(surface.to_string(), tag.to_string());
                }
                "reject" => rejects
                    .entry(surface.to_string())
                    .or_default()
                    .push((tag.to_string(), value.to_string())),
                "ban" => {
                    bans.insert(surface.to_string(), value.to_string());
                }
                _ => {}
            }
        }
        Ok(Lexicon { forms, rejects, bans })
    }

    /// Tokenize a sentence. Commas become COMMA tokens; a trailing period is
    /// dropped; a double-quoted span is a NAME (verbatim identifier);
    /// capitalized words are NAMEs unless they fold to a lexicon word at
    /// sentence start (ADR 0018); anything else not in the lexicon is a
    /// LexError with a redirect suggestion when the reject table has one.
    pub fn tokenize(&self, sentence: &str) -> Result<Vec<(usize, Tok)>, LexError> {
        let mut out: Vec<(usize, Tok)> = Vec::new();
        // quoted spans become single NAME tokens; even-indexed pieces are
        // ordinary text
        let pieces: Vec<&str> = sentence.split('"').collect();
        for (pi, piece) in pieces.iter().enumerate() {
            if pi % 2 == 1 {
                let pos = out.len();
                out.push((pos, Tok::Name(piece.trim().to_string())));
                continue;
            }
            self.tokenize_plain(piece, &mut out)?;
        }
        // merge consecutive capitalized NAMEs ("Visual Studio Code")
        let mut merged: Vec<(usize, Tok)> = Vec::new();
        for (_, t) in out {
            if let (Some((_, Tok::Name(prev))), Tok::Name(w)) = (merged.last_mut(), &t) {
                prev.push(' ');
                prev.push_str(w);
                continue;
            }
            let pos = merged.len();
            merged.push((pos, t));
        }
        Ok(merged.into_iter().enumerate().map(|(i, (_, t))| (i, t)).collect())
    }

    fn tokenize_plain(
        &self,
        piece: &str,
        out: &mut Vec<(usize, Tok)>,
    ) -> Result<(), LexError> {
        for raw in piece.split_whitespace() {
            let mut word = raw.trim_end_matches('.');
            let mut had_comma = false;
            if let Some(stripped) = word.strip_suffix(',') {
                word = stripped;
                had_comma = true;
            }
            if !word.is_empty() {
                let pos = out.len();
                // ADR 0022: digits are a lexer class, not lexicon words
                if let Some(numeral) = number_token(word) {
                    let tok = numeral.map_err(|suggestion| LexError {
                        word: word.to_string(),
                        position: pos,
                        suggestion: Some(suggestion),
                        banned: false,
                    })?;
                    out.push((pos, tok));
                    if had_comma {
                        let pos = out.len();
                        out.push((pos, Tok::Comma));
                    }
                    continue;
                }
                let capitalized = word.chars().next().is_some_and(|c| c.is_uppercase());
                // ADR 0018, fail-loud: unquoted NAME only when capitalized,
                // mid-sentence, and not a lexicon word in lowercase. A
                // sentence-initial capital folds to the lexicon or errors —
                // never silently becomes a name (typos must stay loud).
                let folded;
                let word = if !self.forms.contains_key(word) {
                    folded = word.to_lowercase();
                    let known_lower = self.forms.contains_key(&folded);
                    if known_lower && (pos == 0 || word == "I") {
                        folded.as_str()
                    } else if capitalized && known_lower {
                        return Err(LexError {
                            word: word.to_string(),
                            position: pos,
                            suggestion: Some(format!(
                                "minglish words are lowercase (\"{folded}\"); a name \
                                 that equals a word needs quotes"
                            )),
                            banned: false,
                        });
                    } else if capitalized && pos > 0 {
                        out.push((pos, Tok::Name(word.to_string())));
                        continue;
                    } else if capitalized {
                        return Err(LexError {
                            word: word.to_string(),
                            position: pos,
                            suggestion: Some(
                                "a name cannot start a sentence — introduce it \
                                 (\"the tool Lexgen …\") or quote it; or if this \
                                 is a command, use a minglish verb in lowercase \
                                 (\"delete the file\")"
                                    .to_string(),
                            ),
                            banned: false,
                        });
                    } else {
                        word
                    }
                } else {
                    word
                };
                if let Some(advice) = self.bans.get(word) {
                    return Err(LexError {
                        word: word.to_string(),
                        position: pos,
                        suggestion: Some(advice.clone()),
                        banned: true,
                    });
                }
                let tag = self.forms.get(word).ok_or_else(|| LexError {
                    word: word.to_string(),
                    position: pos,
                    suggestion: self.rejects.get(word).map(|rs| {
                        rs.iter()
                            .map(|(pos, sugg)| format!("as a {pos} use \"{sugg}\""))
                            .collect::<Vec<_>>()
                            .join("; ")
                    }),
                    banned: false,
                })?;
                let tok = tag_to_tok(tag, word).ok_or_else(|| LexError {
                    word: word.to_string(),
                    position: pos,
                    suggestion: Some(format!("its category {tag} is not yet usable in any sentence structure")),
                    banned: false,
                })?;
                out.push((pos, tok));
            }
            if had_comma {
                let pos = out.len();
                out.push((pos, Tok::Comma));
            }
        }
        Ok(())
    }
}

/// Digit strings (ADR 0022): `2` and up are NUM_PL; `0` and `1` are
/// redirected (*no* / *one*), leading zeros are rejected. Non-digit words
/// return None.
fn number_token(word: &str) -> Option<Result<Tok, String>> {
    if word.is_empty() || !word.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(match word {
        "0" => Err("for none write \"no <noun> …\" as the subject, or \
                    \"… does not <verb> <nouns>\" (ADR 0022)"
            .to_string()),
        "1" => Err("write \"one\" — exactly one stays a word (ADR 0016)".to_string()),
        _ if word.starts_with('0') => {
            Err("a number does not start with 0 (ADR 0022)".to_string())
        }
        _ => Ok(Tok::NumPl(word.to_string())),
    })
}

fn tag_to_tok(tag: &str, word: &str) -> Option<Tok> {
    let w = word.to_string();
    Some(match tag {
        "DET" => Tok::Det(w),
        "DET_SG" => Tok::DetSg(w),
        "ADJ" => Tok::Adj(w),
        "NOUN_SG" => Tok::NounSg(w),
        "NOUN_PL" => Tok::NounPl(w),
        "VERB_TRANS_BASE" => Tok::VtBase(w),
        "VERB_TRANS_3SG" => Tok::Vt3(w),
        "VERB_TRANS_ED" => Tok::VtEd(w),
        "VERB_TRANS_ING" => Tok::VtIng(w),
        "VERB_INTRANS_BASE" => Tok::ViBase(w),
        "VERB_INTRANS_3SG" => Tok::Vi3(w),
        "VERB_INTRANS_ED" => Tok::ViEd(w),
        "VERB_INTRANS_ING" => Tok::ViIng(w),
        "PREP_N" => Tok::PrepN(w),
        "PREP_V" => Tok::PrepV(w),
        "PRON_1SG" => Tok::Pron1(w),
        "PRON_2" => Tok::Pron2(w),
        "POSS_1SG" | "POSS_2" => Tok::Poss(w),
        "COPULA_SG" => Tok::CopSg(w),
        "COPULA_PL" => Tok::CopPl(w),
        "COPULA_SG_PAST" => Tok::CopSgPast(w),
        "COPULA_PL_PAST" => Tok::CopPlPast(w),
        "CONJ" => Tok::Conj(w),
        "NEG" => Tok::Neg(w),
        "NEG_AUX_BASE" => Tok::DoBase(w),
        "NEG_AUX_3SG" => Tok::Do3(w),
        "NEG_AUX_PAST" => Tok::DoPast(w),
        "MODAL_MUST" => Tok::ModalMust(w),
        "MODAL_CAN" => Tok::ModalCan(w),
        "MODAL_CAN_NEG" => Tok::ModalCannot(w),
        "SCONJ_COND" => Tok::If(w),
        "THEN" => Tok::Then(w),
        "QUANT_UNIV" => Tok::Every(w),
        "QUANT_NEG" => Tok::No(w),
        "NUM_SG" => Tok::Num(w),
        "QUANT_EXIST" => Tok::Some_(w),
        // NAME is produced directly by the tokenizer, never from the lexicon
        _ => return None,
    })
}

// ------------------------------------------------------------------ tree --

#[derive(Clone, Debug)]
pub enum Tree {
    Leaf { pos: usize, word: String },
    Node { label: &'static str, head: usize, children: Vec<Tree> },
}

pub fn leaf(pos: usize, word: String) -> Tree {
    Tree::Leaf { pos, word }
}

/// Build a node from fixed slots, dropping absent optionals. `head_slot`
/// indexes the *slot* list; it is re-based onto the surviving children.
pub fn nd(label: &'static str, head_slot: usize, slots: Vec<Option<Tree>>) -> Tree {
    let head = slots[..head_slot].iter().filter(|s| s.is_some()).count();
    let children: Vec<Tree> = slots.into_iter().flatten().collect();
    assert!(head < children.len(), "head slot must be present");
    Tree::Node { label, head, children }
}

impl Tree {
    pub fn head_word_pos(&self) -> usize {
        match self {
            Tree::Leaf { pos, .. } => *pos,
            Tree::Node { head, children, .. } => children[*head].head_word_pos(),
        }
    }

    pub fn render(&self, out: &mut String, indent: usize) {
        match self {
            Tree::Leaf { word, .. } => {
                out.push_str(&format!("{}{}\n", "  ".repeat(indent), word))
            }
            Tree::Node { label, head, children, .. } => {
                out.push_str(&format!("{}({label} h={head}\n", "  ".repeat(indent)));
                for c in children {
                    c.render(out, indent + 1);
                }
                out.push_str(&format!("{})\n", "  ".repeat(indent)));
            }
        }
    }
}

// --------------------------------------------------------------- metrics --

/// Cognitive-load metrics per ADR 0006, derived from the head-annotated tree.
#[derive(Debug, Clone, Copy)]
pub struct Metrics {
    /// Peak number of dependency arcs spanning any inter-word gap
    /// (comfortable ceiling ≈ 4 per the research findings).
    pub peak_open_deps: usize,
    /// Longest dependency in words.
    pub max_dep_len: usize,
    /// Deepest nesting of clause-level nodes.
    pub embedding_depth: usize,
    /// Share of dependencies whose dependent follows its head
    /// (head-initial = right-branching).
    pub right_branching: f64,
    /// Tokens before the sentence's main head word (fronted material).
    pub fronted: usize,
}

pub fn metrics(tree: &Tree) -> Metrics {
    let mut arcs: Vec<(usize, usize)> = Vec::new(); // (dependent, head)
    collect_arcs(tree, &mut arcs);
    let max_pos = arcs
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .max()
        .unwrap_or(0);
    let mut peak = 0usize;
    for gap in 0..max_pos {
        // arcs spanning the gap between word `gap` and `gap+1`
        let open = arcs
            .iter()
            .filter(|&&(a, b)| a.min(b) <= gap && a.max(b) > gap)
            .count();
        peak = peak.max(open);
    }
    let max_dep_len = arcs.iter().map(|&(a, b)| a.abs_diff(b)).max().unwrap_or(0);
    let right = arcs.iter().filter(|&&(dep, head)| dep > head).count();
    Metrics {
        peak_open_deps: peak,
        max_dep_len,
        embedding_depth: clause_depth(tree),
        right_branching: if arcs.is_empty() {
            1.0
        } else {
            right as f64 / arcs.len() as f64
        },
        fronted: tree.head_word_pos(),
    }
}

fn collect_arcs(tree: &Tree, arcs: &mut Vec<(usize, usize)>) {
    if let Tree::Node { head, children, .. } = tree {
        let h = children[*head].head_word_pos();
        for (i, c) in children.iter().enumerate() {
            if i != *head {
                arcs.push((c.head_word_pos(), h));
            }
            collect_arcs(c, arcs);
        }
    }
}

fn clause_depth(tree: &Tree) -> usize {
    match tree {
        Tree::Leaf { .. } => 0,
        Tree::Node { label, children, .. } => {
            let inner = children.iter().map(clause_depth).max().unwrap_or(0);
            let is_clause = matches!(*label, "S" | "Clause" | "Cond" | "Prohib" | "Imp");
            inner + usize::from(is_clause)
        }
    }
}

// ----------------------------------------------------------------- parse --

pub type ParseError =
    lalrpop_util::ParseError<usize, Tok, LexError>;

pub fn parse(lexicon: &Lexicon, sentence: &str) -> Result<Tree, String> {
    let tokens = lexicon
        .tokenize(sentence)
        .map_err(|e| e.to_string())?;
    let iter = tokens
        .into_iter()
        .map(|(i, t)| Ok::<(usize, Tok, usize), LexError>((i, t, i + 1)));
    minglish::SentenceParser::new()
        .parse(iter)
        .map_err(|e| format_parse_error(&e))
}

fn format_parse_error(e: &ParseError) -> String {
    match e {
        lalrpop_util::ParseError::UnrecognizedToken { token: (pos, tok, _), .. } => {
            format!("unexpected {tok:?} at word {pos} — no sanctioned sentence structure continues this way")
        }
        lalrpop_util::ParseError::UnrecognizedEof { .. } => {
            "sentence ended before a sanctioned structure was complete".to_string()
        }
        other => format!("{other:?}"),
    }
}
