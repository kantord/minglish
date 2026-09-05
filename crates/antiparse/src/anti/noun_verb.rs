use super::Repair;

#[derive(Debug, Clone)]
pub struct NounVerbMatch {
    pub intro: Option<String>,
    pub word: String,
    pub next_det: Option<String>,
}

impl NounVerbMatch {
    /// Always a single deterministic repair when the lemma has a known
    /// VERB redirect (the substitution data already lives in the seed —
    /// ADR 0008's per-sense-synonym policy is exactly what makes this
    /// case fully mechanical, unlike bare_coord's ellipsis case).
    pub fn repair(&self, verb_redirect: Option<&str>) -> Repair {
        match verb_redirect {
            Some(v) => Repair::Single(format!(
                "{}{}",
                self.intro.as_ref().map(|i| format!("{i} ")).unwrap_or_default(),
                v
            )),
            None => Repair::None(format!(
                "\"{}\" has no known verb-sense redirect — restructure the sentence",
                self.word
            )),
        }
    }
}
