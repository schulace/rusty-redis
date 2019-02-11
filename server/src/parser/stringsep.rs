pub struct StringTokenizer<'a> {
  buffer: &'a str,
}

impl<'a> StringTokenizer<'a> {
  pub fn new(s: &'a str) -> StringTokenizer<'a> {
    StringTokenizer { buffer: s }
  }
}

impl<'a> Iterator for StringTokenizer<'a> {
  type Item = &'a str;
  fn next(&mut self) -> Option<Self::Item> {
    self.buffer = self.buffer.trim_start();
    let first_char = self.buffer.chars().next();
    match first_char {
      Some('"') => {
        let (_head, tail) = self.buffer.split_at(1);
        if let Some(next_quote) = tail.find('"') {
          self.buffer = &tail[next_quote + 1..];
          Some(&tail[0..next_quote])
        } else {
          None
        }
      },
      Some(_) => {
        if let Some(index) = self.buffer.find(|c:char| c.is_whitespace()) {
          let (head, tail) = self.buffer.split_at(index);
          self.buffer = tail;
          Some(head)
        } else { //no spaces left in the string, so we'll return the buffer
          let tmp = self.buffer;
          self.buffer = &self.buffer[self.buffer.len().. self.buffer.len()];
          Some(tmp)
        }

      },
      None => None,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_strsep() {
    let s = String::from(" get the \"big thing\" done           ");
    let v = vec!["get", "the", "big thing", "done"];
    let tv = StringTokenizer::new(&s).collect::<Vec<_>>();
    assert_eq!(v, tv);
  }
}
