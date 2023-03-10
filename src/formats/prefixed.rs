use crate::formats::{is_match, FormatMatcher};
use std::io;

use grep::regex::RegexMatcher;
use grep::searcher::Searcher;
use std::io::{Read, Write};

use std::io::{BufRead, BufReader, BufWriter, StdinLock};

pub struct PrefixedFormat {}

impl PrefixedFormat {
    pub fn new() -> Self {
        Self {}
    }
}

fn read_until_tab<'a>(vec: &'a mut Vec<u8>, reader: &mut BufReader<StdinLock>) -> Option<&'a [u8]> {
    match reader.read_until(b'\t', vec) {
        Ok(k) => Some(&vec[..k]),
        Err(_) => None,
    }
}

impl FormatMatcher for PrefixedFormat {
    fn drive(
        &self,
        mut reader: BufReader<StdinLock>,
        mut writer: BufWriter<io::StdoutLock>,
        mut searcher: Searcher,
        matcher: RegexMatcher,
    ) {
        let mut name_buf = Vec::with_capacity(1024);
        let mut size_buf = Vec::with_capacity(1024);
        let mut contents_buf = Vec::with_capacity(1024);
        loop {
            match (
                read_until_tab(&mut name_buf, &mut reader),
                read_until_tab(&mut size_buf, &mut reader),
            ) {
                (Some(name), Some(length)) if !name.is_empty() && !length.is_empty() => {
                    let name = std::str::from_utf8(name).unwrap().trim();
                    let length = std::str::from_utf8(length).unwrap().trim();
                    let length: u64 = length.parse().unwrap();
                    io::copy(&mut reader.by_ref().take(length), &mut contents_buf).unwrap();
                    if is_match(&mut searcher, &matcher, &contents_buf) {
                        writeln!(&mut writer, "{name}").unwrap();
                    }
                }
                _ => return,
            }

            name_buf.clear();
            size_buf.clear();
            contents_buf.clear();
        }
    }
}
