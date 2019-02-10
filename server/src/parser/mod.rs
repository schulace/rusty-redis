use std::fmt::{Display, Formatter, self};
use std::error::Error;


#[derive(PartialEq, Eq, Debug)]
pub enum Command {
  INCR(String),
  DECR(String),
  SET(String, String),
  GET(String)
}

#[derive(PartialEq, Eq, Debug)]
pub enum QueryError {
  ParseFailure
}

impl Display for QueryError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "ParseFailure")
  }
}

impl Error for QueryError {}

pub fn parse_query(query: &str) -> Result<Command, QueryError> {
  use self::Command::*;
  let mut iter = query.split_whitespace();
  let first_word = iter.next().ok_or(QueryError::ParseFailure)?;
  let second_word = iter.next().ok_or(QueryError::ParseFailure)?.to_string();
  let third_word = iter.next();
  match first_word {
    "INCR" => Ok(INCR(second_word)),
    "DECR" => Ok(DECR(second_word)),
    "GET"  => Ok(GET(second_word)),
    "SET"  => {
      Ok(SET(second_word, third_word.ok_or(QueryError::ParseFailure)?.to_string()))
    }
    _ => Err(QueryError::ParseFailure)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn test_parser_basic() {
    use self::Command::*;
    assert_eq!(parse_query("GET bunnies"), Ok(GET("bunnies".to_string())));
    assert_eq!(parse_query("SET bunnies 15"), Ok(SET("bunnies".to_string(), 15.to_string())));
  }
}