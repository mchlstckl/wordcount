// cargo clippy : rustup component add clippy
// cargo fmt    : rustup component add rustfmt
// on fail: rustup toolchain remove stable && rustup toolchain add stable
// cargo run --bin wc -- ./benches/big.txt

// cargo install cargo-expand
// cargo expand
use std::error::Error;
use std::time::Instant;
use structopt::StructOpt;

use wordcount;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    variant: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let now = Instant::now();

    let counts = match args.variant.as_ref() {
        "par_chunks" => {
            println!("Running par_chunks variant");
            wordcount::count_words_par_chunks(&args.path)?
        }
        "queue" => {
            println!("Running queue variant");
            wordcount::count_words_queue(&args.path)?
        }
        "channel" => {
            println!("Running channel variant");
            wordcount::count_words_channel(&args.path)?
        }
        "tokio" => {
            println!("Running tokio variant");
            wordcount::count_words_tokio(&args.path);
            vec![("empty".to_owned(), 0)]
        }
        _ => {
            println!("Running single variant");
            wordcount::count_words_single(&args.path)?
        }
    };

    let count_time = now.elapsed();

    // https://rust-lang-nursery.github.io/cli-wg/tutorial/output.html
    for (word, count) in counts {
        println!("{:5} {}", count, word);
    }

    let total_time = now.elapsed();

    println!("Count time: {:?}, Total time: {:?}", count_time, total_time);

    Ok(())
}
