//! Tier-2 diagnosis: when tier-1 rejects a sentence, explain why.
//!
//! A deliberately loose superset grammar (agreement ignored, transitivity
//! ignored, banned attachments and orders admitted) is parsed with an
//! all-parses counter:
//!   count > 1 → AMBIGUOUS — the sentence has several readings, and the
//!               count itself is the red flag (see CONTEXT.md "Rejection");
//!   count = 1 → STYLE — a banned construction, named by pattern checks;
//!   count = 0 → not recognizable as an English-like clause at all.
//! Pattern checks over the token stream attach specific, actionable names
//! (missing *then*, quantifier+negation, reduced relative, …).

use grammar::{metrics, parse, Lexicon, Metrics, Tok};
use std::collections::BTreeMap;

// ------------------------------------------------------------ diagnosis --

#[derive(Debug)]
pub enum Diagnosis {
    /// Tier-1 accepts: unique parse, with its load metrics.
    Clean(Metrics),
    /// A word-level rejection (unknown word / rejected sense), with advice.
    Word(String),
    /// Banned structure with exactly one tier-2 reading.
    Style(Vec<String>),
    /// Multiple tier-2 readings: structurally ambiguous.
    Ambiguous { readings: u64, findings: Vec<String> },
    /// Tier-2 cannot recognize it either.
    Unknown,
}

pub fn diagnose(lexicon: &Lexicon, sentence: &str) -> Diagnosis {
    match parse(lexicon, sentence) {
        Ok(tree) => return Diagnosis::Clean(metrics(&tree)),
        Err(e) if e.contains("not a minglish word")
            || e.contains("is banned in minglish")
            || e.contains("not yet usable") => {
            return Diagnosis::Word(e)
        }
        Err(_) => {}
    }
    let Ok(toks) = lexicon.tokenize(sentence) else {
        return Diagnosis::Unknown;
    };
    let toks: Vec<Tok> = toks.into_iter().map(|(_, t)| t).collect();
    let mut findings = pattern_findings(&toks);
    let readings = Tier2::new(&toks).count();
    match readings {
        0 => {
            if findings.is_empty() {
                Diagnosis::Unknown
            } else {
                Diagnosis::Style(findings)
            }
        }
        1 => {
            if findings.is_empty() {
                findings.push(
                    "this structure is outside the sanctioned sentence shapes — \
                     restructure into one of the minglish templates"
                        .to_string(),
                );
            }
            Diagnosis::Style(findings)
        }
        n => {
            findings.push(
                "split the sentence, or move the prepositional phrase so each \
                 attachment is explicit"
                    .to_string(),
            );
            Diagnosis::Ambiguous { readings: n, findings }
        }
    }
}

// ------------------------------------------------------ pattern findings --

fn word(t: &Tok) -> &str {
    match t {
        Tok::Det(w) | Tok::DetSg(w) | Tok::Adj(w) | Tok::NounSg(w) | Tok::NounPl(w)
        | Tok::VtBase(w) | Tok::Vt3(w) | Tok::VtEd(w) | Tok::VtIng(w) | Tok::ViBase(w)
        | Tok::Vi3(w) | Tok::ViEd(w) | Tok::ViIng(w) | Tok::PrepN(w) | Tok::PrepV(w)
        | Tok::Pron1(w) | Tok::Pron2(w) | Tok::Poss(w) | Tok::CopSg(w) | Tok::CopPl(w)
        | Tok::CopSgPast(w) | Tok::CopPlPast(w) | Tok::Conj(w) | Tok::Neg(w)
        | Tok::DoBase(w) | Tok::Do3(w) | Tok::DoPast(w) | Tok::ModalMust(w)
        | Tok::ModalCan(w) | Tok::ModalCannot(w) | Tok::If(w) | Tok::Then(w)
        | Tok::Every(w) | Tok::No(w) | Tok::Num(w) | Tok::NumPl(w) | Tok::Percent(w)
        | Tok::Approx(w) | Tok::So(w) | Tok::Because(w) | Tok::Some_(w) | Tok::Name(w) => w,
        Tok::Comma => ",",
    }
}

