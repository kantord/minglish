use super::Repair;

pub fn join_np(det: &Option<String>, adj: &Option<String>, noun: &str) -> String {
    match (det, adj) {
        (Some(d), Some(a)) => format!("{d} {a} {noun}"),
        (Some(d), None) => format!("{d} {noun}"),
        (None, Some(a)) => format!("{a} {noun}"),
        (None, None) => noun.to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct ClauseObjectMatch {
    pub outer_verb: String,
    pub inner_subject: String,
    pub inner_verb: String,
}

impl ClauseObjectMatch {
    /// Never a mechanical fix: which sentence to split into, and what (if
    /// anything) to name the embedded fact, are decisions only the writer
    /// can make — the same "genuinely missing information" case as
    /// `bare_coord`'s elliptical conjunct (see crate docs).
    pub fn repair(&self) -> Repair {
        Repair::None(format!(
            "\"{} {} {} …\" — \"{}\" starts a second clause after \"{}\"; a clause cannot be \
             the object of a verb. State the fact in its own sentence, or give it a Name and \
             use the Name here",
            self.outer_verb, self.inner_subject, self.inner_verb, self.inner_subject, self.outer_verb
        ))
    }
}
