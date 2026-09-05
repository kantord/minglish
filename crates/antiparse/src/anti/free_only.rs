use super::Repair;

#[derive(Debug, Clone)]
pub struct FreeOnlyMatch {
    pub verb: String,
}

impl FreeOnlyMatch {
    /// Never a single fix: the whole reason ADR 0047 bans this position is
    /// that "only" here is genuinely scope-ambiguous between the subject
    /// and the object. Two candidate repairs, never auto-picked.
    pub fn repair(&self) -> Repair {
        Repair::Menu(vec![
            format!("move \"only\" before the subject (scopes the subject)"),
            format!("move \"only\" before the object of \"{}\" (scopes the object)", self.verb),
        ])
    }
}
