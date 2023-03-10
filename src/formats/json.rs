use crate::formats::{is_match, ExtractionError, FormatMatcher};
use std::io;

use grep::regex::RegexMatcher;
use grep::searcher::Searcher;
use std::io::Write;

use serde_json::Value;

use std::io::{BufRead, BufReader, BufWriter, StdinLock};

pub struct JSONFormat {
    key: String,
}

impl JSONFormat {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn extract_target(&self, line: &str) -> Result<String, ExtractionError> {
        let value: Value = serde_json::from_str(line)
            .map_err(|e| ExtractionError::DeserializationError(e.into()))?;
        if let Value::Object(mut map) = value {
            let key = map.remove(&self.key).ok_or(ExtractionError::CannotMatch(
                "Key not present in input JSON",
            ))?;
            if let Value::String(s) = key {
                Ok(s)
            } else {
                Err(ExtractionError::CannotMatch("Input key is not a string"))
            }
        } else {
            Err(ExtractionError::CannotMatch("Input line is not an object"))
        }
    }
}

impl FormatMatcher for JSONFormat {
    fn drive(
        &self,
        reader: BufReader<StdinLock>,
        mut writer: BufWriter<io::StdoutLock>,
        mut searcher: Searcher,
        matcher: RegexMatcher,
    ) {
        for line in reader.lines().flatten() {
            if let Ok(target) = self.extract_target(&line) {
                if is_match(&mut searcher, &matcher, &target.into_bytes()) {
                    writeln!(writer, "{line}").unwrap();
                }
            }
        }
    }
}
