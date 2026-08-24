//! Vendored reference data (data/ — see data/README.md). Used for CHECKING
//! only, never for choosing words (ADR 0001).

use std::collections::{BTreeMap, BTreeSet};

pub struct RefData {
    /// word → set of WordNet/moby POS letters: n, v, a, r
    pos: BTreeMap<String, BTreeSet<char>>,
    /// (word, pos letter) → WordNet synset count
    senses: BTreeMap<(String, char), u32>,
    /// word → zipf frequency
    zipf: BTreeMap<String, f64>,
    /// every word known to any source (attestation universe)
    known: BTreeSet<String>,
}

pub fn pos_name(pos: char) -> &'static str {
    match pos {
        'n' => "NOUN",
        'v' => "VERB",
        'a' => "ADJ",
        'r' => "ADV",
        _ => "?",
    }
}

impl RefData {
    pub fn load(dir: &str) -> Result<RefData, String> {
        let mut r = RefData {
            pos: BTreeMap::new(),
            senses: BTreeMap::new(),
            zipf: BTreeMap::new(),
            known: BTreeSet::new(),
        };
        for (file, pos) in [
            ("index.noun", 'n'),
            ("index.verb", 'v'),
            ("index.adj", 'a'),
            ("index.adv", 'r'),
        ] {
            r.load_wordnet_index(&format!("{dir}/wordnet/{file}"), pos)?;
        }
        r.load_mobypos(&format!("{dir}/moby/mobypos.txt"))?;
        r.load_zipf(&format!("{dir}/freq/en_zipf.tsv"))?;
        Ok(r)
    }

    pub fn attested(&self, word: &str) -> bool {
        self.known.contains(word)
    }

    pub fn pos_of(&self, word: &str) -> BTreeSet<char> {
        self.pos.get(word).cloned().unwrap_or_default()
    }

    pub fn sense_count(&self, word: &str, pos: char) -> u32 {
        *self.senses.get(&(word.to_string(), pos)).unwrap_or(&0)
    }

    pub fn zipf(&self, word: &str) -> Option<f64> {
        self.zipf.get(word).copied()
    }

    fn load_wordnet_index(&mut self, path: &str, pos: char) -> Result<(), String> {
        let text = read(path)?;
        for line in text.lines() {
            // license header lines start with a space
            if line.starts_with(' ') {
                continue;
            }
            let mut fields = line.split_whitespace();
            let (Some(lemma), Some(_pos), Some(cnt)) =
                (fields.next(), fields.next(), fields.next())
            else {
                continue;
            };
            if lemma.contains('_') {
                continue; // multi-word entries are out of scope
            }
            let senses: u32 = cnt.parse().map_err(|_| format!("{path}: bad line: {line}"))?;
            self.pos.entry(lemma.to_string()).or_default().insert(pos);
            self.senses.insert((lemma.to_string(), pos), senses);
            self.known.insert(lemma.to_string());
        }
        Ok(())
    }

    fn load_mobypos(&mut self, path: &str) -> Result<(), String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
        // latin-1-ish; lossy is fine, we only keep ascii words
        let text = String::from_utf8_lossy(&bytes);
        for line in text.lines() {
            let line = line.trim_end_matches('\r');
            let Some((word, codes)) = line.split_once('\\') else {
                continue;
            };
            if !word.chars().all(|c| c.is_ascii_lowercase()) {
                continue; // skip phrases, capitalized entries, non-ascii
            }
            self.known.insert(word.to_string());
            for code in codes.chars() {
                let pos = match code {
                    'N' | 'p' | 'h' => 'n',
                    'V' | 't' | 'i' => 'v',
                    'A' => 'a',
                    'v' => 'r',
                    _ => continue,
                };
                self.pos.entry(word.to_string()).or_default().insert(pos);
            }
        }
        Ok(())
    }

    fn load_zipf(&mut self, path: &str) -> Result<(), String> {
        let text = read(path)?;
        for line in text.lines() {
            if line.starts_with('#') {
                continue;
            }
            let Some((word, z)) = line.split_once('\t') else {
                continue;
            };
            let z: f64 = z.parse().map_err(|_| format!("{path}: bad line: {line}"))?;
            self.zipf.insert(word.to_string(), z);
            self.known.insert(word.to_string());
        }
        Ok(())
    }
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))
}
