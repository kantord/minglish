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
    findings.extend(slot_findings(lexicon, &toks));
    findings.sort();
    findings.dedup();
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
    // a word mentioned as a word must be quoted (ADR 0018)
    for w in toks.windows(2) {
        let function_word = matches!(
            w[1],
            Tok::Pron1(_) | Tok::Pron2(_) | Tok::Poss(_) | Tok::Det(_) | Tok::DetSg(_)
                | Tok::If(_) | Tok::Then(_) | Tok::No(_) | Tok::Every(_) | Tok::Some_(_)
        );
        if matches!(w[0], Tok::NounSg(_)) && function_word {
            out.push(format!(
                "\"{} {}\" — a word mentioned as a word must be quoted: \"{} \\\"{}\\\"\" (ADR 0018)",
                word(&w[0]), word(&w[1]), word(&w[0]), word(&w[1])
            ));
        }
    }
    // "the i pronoun": a word used as a name goes after the noun, in quotes
    for w in toks.windows(2) {
        if matches!(w[0], Tok::Pron1(_) | Tok::Pron2(_) | Tok::Poss(_)) && matches!(w[1], Tok::NounSg(_) | Tok::NounPl(_)) {
            out.push(format!(
                "\"{} {}\" — a word used as a name follows the noun in quotes: \"the {} \\\"{}\\\"\" (ADR 0018)",
                word(&w[0]), word(&w[1]), word(&w[1]), word(&w[0])
            ));
        }
    }
    // of-chains (ADR 0011): "the point of the split of the sentences"
    {
        let ofs: Vec<usize> = toks.iter().enumerate().filter(|(_, t)| matches!(t, Tok::PrepN(_))).map(|(i, _)| i).collect();
        for w in ofs.windows(2) {
            let between = &toks[w[0] + 1..w[1]];
            if !between.iter().any(|t| is_finite_verb(t) || matches!(t, Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_) | Tok::Conj(_) | Tok::Comma | Tok::ModalMust(_) | Tok::ModalCan(_) | Tok::ModalCannot(_) | Tok::Do3(_) | Tok::DoBase(_) | Tok::DoPast(_))) {
                out.push(
                    "\"of … of\" — \"of\" does not chain; name the inner thing in its own sentence, \
                     or drop one level (ADR 0011)"
                        .to_string(),
                );
                break;
            }
        }
    }
    if let [first, Tok::CopSg(_) | Tok::CopPl(_), ..] = toks {
        if matches!(first, Tok::Pron2(_) | Tok::Poss(_) | Tok::Det(_) | Tok::DetSg(_)) {
            out.push(format!(
                "\"{}\" used as a word must be quoted: \"\\\"{}\\\" is …\" (ADR 0018)",
                word(first), word(first)
            ));
        }
    }
    // bare singular noun: "prefers clarity" — every singular noun takes a
    // determiner (mass nouns take "the")
    for (i, t) in toks.iter().enumerate() {
        if let Tok::NounSg(n) = t {
            let prev = i.checked_sub(1).map(|j| &toks[j]);
            let determined = prev.is_some_and(|p| {
                is_det(p) || matches!(p, Tok::Adj(_) | Tok::NounSg(_) | Tok::Approx(_) | Tok::Percent(_)
                    | Tok::Pron1(_) | Tok::Pron2(_) | Tok::Poss(_))
            });
            let mentioned = toks.get(i + 1).is_some_and(|nx| matches!(nx, Tok::Name(_)));
            if !determined && !mentioned && i > 0 {
                out.push(format!(
                    "\"{n}\" — a singular noun needs a determiner: \"the {n}\" (mass nouns take \"the\")"
                ));
            }
        }
    }
    // clause as object: "shows pronouns are …" — two finite verbs and no
    // connective between them
    {
        let is_verbish = |t: &Tok| {
            is_finite_verb(t)
                || matches!(t, Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_)
                    | Tok::ModalMust(_) | Tok::ModalCan(_) | Tok::ModalCannot(_) | Tok::Do3(_) | Tok::DoPast(_))
        };
        let verbs: Vec<usize> = toks.iter().enumerate().filter(|(_, t)| is_verbish(t)).map(|(i, _)| i).collect();
        if let [a, b, ..] = verbs[..] {
            let connective = toks[a..b].iter().any(|t| matches!(t, Tok::Conj(_) | Tok::If(_) | Tok::Then(_) | Tok::So(_) | Tok::Because(_) | Tok::Comma));
            let reduced_relative = matches!(toks[a], Tok::VtEd(_) | Tok::ViEd(_)) && a >= 1 && matches!(toks[a - 1], Tok::NounSg(_) | Tok::NounPl(_));
            if !connective && !reduced_relative {
                out.push(format!(
                    "\"{} … {}\" — a clause cannot be the object of a verb; state the fact in its own \
                     sentence, or name it: \"the report shows the result\"",
                    word(&toks[a]), word(&toks[b])
                ));
            }
        }
    }
    // participle as a noun modifier: "the banned pronouns" (no participial
    // adjectives; the verb keeps the form)
    for w in toks.windows(3) {
        if is_det(&w[0]) && matches!(w[1], Tok::VtEd(_) | Tok::VtIng(_) | Tok::ViEd(_) | Tok::ViIng(_))
            && matches!(w[2], Tok::NounSg(_) | Tok::NounPl(_))
        {
            out.push(format!(
                "\"{} {} {}\" — a verb form cannot modify a noun; say who does it: \
                 \"the Linter bans the pronouns\", or split the sentence",
                word(&w[0]), word(&w[1]), word(&w[2])
            ));
        }
    }
    // a preposition other than "of" right after a subject noun attaches to
    // the verb, not the noun (ADR 0011): "pronouns for the speaker are …"
    {
        let first_verb = toks.iter().position(|t| {
            is_finite_verb(t)
                || matches!(t, Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_)
                    | Tok::ModalMust(_) | Tok::ModalCan(_) | Tok::ModalCannot(_) | Tok::Do3(_) | Tok::DoBase(_) | Tok::DoPast(_))
        });
        if let Some(v) = first_verb {
            for i in 1..v {
                if matches!(toks[i - 1], Tok::NounSg(_) | Tok::NounPl(_)) {
                    if let Tok::PrepV(p) = &toks[i] {
                        out.push(format!(
                            "\"{} {p} …\" — only \"of\" attaches to a noun; \"{p}\" attaches to the verb. \
                             Write \"the {} of …\", or move the phrase after the verb (ADR 0011)",
                            word(&toks[i - 1]), word(&toks[i - 1])
                        ));
                    }
                }
            }
        }
    }
    // copula + prepositional phrase / adjective + PP (ADR 0003; ADR 0023 deferral)
    for (i, t) in toks.iter().enumerate() {
        if !matches!(t, Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_)) {
            continue;
        }
        let mut j = i + 1;
        if matches!(toks.get(j), Some(Tok::Neg(_))) {
            j += 1;
        }
        match (toks.get(j), toks.get(j + 1)) {
            (Some(Tok::PrepV(p) | Tok::PrepN(p)), _) => out.push(format!(
                "\"{} {p} …\" — the copula takes an adjective or a noun phrase, not a \
                 prepositional phrase; use a verb: \"the Lexicon contains the pronouns\" (ADR 0003)",
                word(t)
            )),
            (Some(Tok::Adj(a)), Some(Tok::PrepV(p))) => out.push(format!(
                "\"{a} {p} …\" — an adjective cannot take a prepositional phrase yet; \
                 restructure with a verb, or split the sentence (deferred, ADR 0023)"
            )),
            _ => {}
        }
    }
    // noun-phrase coordination (ADR 0004: coordinate predicates or clauses)
    for (i, t) in toks.iter().enumerate() {
        if !matches!(t, Tok::Conj(_)) || i == 0 || !matches!(toks[i - 1], Tok::NounSg(_) | Tok::NounPl(_) | Tok::Name(_)) {
            continue;
        }
        let mut j = i + 1;
        if toks.get(j).is_some_and(is_det) {
            j += 1;
        }
        while matches!(toks.get(j), Some(Tok::Adj(_))) {
            j += 1;
        }
        let verb_before = toks[..i].iter().any(|t| {
            is_finite_verb(t)
                || matches!(t, Tok::CopSg(_) | Tok::CopPl(_) | Tok::CopSgPast(_) | Tok::CopPlPast(_)
                    | Tok::ModalMust(_) | Tok::ModalCan(_) | Tok::ModalCannot(_) | Tok::Do3(_) | Tok::DoBase(_) | Tok::DoPast(_))
        });
        if matches!(toks.get(j), Some(Tok::NounSg(_) | Tok::NounPl(_) | Tok::Name(_)))
            && (!verb_before || toks.get(j + 1).is_none_or(|n| matches!(n, Tok::PrepV(_) | Tok::Comma | Tok::Conj(_))))
        {
            out.push(
                "noun phrases cannot be coordinated — repeat the verb: \"the mechanism stores \
                 a word and stores a message\", or split the sentence (ADR 0004)"
                    .to_string(),
            );
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

// ------------------------------------------------ slot (redirect) findings --

fn is_det(t: &Tok) -> bool {
    matches!(t, Tok::Det(_) | Tok::DetSg(_) | Tok::Poss(_) | Tok::Num(_) | Tok::NumPl(_) | Tok::Every(_) | Tok::No(_) | Tok::Some_(_))
}

fn is_noun_head(t: &Tok) -> bool {
    matches!(t, Tok::NounSg(_) | Tok::NounPl(_) | Tok::Name(_) | Tok::Pron1(_) | Tok::Pron2(_))
}

/// A word in the wrong slot whose rejected sense has a redirect (ideas,
/// advice gap 2): "the cost" (cost is a verb; noun → expense), "the agent
/// files the report" (files is a noun; verb → submit).
fn slot_findings(lexicon: &Lexicon, toks: &[Tok]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, t) in toks.iter().enumerate() {
        let prev = i.checked_sub(1).map(|j| &toks[j]);
        let next = toks.get(i + 1);
        match t {
            // verb form in a noun slot: right after a determiner
            Tok::VtBase(w) | Tok::ViBase(w) | Tok::Vt3(w) | Tok::Vi3(w)
                if prev.is_some_and(is_det) =>
            {
                out.push(match lexicon.redirect(w, "NOUN") {
                    Some(s) => format!("\"{w}\" is a verb in minglish — as a noun use \"{s}\""),
                    None => format!("\"{w}\" is a verb in minglish and cannot follow a determiner"),
                });
            }
            // noun form in a verb slot: after a subject head, before an object start
            Tok::NounSg(w) | Tok::NounPl(w)
                if prev.is_some_and(is_noun_head) && next.is_some_and(|n| is_det(n) || matches!(n, Tok::Adj(_))) =>
            {
                if let Some(s) = lexicon.redirect(w, "VERB") {
                    out.push(format!("\"{w}\" is a noun in minglish — as a verb use \"{s}\""));
                }
            }
            _ => {}
        }
    }
    // a defined multi-word term written in lowercase ("reference ambiguity")
    // beats the compound rule below (ADR 0027)
    let mut term_spans: Vec<(usize, usize)> = Vec::new();
    for len in [3usize, 2] {
        for i in 0..toks.len().saturating_sub(len - 1) {
            if term_spans.iter().any(|&(s, e)| i < e && i + len > s) {
                continue;
            }
            let ws: Vec<&str> = toks[i..i + len].iter().map(word).collect();
            if ws.iter().any(|w| *w == "," || w.starts_with('"')) {
                continue;
            }
            let joined = ws.join(" ").to_lowercase();
            if let Some(cap) = lexicon.term(&joined) {
                out.push(format!("\"{joined}\" is a defined term — write \"{cap}\" (see CONTEXT.md)"));
                term_spans.push((i, i + len));
            }
        }
    }
    // noun-noun compound (ADR 0015): two nouns in a row, unless the second
    // is a noun form used in the verb slot (handled above) or the pair is a
    // defined term (handled just above)
    for (i, w) in toks.windows(2).enumerate() {
        if let (Tok::NounSg(a), Tok::NounSg(b) | Tok::NounPl(b)) = (&w[0], &w[1]) {
            if term_spans.iter().any(|&(s, e)| i >= s && i + 1 < e) {
                continue;
            }
            let verb_slot = toks.get(i + 2).is_some_and(|n| is_det(n) || matches!(n, Tok::Adj(_)))
                && lexicon.redirect(b, "VERB").is_some();
            if !verb_slot {
                out.push(format!(
                    "\"{a} {b}\" — noun-noun compounds are not minglish; write \"the {b} of the {a}\", \
                     or one transparent word (ADR 0015)"
                ));
            }
        }
    }
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
