use std::collections::HashMap;
use std::error::Error;
use std::io;

mod parser;

fn main() -> Result<(), Box<dyn Error>> {
  let mut map: HashMap<String, String> = HashMap::new();
  loop {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    println!("{}", handle_query(s, &mut map));
  }
}

fn handle_query(query_str: String, map: &mut HashMap<String, String>) -> String {
  use parser::Command::*;
  if let Ok(query) = parser::parse_query(&query_str) {
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
        }});
        let k = ent.key().clone();
        map.get(&k).unwrap_or(&"key not in map".to_string()).to_owned()
      },
      DECR(k) => {
        let ent = map.entry(k).and_modify(|st: &mut String| {
          if let Ok(i) = st.parse::<i128>() {
            *st = (i - 1).to_string();
          }
        });
        let k = ent.key().clone();
        map.get(&k).unwrap_or(&"key not in map".to_string()).to_owned()
      }
    }
  } else {
    "Couldn't parse query".to_string()
  }
}
