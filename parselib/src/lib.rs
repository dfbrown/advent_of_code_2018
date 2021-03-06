use std::error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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

#[derive(Debug, Clone)]
pub struct GenericError {
    message: String,
}

impl GenericError {
    pub fn new(message: &str) -> GenericError {
        GenericError {
            message: String::from(message)
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl error::Error for GenericError {
}

pub fn parse_lines<V: FromStr, P: AsRef<Path>>(path: P) -> Result<Vec<V>, ParseError> {
    let input_str = load_text_file(path)?;
    let mut result = Vec::new();
    for line in input_str.lines() {
        result.push(line.parse().map_err(|_| ParseError)?);
    }

    return Ok(result);
}

pub fn parse_lines_fn<V, E, P, F>(path: P, mut func: F) -> Result<Vec<V>, ParseError>
where
    P: AsRef<Path>,
    F: FnMut(&str) -> Result<V, E>,
{
    let input_str = load_text_file(path)?;
    let mut result = Vec::new();
    for line in input_str.lines() {
        result.push(func(line).map_err(|_| ParseError)?);
    }
    return Ok(result);
}

pub fn load_text_file<P: AsRef<Path>>(path: P) -> Result<String, ParseError> {
    let mut f = File::open(path).map_err(|_| ParseError)?;

    let mut input_str = String::new();
    f.read_to_string(&mut input_str).map_err(|_| ParseError)?;
    return Ok(input_str);
}

pub fn load_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ParseError> {
    let mut f = File::open(path).map_err(|_| ParseError)?;

    let mut result = Vec::new();
    f.read_to_end(&mut result).map_err(|_| ParseError)?;

    return Ok(result);
}
