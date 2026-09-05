pub mod bare_coord;
pub mod clause_object;
pub mod free_only;
pub mod noun_verb;

/// What an antiparser match can offer as a fix — the 3 categories found
/// while prototyping (see crate docs): a Redirect-backed mistake is always
/// `Single`; a genuinely scope-ambiguous construction is a `Menu`; a
/// construction banned because information is missing from the sentence
/// itself is `None`, with an explanation but no guess.
#[derive(Debug, Clone)]
pub enum Repair {
    Single(String),
    Menu(Vec<String>),
    None(String),
}
