
// vim: shiftwidth=2

use crate::keys::Layout;
use std::fs::OpenOptions;

fn convert_io_error<T>(whats_happening: &str, res: Result<T, std::io::Error>) -> Result<T, String> {
  match res {
    Ok(t) => Ok(t),
    Err(e) => Err(format!("Error {}: {}", whats_happening, e))
  }
}

fn convert_json_error<T>(whats_happening: &str, res: Result<T, serde_json::Error>) -> Result<T, String> {
  match res {
    Ok(t) => Ok(t),
    Err(e) => Err(format!("Error {}: {}", whats_happening, e))
  }
}

pub fn load_layout_from_file(path: &str) -> Result<Layout, String> {
  let file_in =
    convert_io_error(
      format!("reading {}", path).as_str(),
      OpenOptions::new()
        .truncate(false).read(true).create(false).write(false)
        .open(path)
    )?;
      
  let buf_reader = std::io::BufReader::new(file_in);
  
  crate::fancy_layout_interpreting::convert(
    &crate::layout_parsing_formatting::parse_layout_from_json(
      &convert_json_error(
        format!("parsing {}", path).as_str(),
        serde_json::from_reader(buf_reader)
      )?
    )?
  )
}

#[cfg(test)]
mod tests {
  use crate::keys::Layout;
  
  #[test]
  fn load_test_1() {
    let _layout: Layout = serde_json::from_str(r#"
      {
        "mappings": [
          { "from": [ "CAPSLOCK" ], "to": [] },
          { "from": [ "CAPSLOCK", "Q" ], "to": [ "ESC" ] }
        ],
        "no_repeat_keys": []
      }
    "#).unwrap();
  }
  
  #[test]
  fn load_test_2() {
    let _layout: Layout = serde_json::from_str(r#"
      {
        "mappings": [
          { "from": [ "CAPSLOCK" ], "to": [] },
          { "from": [ "CAPSLOCK", "Q" ], "to": [ "ESC" ] }
        ]
      }
    "#).unwrap();
  }
  
  #[test]
  fn load_test_3() {
    let _layout: Layout = serde_json::from_str(r#"
      {
        "mappings": [
          { "from": [ "CAPSLOCK" ], "to": [] },
          { "from": [ "CAPSLOCK", "Q" ], "to": [ "ESC" ] }
        ]
      }
    "#).unwrap();
  }
  
  #[test]
  fn load_test_4() {
    let _layout: Layout = serde_json::from_str(r#"
      {
        "mappings": [
          { "from": [ "SEMICOLON" ], "to": [ "S" ], "repeat": { "Special": { "keys": ["F21"], "delay_ms": 180, "interval_ms": 30 } } }
        ]
      }
    "#).unwrap();
  }
}

