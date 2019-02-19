// cargo clippy : rustup component add clippy
// cargo fmt    : rustup component add rustfmt
// on fail: rustup toolchain remove stable && rustup toolchain add stable
// cargo run --bin wc -- ./benches/big.txt
use std::error::Error;
use structopt::StructOpt;

use wordcount;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::from_args();

    let counts = wordcount::count_words(&args.path)?;

    // https://rust-lang-nursery.github.io/cli-wg/tutorial/output.html
    for (word, count) in counts {
        println!("{:5} {}", count, word);
    }

    Ok(())
}