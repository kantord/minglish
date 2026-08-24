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
            Category::VerbTrans | Category::VerbIntrans => {
                &["third", "past", "ppart", "ing"]
            }
            _ => &[],
        }
    }
}

pub fn load(path: &str) -> Result<Vec<SeedEntry>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let entries: Vec<SeedEntry> =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;
    for e in &entries {
        if e.lemma.is_empty()
            || !e
                .lemma
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-')
        {
            return Err(format!(
                "lemma \"{}\" must be lowercase ascii (hyphens allowed)",
                e.lemma
            ));
        }
    }
    Ok(entries)
}
