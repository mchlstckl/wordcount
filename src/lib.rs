use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

fn find_words(content: &str) -> HashMap<String, u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b[\p{L}']+\b").unwrap();
    }
    let mut word_counts = HashMap::new();
    for mat in RE.find_iter(content) {
        let word = &content[mat.start()..mat.end()];
        let count = word_counts.entry(word.to_owned()).or_insert(0u32);
        *count += 1;
    }
    word_counts
}

pub fn count_words(path: impl AsRef<Path>) -> Result<Vec<(String, u32)>, Box<dyn Error>> {
    let content = std::fs::read_to_string(path.as_ref())?;
    let count_map = find_words(&content);
    let mut count_vec: Vec<_> = count_map.into_iter().collect();   
    count_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    Ok(count_vec)
}
