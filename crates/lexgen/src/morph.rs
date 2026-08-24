//! Regular English morphology. Deliberately small: irregulars belong in the
//! seed entry's `forms` override, and a wrong guess here is caught loudly by
//! the unattested-form lint (e.g. generated "runned" is attested nowhere).

pub fn pluralize(noun: &str) -> String {
    sibilant_s(noun)
}

pub fn third_singular(verb: &str) -> String {
    sibilant_s(verb)
}

pub fn past(verb: &str) -> String {
    if verb.ends_with('e') {
        return format!("{verb}d");
    }
    if let Some(stem) = strip_consonant_y(verb) {
        return format!("{stem}ied");
    }
    format!("{}ed", doubled(verb))
}

pub fn gerund(verb: &str) -> String {
    if let Some(stem) = verb.strip_suffix("ie") {
        return format!("{stem}ying");
    }
    if verb.ends_with('e') && !verb.ends_with("ee") {
        return format!("{}ing", &verb[..verb.len() - 1]);
    }
    format!("{}ing", doubled(verb))
}

/// -s / -es / -ies, shared by noun plurals and 3rd-person singular.
fn sibilant_s(word: &str) -> String {
    if let Some(stem) = strip_consonant_y(word) {
        return format!("{stem}ies");
    }
    let es = ["s", "x", "z", "ch", "sh", "o"];
    if es.iter().any(|suf| word.ends_with(suf)) {
        return format!("{word}es");
    }
    format!("{word}s")
}

/// For "retry" → Some("retr"); None when the y follows a vowel ("play").
fn strip_consonant_y(word: &str) -> Option<&str> {
    let stem = word.strip_suffix('y')?;
    let last = stem.chars().last()?;
    (!is_vowel(last)).then_some(stem)
}

/// Final-consonant doubling for short CVC words ("stop" → "stopp").
/// Stress can't be detected, so this only fires for words ≤ 4 letters;
/// longer CVC words that need doubling take a `forms` override.
fn doubled(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    if word.len() <= 4 && chars.len() >= 3 {
        let [a, b, c] = [
            chars[chars.len() - 3],
            chars[chars.len() - 2],
            chars[chars.len() - 1],
        ];
        if !is_vowel(a) && is_vowel(b) && !is_vowel(c) && !"wxy".contains(c) {
            return format!("{word}{c}");
        }
    }
    word.to_string()
}

fn is_vowel(c: char) -> bool {
    "aeiou".contains(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_morphology() {
        assert_eq!(pluralize("agent"), "agents");
        assert_eq!(pluralize("process"), "processes");
        assert_eq!(pluralize("query"), "queries");
        assert_eq!(third_singular("watch"), "watches");
        assert_eq!(third_singular("go"), "goes");
        assert_eq!(third_singular("retry"), "retries");
        assert_eq!(past("store"), "stored");
        assert_eq!(past("retry"), "retried");
        assert_eq!(past("stop"), "stopped");
        assert_eq!(past("start"), "started");
        assert_eq!(past("reject"), "rejected");
        assert_eq!(gerund("write"), "writing");
        assert_eq!(gerund("stop"), "stopping");
        assert_eq!(gerund("die"), "dying");
        assert_eq!(gerund("see"), "seeing");
        assert_eq!(gerund("fail"), "failing");
    }
}
