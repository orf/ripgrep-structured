mod formats;

use crate::formats::{CSVFormat, FormatMatcher, JSONFormat, PrefixedFormat, TarFormat};
use clap::{Parser, Subcommand};
use grep::regex::RegexMatcherBuilder;
use grep::searcher::{BinaryDetection, SearcherBuilder};

use std::io;
use std::io::{BufReader, BufWriter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg()]
    pattern: String,
    #[command(subcommand)]
    format: Format,
}

#[derive(Subcommand, Debug)]
enum Format {
    Json {
        #[arg()]
        key: String,
    },
    Csv {
        #[arg()]
        column: usize,
        #[arg(default_value=",", short, long)]
        delimiter: char,
    },
    Prefixed,
    Tar,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let matcher = RegexMatcherBuilder::new()
        .multi_line(true)
        .build(&args.pattern)?;
    let searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(false)
        .multi_line(true)
        .build();

    let reader = BufReader::new(io::stdin().lock());
    let writer = BufWriter::new(io::stdout().lock());

    let format: Box<dyn FormatMatcher> = match args.format {
        Format::Json { key } => Box::new(JSONFormat::new(key)),
        Format::Csv { column, delimiter } => Box::new(CSVFormat::new(column, delimiter)),
        Format::Prefixed => Box::new(PrefixedFormat::new()),
        Format::Tar => Box::new(TarFormat::new()),
    };

    format.drive(reader, writer, searcher, matcher);

    Ok(())
}
