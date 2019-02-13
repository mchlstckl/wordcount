use std::collections::HashMap;
use std::error::Error;
use structopt::StructOpt;
use regex::Regex;


pub fn count_words(path: &std::path::PathBuf) -> Result<Vec<(String, u32)>, Box<dyn std::error::Error>> {
    let mut word_counts = HashMap::new();
    let re = Regex::new(r"\b[\p{L}']+\b").unwrap();
    let content = std::fs::read_to_string(path)?;

    for mat in re.find_iter(&content) {
        let word = &content[mat.start()..mat.end()];
        let count = word_counts.entry(word).or_insert(0u32);
        *count += 1;
    }

    let mut counts: Vec<(String, u32)> = vec!();
    for (word, count) in word_counts {
        counts.push((word.to_owned(), count));
    }

    counts.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    Ok(counts)
}
