// cargo clippy : rustup component add clippy
// cargo fmt    : rustup component add rustfmt
// on fail: rustup toolchain remove stable && rustup toolchain add stable
// cargo run ./src/main.rs
// cargo build --release
// ./target/release/wordcount src/main.rs
use std::collections::HashMap;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)?;
    let mut word_counts = HashMap::new();

    for line in content.lines() {
        let tokens = line.split_whitespace();
        for token in tokens {
            *word_counts.entry(token).or_insert(0) += 1;
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
