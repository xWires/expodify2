use std::{fs, process};
use std::path::PathBuf;
use clap::Parser;
use expodify2::Extractor;

#[derive(Parser)]
struct Args {
    source: PathBuf,
    dest: PathBuf,

    /// Only show what would have been done, without modifying any files
    #[clap(long)]
    dry_run: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let extractor_builder = Extractor::builder()
        .source(&args.source)
        .destination(&args.dest);

    if !fs::exists(&args.source).unwrap() {
        eprintln!("Source does not exist: {}", args.source.display());
        process::exit(1);
    }

    if !fs::exists(&args.dest).unwrap() {
        eprintln!("Destination does not exist: {}", args.dest.display());
        process::exit(1);
    }

    if args.dry_run {
        extractor_builder
            .dry_run()
            .build()
            .unwrap()
            .extract()
            .unwrap();
    } else {
        extractor_builder
            .build()
            .unwrap()
            .extract()
            .unwrap();
    }
}
