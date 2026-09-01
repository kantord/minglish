//! lexgen — deterministically generates lexicon.tsv and docs/lexicon-report.md
//! from the curated seed list (seed/seed.json). See ADR 0001.
//!
//! Usage:
//!   cargo run -p lexgen            regenerate outputs (fails on lint errors)
//!   cargo run -p lexgen -- --check verify committed outputs are up to date

use std::collections::{BTreeMap, BTreeSet};
use std::process::exit;

mod morph;
mod refdata;
mod seed;

use refdata::RefData;
use seed::{Category, SeedEntry};

const SEED_PATH: &str = "seed/seed.json";
const LEXICON_PATH: &str = "lexicon.tsv";
const REPORT_PATH: &str = "docs/lexicon-report.md";
const CORPUS_PATH: &str = "corpus/accept.txt";

/// One enabled surface form in the lexicon.
struct Form {
    surface: String,
    tag: String,
    lemma: String,
    /// false when the surface came out of the regular rules (needs attestation),
    /// true when the curator typed it (lemma itself or an explicit override).
    explicit: bool,
}

/// Review trigger for redirect suggestions (ADR 0023): below this zipf a
/// suggested word is not common knowledge and the redirect needs review.
const REDIRECT_ZIPF_FLOOR: f64 = 3.5;

fn main() {
    let check_mode = std::env::args().any(|a| a == "--check");

    let entries = match seed::load(SEED_PATH) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("error: {SEED_PATH}: {e}");
            exit(1);
        }
    };
    let refdata = match RefData::load("data") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: loading reference data: {e}");
            exit(1);
        }
    };

    let mut errors: Vec<String> = Vec::new();

    // -- duplicate lemma+category check ------------------------------------
    let mut seen = BTreeSet::new();
    for e in &entries {
        if !seen.insert((e.lemma.clone(), e.category.clone())) {
            errors.push(format!(
                "duplicate seed entry: {} ({})",
                e.lemma, e.category
            ));
        }
    }

    // -- paradigm expansion ------------------------------------------------
    let mut forms: Vec<Form> = Vec::new();
    for e in &entries {
        forms.extend(expand(e, &mut errors));
    }

    // -- lint: collisions --------------------------------------------------
    let mut by_surface: BTreeMap<&str, Vec<&Form>> = BTreeMap::new();
    for f in &forms {
        by_surface.entry(&f.surface).or_default().push(f);
    }
    for (surface, claims) in &by_surface {
        if claims.len() > 1 {
            let detail: Vec<String> = claims
                .iter()
                .map(|f| format!("{} (lemma {})", f.tag, f.lemma))
                .collect();
            errors.push(format!(
                "collision: surface form \"{surface}\" claimed as {}",
                detail.join(" and ")
            ));
        }
    }

    // -- lint: unattested generated forms ----------------------------------
    for f in &forms {
        if !f.explicit && !refdata.attested(&f.surface) {
            errors.push(format!(
                "unattested form \"{}\" generated from lemma \"{}\" — if the \
                 rules misfired, add an explicit override in `forms`; if the \
                 form is genuinely right but rare, spell it out in `forms` to \
                 acknowledge it",
                f.surface, f.lemma
            ));
        }
    }

    // -- lint: a banned surface must not also be an enabled form ------------
    {
        let enabled: std::collections::BTreeSet<&str> =
            forms.iter().map(|f| f.surface.as_str()).collect();
        for e in &entries {
            if matches!(e.cat(), Category::Banned) {
                if enabled.contains(e.lemma.as_str()) {
                    errors.push(format!(
                        "\"{}\" is BANNED but also an enabled surface form",
                        e.lemma
                    ));
                }
                if e.advice.is_empty() {
                    errors.push(format!("BANNED \"{}\" needs an `advice` text", e.lemma));
                }
            }
        }
    }

    // -- lint: cross-POS completeness --------------------------------------
    for e in &entries {
        let Some(own_pos) = e.cat().wordnet_pos() else {
            continue; // closed classes are fiat; reference data can't judge them
        };
        let attested = refdata.pos_of(&e.lemma);
        for pos in attested {
            if pos == own_pos {
                continue;
            }
            let name = refdata::pos_name(pos);
            if e.reject.contains_key(name) || e.waive.iter().any(|w| w == name) {
                continue;
            }
            errors.push(format!(
                "cross-POS: \"{}\" is enabled as {} but is also attested as \
                 {name} — add a redirect in `reject` (\"{name}\": \"<word>\") \
                 or an explicit entry in `waive`",
                e.lemma, e.category
            ));
        }
    }

    if !errors.is_empty() {
        eprintln!("lexgen: {} error(s):\n", errors.len());
        for e in &errors {
            eprintln!("  - {e}");
        }
        exit(1);
    }

    // -- outputs -----------------------------------------------------------
    let lexicon = render_lexicon(&forms, &entries);
    let report = render_report(&forms, &entries, &refdata);

    if check_mode {
        let mut stale = Vec::new();
        if std::fs::read_to_string(LEXICON_PATH).ok().as_deref() != Some(lexicon.as_str()) {
            stale.push(LEXICON_PATH);
        }
        if std::fs::read_to_string(REPORT_PATH).ok().as_deref() != Some(report.as_str()) {
            stale.push(REPORT_PATH);
        }
        if stale.is_empty() {
            println!("lexgen --check: outputs are up to date");
        } else {
            eprintln!(
                "lexgen --check: stale outputs: {} — run `cargo run -p lexgen`",
                stale.join(", ")
            );
            exit(1);
        }
    } else {
        std::fs::write(LEXICON_PATH, &lexicon).expect("write lexicon.tsv");
        std::fs::write(REPORT_PATH, &report).expect("write report");
        println!(
            "lexgen: wrote {LEXICON_PATH} ({} forms, {} redirects) and {REPORT_PATH}",
            forms.len(),
            entries.iter().map(|e| e.reject.len()).sum::<usize>()
        );
    }
}

