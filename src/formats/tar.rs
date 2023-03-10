use crate::formats::{is_match, FormatMatcher};
use std::io;

use grep::regex::RegexMatcher;
use grep::searcher::Searcher;

use std::io::{BufReader, BufWriter, StdinLock};

pub struct TarFormat {}

impl TarFormat {
    pub fn new() -> Self {
        Self {}
    }
}

impl FormatMatcher for TarFormat {
    fn drive(
        &self,
        reader: BufReader<StdinLock>,
        writer: BufWriter<io::StdoutLock>,
        mut searcher: Searcher,
        matcher: RegexMatcher,
    ) {
        let mut archive = tar::Archive::new(reader);
        let mut writer = tar::Builder::new(writer);
        for mut entry in archive.entries().unwrap().flatten() {
            let mut data = vec![];
            io::copy(&mut entry, &mut data).unwrap();
            if is_match(&mut searcher, &matcher, &data) {
                writer.append(entry.header(), &*data).unwrap();
            }
        }
        writer.into_inner().unwrap();
    }
}
