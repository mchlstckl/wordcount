use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
// use std::collections::HashMap;
use hashbrown::HashMap;
use std::error::Error;
use walkdir::WalkDir;
use rayon::prelude::*;

pub fn count_words(path: impl AsRef<Path>) -> Result<Vec<(String, u32)>, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b[\p{L}']+\b").unwrap();
    }

    let walker: Vec<_> = WalkDir::new(path)
        .follow_links(true)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let mut counts = HashMap::new();

    for entry in walker {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            for mat in RE.find_iter(&content) {
                let word = &content[mat.start()..mat.end()];
                let count = counts.entry(word.to_owned()).or_insert(0u32);
                *count += 1;
            }
        }
    }

    let mut count_vec: Vec<_> = counts.into_iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    Ok(count_vec)
}
