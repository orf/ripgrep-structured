use crate::formats::{is_match, FormatMatcher};
use std::io;

use grep::regex::RegexMatcher;
use grep::searcher::Searcher;
use std::io::{BufReader, BufWriter, StdinLock};

pub struct CSVFormat {
    index: usize,
    delimiter: char
}

impl CSVFormat {
    pub fn new(index: usize, delimiter: char) -> Self {
        Self { index, delimiter }
    }
}

impl FormatMatcher for CSVFormat {
    fn drive(
        &self,
        reader: BufReader<StdinLock>,
        writer: BufWriter<io::StdoutLock>,
        mut searcher: Searcher,
        matcher: RegexMatcher,
    ) {
        let mut reader = csv::ReaderBuilder::new().delimiter(self.delimiter as u8).from_reader(reader);
        // let mut writer = csv::Writer::from_writer(writer);
        let mut writer = csv::WriterBuilder::new().from_writer(writer);
        for record in reader.byte_records().flatten() {
            let field = record.get(self.index).unwrap();
            if is_match(&mut searcher, &matcher, field) {
                writer.write_byte_record(&record).unwrap();
            }
        }
    }
}
