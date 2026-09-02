//! The curated seed list: the only hand-edited input (ADR 0001).

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeedEntry {
    pub lemma: String,
    pub category: String,
    /// Irregular-form overrides, keyed by slot name (see Category::slots).
    #[serde(default)]
    pub forms: BTreeMap<String, String>,
    /// Rejected uses: attested-but-banned POS → suggested replacement word.
    #[serde(default)]
    pub reject: BTreeMap<String, String>,
    /// Attested POSes deliberately left without a redirect.
    #[serde(default)]
    pub waive: Vec<String>,
    /// Writer-facing advice for BANNED words (shown by the linter).
    #[serde(default)]
    pub advice: String,
    /// Domain-model entries only (ADR 0027): the term's meaning, in
    /// minglish — self-linted, shown to writers and to the repair model.
    #[serde(default)]
    pub definition: String,
    /// True for entries loaded from domain/model.json (never serialized).
    #[serde(skip)]
    pub domain: bool,
    /// Curation rationale. Free text, never machine-read (hence dead_code).
    #[serde(default)]
    #[allow(dead_code)]
    pub note: String,
}

pub enum Category {
    Noun,
    VerbTrans,
    VerbIntrans,
    Adj,
    Prep,
    Det,
    /// A banned surface form with writer-facing advice (ADR 0008).
    Banned,
    /// A proper name with a definition (domain model only): no forms — the
    /// lexer already treats capitalized words as names (ADR 0018).
    Name,
    /// Function words: any other UPPERCASE tag, by fiat (CONJ, NEG, …).
    Closed(String),
}

impl SeedEntry {
    pub fn cat(&self) -> Category {
        match self.category.as_str() {
            "NOUN" => Category::Noun,
            "VERB_TRANS" => Category::VerbTrans,
            "VERB_INTRANS" => Category::VerbIntrans,
            "ADJ" => Category::Adj,
            "PREP" => Category::Prep,
            "DET" => Category::Det,
            "BANNED" => Category::Banned,
            "NAME" => Category::Name,
            other => Category::Closed(other.to_string()),
        }
    }
}

impl Category {
    /// The WordNet POS letter this category occupies, if it is open-class.
    pub fn wordnet_pos(&self) -> Option<char> {
        match self {
            Category::Noun => Some('n'),
            Category::VerbTrans | Category::VerbIntrans => Some('v'),
            Category::Adj => Some('a'),
            _ => None,
        }
    }

    /// Valid `forms` override slots.
    pub fn slots(&self) -> &'static [&'static str] {
        match self {
            Category::Noun => &["plural"],
            // ADR 0030: short adjectives inflect (-er); "none" opts out
            Category::Adj => &["comparative"],
            Category::VerbTrans | Category::VerbIntrans => {
                &["third", "past", "ppart", "ing"]
            }
            // a NAME may spell its capitalization ("WordNet")
            Category::Name => &["name"],
            _ => &[],
        }
    }
}

pub fn load(path: &str) -> Result<Vec<SeedEntry>, String> {
    load_with(path, false)
}

/// Load a seed-shaped file. Domain-model entries may have multi-word lemmas
/// ("anaphoric pronoun") and must carry a definition.
pub fn load_with(path: &str, domain: bool) -> Result<Vec<SeedEntry>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut entries: Vec<SeedEntry> =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;
    for e in &mut entries {
        e.domain = domain;
        let ok = !e.lemma.is_empty()
            && e.lemma
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-' || (domain && c == ' '));
        if !ok {
            return Err(format!(
                "lemma \"{}\" must be lowercase ascii (hyphens{} allowed)",
                e.lemma,
                if domain { " and spaces" } else { "" }
            ));
        }
        if domain && e.definition.trim().is_empty() {
            return Err(format!("domain term \"{}\" needs a `definition`", e.lemma));
        }
        if !domain && !e.definition.is_empty() {
            return Err(format!("\"{}\": `definition` belongs in domain/model.json", e.lemma));
        }
    }
    Ok(entries)
}

/// The written form of a NAME entry: its `name` override or the capitalized lemma.
pub fn shown_name(e: &SeedEntry) -> String {
    e.forms.get("name").cloned().unwrap_or_else(|| capitalize(&e.lemma))
}

/// "anaphoric pronoun" → "Anaphoric Pronoun" (the written form of a term).
pub fn capitalize(lemma: &str) -> String {
    lemma
        .split(' ')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
