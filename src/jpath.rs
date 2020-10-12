use jmespath::{JmespathError, Variable};
use rutie::{AnyObject};
use snafu::{ResultExt, Snafu};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Snafu)]
pub enum Error {
  #[snafu(display("There was an error compiling the pattern at {} {}", pattern, source))]
  CompileError {source: JmespathError, pattern: String},
  #[snafu(display("There was an error parsing JSON at {}", source))]
  ParseError {source: serde_json::Error},
  #[snafu(display("There was an searching for the pattern at {}", source))]
  SearchError {source: JmespathError},
  #[snafu(display("There was a problem serializing variale to AnyObject {}", source))]
  SerializationError {source: crate::serde_anyobject::Error},
  #[snafu(display("There was a problem opening the file {} at {}", filename, source))]
  FileOpenError {source: std::io::Error, filename: String},
  #[snafu(display("There was a problem reading the file {} at {}", filename, source))]
  FileReadError {source: std::io::Error, filename: String}
}

type Result<T> = std::result::Result<T, Error>;

pub fn extract_string<'a, 'b>(pat: &'a str, input: &'b str) -> Result<AnyObject> {
  let expr = jmespath::compile(pat).context(CompileError{pattern: pat.to_string()})?;
  let json = serde_json::from_str::<Variable>(input).context(ParseError)?;
  let val = expr.search(json).context(SearchError)?;
  // TODO: remove this and replace it with rutie-serde
  let obj = crate::serde_anyobject::new_ruby_object(val).context(SerializationError)?;
  Ok(obj)
}
pub fn extract_file<'a, 'b>(pat: &'a str, filename: &'b str) -> Result<AnyObject> {
  let mut contents = String::new();
  let mut json_file= File::open(filename).context(FileOpenError{filename: filename.to_string()})?;
  json_file.read_to_string(&mut contents).context(FileReadError{filename: filename.to_string()})?;
  let expr = jmespath::compile(pat).context(CompileError{pattern: pat.to_string()})?;
  let json: Variable = serde_json::from_str(&contents).context(ParseError)?;
  let val = expr.search(json).context(SearchError)?;
  // TODO: remove this and replace it with rutie-serde
  let obj = crate::serde_anyobject::new_ruby_object(val).context(SerializationError)?;
  Ok(obj)
}