fn is_finite_verb(t: &Tok) -> bool {
    matches!(
        t,
        Tok::VtBase(_) | Tok::Vt3(_) | Tok::VtEd(_) | Tok::ViBase(_) | Tok::Vi3(_) | Tok::ViEd(_)
    )
}

fn pattern_findings(toks: &[Tok]) -> Vec<String> {
    let mut out = Vec::new();
    let has = |f: &dyn Fn(&Tok) -> bool| toks.iter().any(|t| f(t));

    // conditionals: missing then / consequent-first
    if matches!(toks.first(), Some(Tok::If(_))) && !has(&|t| matches!(t, Tok::Then(_))) {
        out.push(
            "conditional without \"then\" — write: if <clause>, then <clause> (ADR 0007)"
                .to_string(),
        );
    }
    if toks.iter().skip(1).any(|t| matches!(t, Tok::If(_))) {
        out.push(
            "condition must come first — write: if <clause>, then <clause> (ADR 0007)"
                .to_string(),
        );
    }
    // quantifier + negation
    if matches!(toks.first(), Some(Tok::Every(_)))
        && has(&|t| matches!(t, Tok::Neg(_) | Tok::ModalCannot(_)))
    {
        out.push(
            "\"every … not\" is scope-ambiguous — for none write \"no <noun> …\"; \
             for not-all write \"some <nouns> do not …\" (ADR 0014)"
                .to_string(),
        );
    }
    if matches!(toks.first(), Some(Tok::No(_)))
        && has(&|t| matches!(t, Tok::Neg(_) | Tok::ModalCannot(_)))
    {
        out.push("\"no …\" already negates — drop the second negation (ADR 0014)".to_string());
    }
    if matches!(toks.first(), Some(Tok::No(_))) && has(&|t| matches!(t, Tok::ModalMust(_))) {
        out.push(
            "\"no … must\" is ambiguous — for prohibition write, for example, \
             \"agents must not check the input\" (bare plural + must not, ADR 0014)"
                .to_string(),
        );
    }
    // causal connectives (ADR 0026): never sentence-initial; comma mandatory
    if matches!(toks.first(), Some(Tok::Because(_))) {
        out.push(
            "\"because\" cannot start a sentence — write the result first: \
             \"<result>, because <reason>\"; or cause first: \"<cause>, so <result>\" (ADR 0026)"
                .to_string(),
        );
    }
    if matches!(toks.first(), Some(Tok::So(_))) {
        out.push(
            "\"so\" cannot start a sentence — join the cause in the same sentence: \
             \"<cause>, so <result>\" (ADR 0026)"
                .to_string(),
        );
    }
    for w in toks.windows(2) {
        if matches!(w[1], Tok::So(_) | Tok::Because(_)) && !matches!(w[0], Tok::Comma) {
            out.push(format!(
                "a comma before \"{}\" is mandatory — \"<clause>, {} <clause>\" (ADR 0026)",
                word(&w[1]),
                word(&w[1])
            ));
        }
    }
    // no / some misplacement
    if toks.iter().skip(1).any(|t| matches!(t, Tok::No(_))) {
        out.push(
            "\"no\" is subject-only — write \"… does not <verb> <nouns>\" instead (ADR 0014)"
                .to_string(),
        );
    }
    for w in toks.windows(2) {
        if matches!(w[0], Tok::Some_(_)) && matches!(w[1], Tok::NounSg(_) | Tok::Adj(_)) {
            if let [Tok::Some_(_), Tok::NounSg(_)] = w {
                out.push("\"some\" takes a plural noun (ADR 0017)".to_string());
            }
        }
        // determiner–number agreement
        if matches!(w[0], Tok::DetSg(_)) && matches!(w[1], Tok::NounPl(_)) {
            out.push(format!(
                "\"{} {}\" — a/an take a singular noun",
                word(&w[0]),
                word(&w[1])
            ));
        }
        if matches!(w[0], Tok::Num(_)) && matches!(w[1], Tok::NounPl(_)) {
            out.push(format!(
                "\"{} {}\" — one takes a singular noun (ADR 0016)",
                word(&w[0]),
                word(&w[1])
            ));
        }
        if matches!(w[0], Tok::NumPl(_)) && matches!(w[1], Tok::NounSg(_)) {
            out.push(format!(
                "\"{} {}\" — a number takes a plural noun (ADR 0022)",
                word(&w[0]),
                word(&w[1])
            ));
        }
        // perfect aspect via have
        if matches!(&w[0], Tok::Vt3(v) | Tok::VtBase(v) | Tok::VtEd(v) if ["have","has","had"].contains(&v.as_str()))
            && matches!(w[1], Tok::VtEd(_) | Tok::ViEd(_))
        {
            out.push(
                "perfect aspect (\"has <verb>-ed\") is not in minglish — use the simple past \
                 (ADR 0016)"
                    .to_string(),
            );
        }
        // progressive / passive after copula
        if matches!(w[0], Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_)) {
            match &w[1] {
                Tok::VtIng(_) | Tok::ViIng(_) => out.push(
                    "progressive (\"is <verb>-ing\") is not in minglish — use the simple form \
                     (ADR 0003)"
                        .to_string(),
                ),
                Tok::VtEd(_) => out.push(
                    "passive (\"is <verb>-ed\") is not in minglish — name the doer and use \
                     active voice (ADR 0003)"
                        .to_string(),
                ),
                _ => {}
            }
        }
    }
    // reduced relative: noun + -ed verb + … + second finite verb, no conjunction
    if !has(&|t| matches!(t, Tok::Conj(_) | Tok::If(_))) {
        let verbs: Vec<usize> = toks
            .iter()
            .enumerate()
            .filter(|(_, t)| is_finite_verb(t) || matches!(t, Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_)))
            .map(|(i, _)| i)
            .collect();
        if verbs.len() >= 2 {
            if let Tok::VtEd(_) | Tok::ViEd(_) = toks[verbs[0]] {
                if verbs[0] >= 1 && matches!(toks[verbs[0] - 1], Tok::NounSg(_) | Tok::NounPl(_)) {
                    out.push(
                        "this reads as a reduced relative (\"the file stored …\") — minglish \
                         has no relative clauses yet; split into two sentences (ADR 0010)"
                            .to_string(),
                    );
                }
            }
        }
    }
    // transitive verb with no object
    for (i, t) in toks.iter().enumerate() {
        if matches!(t, Tok::VtBase(_) | Tok::Vt3(_) | Tok::VtEd(_)) {
            let next = toks.get(i + 1);
            let object_start = matches!(
                next,
                Some(Tok::Det(_) | Tok::DetSg(_) | Tok::Poss(_) | Tok::Num(_) | Tok::NumPl(_) | Tok::Approx(_) | Tok::Every(_)
                    | Tok::Adj(_) | Tok::NounSg(_) | Tok::NounPl(_))
            );
            let have_like = ["have", "has", "had"].contains(&word(t));
            if !object_start && !have_like && (next.is_none() || matches!(next, Some(Tok::PrepV(_)))) {
                out.push(format!(
                    "\"{}\" is transitive in minglish and needs an object",
                    word(t)
                ));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

// --------------------------------------------------- tier-2 loose parser --

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Term {
    Det, DetSg, Poss, Every, No, Some_, Num, Adj, NSg, NPl,
    VAny, PrepN, PrepV, Pron, CopAny, Conj, Neg, DoAny, ModAny, If, Then, Comma,
    Ing, Ed, NameT, Pct, Approx, So, Because,
}

fn term_of(t: &Tok) -> Vec<Term> {
    match t {
        Tok::Det(_) => vec![Term::Det],
        Tok::DetSg(_) => vec![Term::DetSg],
        Tok::Poss(_) => vec![Term::Poss],
        Tok::Every(_) => vec![Term::Every],
        Tok::No(_) => vec![Term::No],
        Tok::Some_(_) => vec![Term::Some_],
        Tok::Num(_) | Tok::NumPl(_) => vec![Term::Num],
        Tok::Percent(_) => vec![Term::Pct],
        Tok::Approx(_) => vec![Term::Approx],
        Tok::So(_) => vec![Term::So],
        Tok::Because(_) => vec![Term::Because],
        Tok::Adj(_) => vec![Term::Adj],
        Tok::NounSg(_) => vec![Term::NSg],
        Tok::NounPl(_) => vec![Term::NPl],
        Tok::VtBase(_) | Tok::Vt3(_) | Tok::ViBase(_) | Tok::Vi3(_) => vec![Term::VAny],
        Tok::VtEd(_) | Tok::ViEd(_) => vec![Term::VAny, Term::Ed],
        Tok::VtIng(_) | Tok::ViIng(_) => vec![Term::Ing],
        Tok::PrepN(_) => vec![Term::PrepN],
        Tok::PrepV(_) => vec![Term::PrepV],
        Tok::Pron1(_) | Tok::Pron2(_) => vec![Term::Pron],
        Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_) => vec![Term::CopAny],
        Tok::Conj(_) => vec![Term::Conj],
        Tok::Neg(_) => vec![Term::Neg],
        Tok::DoBase(_) | Tok::Do3(_) | Tok::DoPast(_) => vec![Term::DoAny],
        Tok::ModalMust(_) | Tok::ModalCan(_) | Tok::ModalCannot(_) => vec![Term::ModAny],
        Tok::If(_) => vec![Term::If],
        Tok::Then(_) => vec![Term::Then],
        Tok::Name(_) => vec![Term::NameT],
        Tok::Comma => vec![Term::Comma],
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Sym {
    T(Term),
    N(usize),
}

// Nonterminal ids
const S2: usize = 0;
const CL: usize = 1;
const NPS: usize = 2;
const DETX: usize = 3;
const ADJS: usize = 4;
const NH: usize = 5;
const PPS: usize = 6;
const PP: usize = 7;
const PRED: usize = 8;
const VPX: usize = 9;
const COMPL: usize = 10;
const N_NONTERMS: usize = 11;

fn productions() -> Vec<Vec<Vec<Sym>>> {
    use Sym::{N, T};
    let mut p = vec![Vec::new(); N_NONTERMS];
    p[S2] = vec![
        vec![N(CL)],
        vec![N(CL), T(Term::Conj), N(CL)],
        vec![N(CL), T(Term::Conj), N(VPX)],
        vec![T(Term::If), N(CL), T(Term::Comma), T(Term::Then), N(CL)],
        vec![T(Term::If), N(CL), T(Term::Then), N(CL)],
        vec![T(Term::If), N(CL), T(Term::Comma), N(CL)],
        vec![N(CL), T(Term::If), N(CL)],
        vec![T(Term::DoAny), T(Term::Neg), N(VPX)],
        vec![N(VPX)], // bare imperative
        // causal (ADR 0026) and its dirty shapes: missing comma, fronted because
        vec![N(CL), T(Term::Comma), T(Term::So), N(CL)],
        vec![N(CL), T(Term::Comma), T(Term::Because), N(CL)],
        vec![N(CL), T(Term::So), N(CL)],
        vec![N(CL), T(Term::Because), N(CL)],
        vec![T(Term::Because), N(CL), T(Term::Comma), N(CL)],
        vec![T(Term::So), N(CL)],
    ];
    p[CL] = vec![vec![N(NPS), N(PRED)]];
    p[NPS] = vec![
        vec![N(DETX), N(ADJS), N(NH), N(PPS)],
        vec![T(Term::Num), T(Term::Pct), N(PPS)], // 43 percent of the swaps
        vec![T(Term::Approx), T(Term::Num), T(Term::Pct), N(PPS)],
        vec![N(ADJS), T(Term::NPl), N(PPS)],
        vec![T(Term::NSg), N(PPS)], // bare singular (dirty)
        vec![T(Term::Pron)],
        vec![T(Term::NameT)],
        vec![N(DETX), N(ADJS), N(NH), T(Term::NameT), N(PPS)],
    ];
    p[DETX] = vec![
        vec![T(Term::Det)], vec![T(Term::DetSg)], vec![T(Term::Poss)],
        vec![T(Term::Every)], vec![T(Term::No)], vec![T(Term::Some_)], vec![T(Term::Num)],
        vec![T(Term::Approx), T(Term::Num)],
    ];
    p[ADJS] = vec![vec![], vec![T(Term::Adj), N(ADJS)]];
    p[NH] = vec![vec![T(Term::NSg)], vec![T(Term::NPl)]];
    p[PPS] = vec![vec![], vec![N(PP), N(PPS)]];
    p[PP] = vec![
        vec![T(Term::PrepN), N(NPS)],
        vec![T(Term::PrepV), N(NPS)], // under NP this is the classic dirty attachment
    ];
    p[PRED] = vec![
        vec![T(Term::VAny), N(PPS)],
        vec![T(Term::VAny), N(NPS), N(PPS)],
        vec![T(Term::CopAny), N(COMPL)],
        vec![T(Term::CopAny), T(Term::Neg), N(COMPL)],
        vec![T(Term::DoAny), T(Term::Neg), N(VPX)],
        vec![T(Term::ModAny), N(VPX)],
        vec![T(Term::ModAny), T(Term::Neg), N(VPX)],
    ];
    p[VPX] = vec![
        vec![T(Term::VAny), N(PPS)],
        vec![T(Term::VAny), N(NPS), N(PPS)],
        vec![T(Term::Ing), N(PPS)],
        vec![T(Term::Ing), N(NPS), N(PPS)],
    ];
    p[COMPL] = vec![
        vec![T(Term::Adj)],
        vec![N(NPS)],
        vec![T(Term::Ed), N(PPS)],  // passive shape (dirty)
        vec![T(Term::Ing), N(PPS)], // progressive shape (dirty)
        vec![T(Term::Ing), N(NPS), N(PPS)],
    ];
    p
}

pub struct Tier2 {
    terms: Vec<Vec<Term>>,
    prods: Vec<Vec<Vec<Sym>>>,
    memo: BTreeMap<(usize, usize), BTreeMap<usize, u64>>,
}

impl Tier2 {
    pub fn new(toks: &[Tok]) -> Tier2 {
        Tier2 {
            terms: toks.iter().map(term_of).collect(),
            prods: productions(),
            memo: BTreeMap::new(),
        }
    }

    /// Number of distinct tier-2 parse trees for the whole token stream.
    pub fn count(&mut self) -> u64 {
        let n = self.terms.len();
        if n == 0 {
            return 0;
        }
        *self.nonterm(S2, 0).get(&n).unwrap_or(&0)
    }

    /// end position → number of derivations of `nt` starting at `i`.
    fn nonterm(&mut self, nt: usize, i: usize) -> BTreeMap<usize, u64> {
        if let Some(m) = self.memo.get(&(nt, i)) {
            return m.clone();
        }
        // no left recursion in the grammar, so seeding with empty is sound
        self.memo.insert((nt, i), BTreeMap::new());
        let mut result: BTreeMap<usize, u64> = BTreeMap::new();
        let prods = self.prods[nt].clone();
        for prod in prods {
            let mut ends: BTreeMap<usize, u64> = BTreeMap::from([(i, 1)]);
            for sym in prod {
                let mut next: BTreeMap<usize, u64> = BTreeMap::new();
                for (&pos, &cnt) in &ends {
                    match sym {
                        Sym::T(term) => {
                            if pos < self.terms.len() && self.terms[pos].contains(&term) {
                                *next.entry(pos + 1).or_default() += cnt;
                            }
                        }
                        Sym::N(sub) => {
                            for (&end, &c2) in &self.nonterm(sub, pos) {
                                *next.entry(end).or_default() += cnt * c2;
                            }
                        }
                    }
                }
                ends = next;
                if ends.is_empty() {
                    break;
                }
            }
            for (end, cnt) in ends {
                *result.entry(end).or_default() += cnt;
            }
        }
        self.memo.insert((nt, i), result.clone());
        result
    }
}
