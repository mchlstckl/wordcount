// cargo clippy : rustup component add clippy
// cargo fmt    : rustup component add rustfmt
// on fail: rustup toolchain remove stable && rustup toolchain add stable
// cargo run ./src/main.rs
// cargo build --release
// ./target/release/wordcount src/main.rs
use std::collections::HashMap;
use std::error::Error;
use structopt::StructOpt;
use regex::Regex;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)?;
    let mut word_counts = HashMap::new();
    let re = Regex::new(r"\b[\p{L}']+\b").unwrap();

    for line in content.lines() {
        for mat in re.find_iter(line) {
            let word = &line[mat.start()..mat.end()];
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    let mut counts: Vec<_> = word_counts.iter().collect();
    counts.sort_by(|a, b| a.1.cmp(b.1).reverse());

    // https://rust-lang-nursery.github.io/cli-wg/tutorial/output.html
    for (word, count) in counts {
        println!("{:5} {}", count, word);
    }

    Ok(())
}


#[cfg(test)]
mod test {

    #[test]
    fn word_count() {
        assert_eq!(1, 2);
    }
}
