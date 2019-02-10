pub struct StringTokenizer<'a> {
  buffer: &'a str
}

impl<'a> StringTokenizer<'a> {
  pub fn new(s: &'a str) -> StringTokenizer<'a> {
    StringTokenizer{buffer: s}
  }
}

impl<'a> Iterator for StringTokenizer<'a> {
  type Item = &'a str;
  fn next(&mut self) -> Option<Self::Item> {
    self.buffer = self.buffer.trim_start();
    let first_char = self.buffer.chars().next();
    match first_char {
      Some(c) => {
        if c == '"' {
          let (head, tail) = self.buffer.split_at(1);
          let next_quote = self.buffer.find('"').unwrap();
          self.buffer = &tail[next_quote+1..];
          Some(&tail[0..next_quote])
        } else {
          None
        }
      },
      None => None
    }
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn do_testing() {

  }
}
