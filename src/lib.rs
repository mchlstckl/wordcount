// enable the await! macro, async support, and the new std::Futures api.
#![feature(await_macro, async_await, futures_api)]
// only needed to manually implement a std future:
#![feature(arbitrary_self_types)]



use futures::{future, stream, Stream};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::error::Error;
use std::path::Path;
use std::str;
use tokio::prelude::*;

use walkdir::{DirEntry, WalkDir};

type GenericError = Box<dyn Error>;
type CountTuples = Vec<(String, u32)>;
type CountResult = Result<CountTuples, GenericError>;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\b[\p{L}']+\b").unwrap();
}

fn get_dir_entries(path: impl AsRef<Path>) -> Vec<DirEntry> {
    WalkDir::new(path)
        .follow_links(true)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect()
}

pub fn count_words_async(path: impl AsRef<Path>) {

    let dir_entries = get_dir_entries(path);

    // Note type information ::<_,()>
    let mut dir_stream = stream::iter_ok::<_,()>(dir_entries);

    let run = async move {
        while let Some(Ok(entry)) = await!(dir_stream.next()) {

            let data = await!(tokio::fs::read(entry.path().to_owned()));
            if let Ok(content) = str::from_utf8(&data) {
                println!("content {:?}", content);
            }
        }
    };

    tokio::run_async(run);
}

pub fn count_words_tokio(path: impl AsRef<Path>) {
    let dir_entries = get_dir_entries(path);

    let f = stream::iter_ok(dir_entries)
        .map(|entry| entry.path().to_owned())
        .and_then(tokio::fs::read)
        .map_err(|e| eprintln!("IO error: {:?}", e))
        .fold(HashMap::new(), |mut counts, data| {
            if let Ok(content) = str::from_utf8(&data) {
                for mat in RE.find_iter(&content) {
                    let word = &content[mat.start()..mat.end()];
                    let count = counts.entry(word.to_owned()).or_insert(0u32);
                    *count += 1;
                }
            }
            future::ok(counts)
        })
        .map(|counts| {
            let mut count_vec: Vec<_> = counts.into_iter().collect();
            count_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            for (word, count) in count_vec {
                println!("{:5} {}", count, word);
            }
        });

    tokio::run(f);
}

pub fn count_words_single(path: impl AsRef<Path>) -> CountResult {
    let dir_entries = get_dir_entries(path);

    let mut counts = HashMap::new();

    for entry in dir_entries {
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

pub fn count_words_par_chunks(path: impl AsRef<Path>) -> CountResult {
    let dir_entries = get_dir_entries(path);

    let counts = dir_entries
        .par_chunks(200)
        .map(|es| {
            let mut counts = HashMap::new();
            for e in es {
                if let Ok(content) = std::fs::read_to_string(e.path()) {
                    for mat in RE.find_iter(&content) {
                        let word = &content[mat.start()..mat.end()];
                        let count = counts.entry(word.to_owned()).or_insert(0u32);
                        *count += 1;
                    }
                }
            }
            counts
        })
        .reduce(HashMap::new, |a, b| {
            let mut r = HashMap::new();
            for (word, count) in a.into_iter().chain(b) {
                let entry = r.entry(word.to_owned()).or_insert(0u32);
                *entry += count
            }
            r
        });

    let mut count_vec: Vec<_> = counts.into_iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    Ok(count_vec)
}
