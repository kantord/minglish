use super::Repair;

#[derive(Debug, Clone)]
pub struct Conjunct {
    pub det: Option<String>,
    pub adj: Option<String>,
    pub noun: String,
}

#[derive(Debug, Clone)]
pub struct BareCoordMatch {
    pub verb: String,
    pub left: Conjunct,
    pub conj: String,
    pub right: Conjunct,
}

impl BareCoordMatch {
    /// Repair category (see crate docs): a single deterministic fix exists
    /// only when both conjuncts already carry their own determiner — the
    /// colon-list construction (ADR 0041) always accepts that shape. When
    /// the right conjunct is elliptical (no determiner, "old files and
    /// reports"), no repair can be invented: the missing determiner/
    /// modifier information genuinely isn't recoverable from the sentence.
    pub fn repair(&self) -> Repair {
        if self.left.det.is_some() && self.right.det.is_some() {
            Repair::Single(format!(
                "{}: {}{}{} {} {}{}{}",
                self.verb,
                self.left.det.as_deref().unwrap_or(""),
                fmt_space(&self.left.adj),
                self.left.noun,
                self.conj,
                self.right.det.as_deref().unwrap_or(""),
                fmt_space(&self.right.adj),
                self.right.noun,
            ))
        } else {
            Repair::None(
                "the second conjunct has no determiner of its own — name it \
                 explicitly (\"the old file and the old report\", not \"the \
                 old file and report\"); the missing determiner/modifier is \
                 not recoverable from the sentence alone (ADR 0004)"
                    .to_string(),
            )
        }
    }
}

fn fmt_space(a: &Option<String>) -> String {
    match a {
        Some(w) => format!("{w} "),
        None => String::new(),
    }
}
