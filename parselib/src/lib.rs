use std::fmt;
use std::fs::File;
use std::path::Path;
use std::error;
use std::io::Read;
use std::str::FromStr;


#[derive(Debug, Clone, Copy)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing input")
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Error parsing input"
    }
}

pub fn parse_lines<V: FromStr, P: AsRef<Path>>(path: P) -> Result<Vec<V>, ParseError> {
    let mut f = File::open(path).map_err(|_| ParseError)?;

    let mut input_str = String::new();
    f.read_to_string(&mut input_str).map_err(|_| ParseError)?;
    let mut result = Vec::new();
    for line in input_str.lines() {
        result.push(line.parse().map_err(|_| ParseError)?);
    }

    return Ok(result);
}