/// Expand one seed entry into its surface forms.
///
/// Within a lemma, syncretic slots merge: the slot order below is priority
/// order, and a surface already produced by an earlier slot is skipped
/// (e.g. `run` past-participle = base form ⇒ only VERB_TRANS_BASE remains).
fn expand(e: &SeedEntry, errors: &mut Vec<String>) -> Vec<Form> {
    let mut out: Vec<Form> = Vec::new();
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut push = |surface: String, tag: String, explicit: bool, out: &mut Vec<Form>| {
        if used.insert(surface.clone()) {
            out.push(Form {
                surface,
                tag,
                lemma: e.lemma.clone(),
                explicit,
            });
        }
    };

    let lemma = e.lemma.as_str();
    let over = |slot: &str| e.forms.get(slot).cloned();
    for slot in e.forms.keys() {
        if !e.cat().slots().contains(&slot.as_str()) {
            errors.push(format!(
                "\"{}\": `forms` override \"{slot}\" is not a slot of {}",
                e.lemma, e.category
            ));
        }
    }

    match e.cat() {
        Category::Noun => {
            push(lemma.into(), "NOUN_SG".into(), true, &mut out);
            let (pl, explicit) = match over("plural") {
                Some(p) => (p, true),
                None => (morph::pluralize(lemma), false),
            };
            push(pl, "NOUN_PL".into(), explicit, &mut out);
        }
        Category::VerbTrans | Category::VerbIntrans => {
            let base_tag = match e.cat() {
                Category::VerbTrans => "VERB_TRANS",
                _ => "VERB_INTRANS",
            };
            push(lemma.into(), format!("{base_tag}_BASE"), true, &mut out);
            let (third, ex3) = match over("third") {
                Some(t) => (t, true),
                None => (morph::third_singular(lemma), false),
            };
            push(third, format!("{base_tag}_3SG"), ex3, &mut out);
            let (past, exp) = match over("past") {
                Some(p) => (p, true),
                None => (morph::past(lemma), false),
            };
            push(past.clone(), format!("{base_tag}_ED"), exp, &mut out);
            // past participle defaults to the past form (already pushed then)
            if let Some(pp) = over("ppart") {
                push(pp, format!("{base_tag}_ED"), true, &mut out);
            }
            let (ing, exi) = match over("ing") {
                Some(g) => (g, true),
                None => (morph::gerund(lemma), false),
            };
            push(ing, format!("{base_tag}_ING"), exi, &mut out);
        }
        Category::Banned => {} // no forms; emitted as a ban row
        Category::Adj => push(lemma.into(), "ADJ".into(), true, &mut out),
        Category::Prep => push(lemma.into(), "PREP".into(), true, &mut out),
        Category::Det => push(lemma.into(), "DET".into(), true, &mut out),
        Category::Closed(tag) => push(lemma.into(), tag, true, &mut out),
    }
    out
}

