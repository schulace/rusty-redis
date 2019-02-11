use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener};

mod parser;

fn main() {
  let mut map: HashMap<String, String> = HashMap::new();
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  for stream in listener.incoming() {
    let mut buffer = [0; 512];
    let mut stream = stream.unwrap();
    stream.read(&mut buffer).unwrap();
    eprintln!(
      "read \"{}\" from the stream",
      std::str::from_utf8(&buffer).unwrap()
    );
    let s = handle_query(std::str::from_utf8(&buffer).unwrap(), &mut map);
    stream.write_all(&s.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
    eprintln!("wrote {} to the socket", &s);
  }
}

fn handle_query(query_str: &str, map: &mut HashMap<String, String>) -> String {
  use parser::Command::*;
  if let Ok(query) = parser::parse_query(query_str) {
    match query {
      GET(k) => map.get(&k).unwrap_or(&"Not in map".to_string()).to_owned(),
      SET(k, v) => map.insert(k, v).map_or_else(
        || "sucessfully inserted".to_string(),
        |old_value| format!("inserted into the map and replaced {}", old_value),
      ),
      INCR(k) => {
        let ent = map.entry(k).and_modify(|st: &mut String| {
          if let Ok(i) = st.parse::<i128>() {
            *st = (i + 1).to_string();
          }
        });
        let k = ent.key().clone();
        map
          .get(&k)
          .unwrap_or(&"key not in map".to_string())
          .to_owned()
      }
      DECR(k) => {
        let ent = map.entry(k).and_modify(|st: &mut String| {
          if let Ok(i) = st.parse::<i128>() {
            *st = (i - 1).to_string();
          }
        });
        let k = ent.key().clone();
        map
          .get(&k)
          .unwrap_or(&"key not in map".to_string())
          .to_owned()
      }
    }
  } else {
    "Couldn't parse query".to_string()
  }
}
