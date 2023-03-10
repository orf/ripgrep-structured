mod csv;
mod json;
mod prefixed;
mod tar;

use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, Sink, SinkMatch};

use std::io;
use std::io::{BufReader, BufWriter, StdinLock};

use thiserror::Error;

pub use crate::formats::csv::CSVFormat;
pub use crate::formats::tar::TarFormat;
pub use json::JSONFormat;
pub use prefixed::PrefixedFormat;

#[derive(Error, Debug)]
pub enum ExtractionError {
    #[error("Cannot deserialize input")]
    DeserializationError(#[from] io::Error),
    #[error("{0}")]
    CannotMatch(&'static str),
}

fn is_match(searcher: &mut Searcher, matcher: &RegexMatcher, slice: &[u8]) -> bool {
    let mut sink = OutputSink::default();
    searcher.search_slice(matcher, slice, &mut sink).unwrap();
    sink.matched
}

pub trait FormatMatcher {
    fn drive(
        &self,
        reader: BufReader<StdinLock>,
        writer: BufWriter<io::StdoutLock>,
        searcher: Searcher,
        matcher: RegexMatcher,
    );
}

#[derive(Default)]
struct OutputSink {
    matched: bool,
}

impl Sink for OutputSink {
    type Error = io::Error;

    fn matched(&mut self, _searcher: &Searcher, _mat: &SinkMatch<'_>) -> Result<bool, Self::Error> {
        self.matched = true;
        Ok(true)
    }
}