fn render_lexicon(forms: &[Form], entries: &[SeedEntry]) -> String {
    let mut rows: Vec<(String, &str, String, String)> = Vec::new();
    for f in forms {
        rows.push((f.surface.clone(), "form", f.tag.clone(), f.lemma.clone()));
    }
    for e in entries {
        for (pos, suggestion) in &e.reject {
            rows.push((e.lemma.clone(), "reject", pos.clone(), suggestion.clone()));
        }
        if matches!(e.cat(), Category::Banned) {
            rows.push((e.lemma.clone(), "ban", "-".to_string(), e.advice.clone()));
        }
    }
    rows.sort();
    let mut out = String::from(
        "# generated by lexgen from seed/seed.json — do not edit\n\
         # surface\tkind\ttag\tvalue   (value = lemma for forms, suggestion for rejects)\n",
    );
    for (surface, kind, tag, value) in rows {
        out.push_str(&format!("{surface}\t{kind}\t{tag}\t{value}\n"));
    }
    out
}

fn render_report(forms: &[Form], entries: &[SeedEntry], refdata: &RefData) -> String {
    let mut out = String::from(
        "# Lexicon report\n\n*Generated by lexgen — do not edit.*\n\n",
    );

    // Summary
    let mut per_cat: BTreeMap<&str, usize> = BTreeMap::new();
    for e in entries {
        *per_cat.entry(e.category.as_str()).or_default() += 1;
    }
    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "- {} lemmas, {} surface forms, {} redirects, {} waivers\n",
        entries.len(),
        forms.len(),
        entries.iter().map(|e| e.reject.len()).sum::<usize>(),
        entries.iter().map(|e| e.waive.len()).sum::<usize>(),
    ));
    out.push_str("- Lemmas per category: ");
    let cats: Vec<String> = per_cat
        .iter()
        .map(|(c, n)| format!("{c} {n}"))
        .collect();
    out.push_str(&cats.join(", "));
    out.push_str("\n\n");

    // Frequency
    let mut zipfs: Vec<(f64, &str)> = entries
        .iter()
        .filter(|e| e.cat().wordnet_pos().is_some())
        .map(|e| (refdata.zipf(&e.lemma).unwrap_or(0.0), e.lemma.as_str()))
        .collect();
    zipfs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if !zipfs.is_empty() {
        let mean = zipfs.iter().map(|(z, _)| z).sum::<f64>() / zipfs.len() as f64;
        out.push_str("## Frequency (open-class lemmas)\n\n");
        out.push_str(&format!("- Mean zipf: {mean:.2} (higher = more common; everyday words sit at 4.5+)\n"));
        out.push_str("- Rarest 5: ");
        let rare: Vec<String> = zipfs
            .iter()
            .take(5)
            .map(|(z, w)| format!("{w} ({z:.2})"))
            .collect();
        out.push_str(&rare.join(", "));
        out.push_str("\n\n");
    }

    // Polysemy within enabled POS
    let mut senses: Vec<(u32, &str)> = entries
        .iter()
        .filter_map(|e| {
            e.cat()
                .wordnet_pos()
                .map(|p| (refdata.sense_count(&e.lemma, p), e.lemma.as_str()))
        })
        .collect();
    senses.sort_by(|a, b| b.cmp(a));
    if !senses.is_empty() {
        let mean = senses.iter().map(|(s, _)| *s as f64).sum::<f64>() / senses.len() as f64;
        out.push_str("## Residual polysemy (WordNet senses within the enabled POS)\n\n");
        out.push_str(&format!("- Mean senses: {mean:.1} (upper bound — WordNet oversplits)\n"));
        out.push_str("- Top 5: ");
        let top: Vec<String> = senses
            .iter()
            .take(5)
            .map(|(s, w)| format!("{w} ({s})"))
            .collect();
        out.push_str(&top.join(", "));
        out.push_str("\n\n");
    }

    // Redirect findability guard (report-only): the suggested word must be
    // common knowledge on its own — an absolute floor, not a gap from the
    // rejected word, because precise words are rarer by construction
    // (ADR 0023).
    let mut rare: Vec<(f64, String)> = Vec::new();
    for e in entries {
        for (pos, sugg) in &e.reject {
            let sugg_z = refdata.zipf(sugg).unwrap_or(0.0);
            if sugg_z < REDIRECT_ZIPF_FLOOR {
                rare.push((
                    sugg_z,
                    format!("{} ({pos}) → \"{sugg}\": zipf {sugg_z:.2}", e.lemma),
                ));
            }
        }
    }
    rare.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    out.push_str(&format!(
        "## Redirect findability guard (floor: zipf {REDIRECT_ZIPF_FLOOR})\n\n"
    ));
    if rare.is_empty() {
        out.push_str("Every redirect suggestion is above the floor. ✓\n\n");
    } else {
        for (_, line) in &rare {
            out.push_str(&format!("- ⚠ {line}\n"));
        }
        out.push('\n');
    }

    // Redirect targets outside the lexicon (report-only, ADR 0023): the
    // advice names a word the writer cannot actually use.
    {
        let enabled: std::collections::BTreeSet<&str> = entries
            .iter()
            .filter(|e| !matches!(e.cat(), Category::Banned))
            .map(|e| e.lemma.as_str())
            .collect();
        let outside: Vec<String> = entries
            .iter()
            .flat_map(|e| {
                e.reject.iter().filter(|(_, s)| !enabled.contains(s.as_str())).map(
                    move |(pos, s)| format!("{} ({pos}) → \"{s}\"", e.lemma),
                )
            })
            .collect();
        out.push_str("## Redirect targets outside the lexicon\n\n");
        if outside.is_empty() {
            out.push_str("Every redirect points at an enabled word. ✓\n\n");
        } else {
            out.push_str(
                "The advice names a word that is not itself enabled (ADR 0023 hole):\n\n",
            );
            for o in &outside {
                out.push_str(&format!("- {o}\n"));
            }
            out.push('\n');
        }
    }

    // Waivers = deliberate debt
    let waivers: Vec<String> = entries
        .iter()
        .flat_map(|e| e.waive.iter().map(move |w| format!("{} ({w})", e.lemma)))
        .collect();
    if !waivers.is_empty() {
        out.push_str("## Waivers (attested senses with no redirect)\n\n");
        for w in &waivers {
            out.push_str(&format!("- {w}\n"));
        }
        out.push('\n');
    }

    // Corpus coverage
    out.push_str("## Corpus coverage\n\n");
    out.push_str(
        "*v0 proxy: a sentence is covered when every token is an enabled \
         surface form. This does not measure whether rejections come with \
         clear, useful alternatives (see CONTEXT.md → Coverage).*\n\n",
    );
    match std::fs::read_to_string(CORPUS_PATH) {
        Err(_) => out.push_str(&format!("`{CORPUS_PATH}` not found — no coverage computed.\n")),
        Ok(corpus) => {
            let surfaces: BTreeSet<&str> = forms.iter().map(|f| f.surface.as_str()).collect();
            let mut covered = 0usize;
            let mut total = 0usize;
            let mut missing: BTreeMap<String, u32> = BTreeMap::new();
            for line in corpus.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                total += 1;
                let mut ok = true;
                for tok in line.split_whitespace() {
                    // NAMEs (capitalized or quoted, ADR 0018) are not lexicon words
                    if tok.starts_with('"')
                        || tok.chars().next().is_some_and(|c| c.is_uppercase())
                    {
                        continue;
                    }
                    let tok = tok
                        .trim_matches(|c: char| c.is_ascii_punctuation())
                        .to_lowercase();
                    if tok.is_empty() {
                        continue;
                    }
                    if !surfaces.contains(tok.as_str()) {
                        ok = false;
                        *missing.entry(tok).or_default() += 1;
                    }
                }
                if ok {
                    covered += 1;
                }
            }
            out.push_str(&format!(
                "- {covered}/{total} sentences fully lexicalized\n"
            ));
            if !missing.is_empty() {
                let mut m: Vec<(&String, &u32)> = missing.iter().collect();
                m.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
                out.push_str("- Missing tokens: ");
                let list: Vec<String> = m
                    .iter()
                    .map(|(w, n)| format!("{w} (×{n})"))
                    .collect();
                out.push_str(&list.join(", "));
                out.push('\n');
            }
        }
    }
    out
}
