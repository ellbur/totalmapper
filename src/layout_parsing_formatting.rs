
use std::{str::FromStr, f32::consts::E};

use key_codes::KeyCode;
use serde_json::{Value, Map};
use Value::{Object, Array};
use crate::{fancy_keys::{Layout, Mapping, Modifier, FromKeys, FromKey, ToKeys, InitialToKey, TerminalToKey, Repeat}, key_codes};
use serde_json::Value as j;

pub fn parse_layout_from_json(root: &Value) -> Result<Layout, String> {
  match root {
    Object(root_values) => {
      if has_exactly_keys(root_values, &vec!["mappings"]) {
        let mappings_v = root_values.get("mappings").unwrap();
        match mappings_v {
          Array(mapping_vs) => {
            let mut mappings = Vec::new();
              
            for mapping_v in mapping_vs {
              match parse_mapping_from_json(mapping_v) {
                Ok(m) => mappings.push(m),
                Err(e) => return Err(format!("Malformed mapping {}: {}", mapping_v, e)),
              }
            }
            
            Ok(Layout {
              mappings
            })
          },
          _ => {
            Err("\"mappings\" must be an array".to_owned())
          }
        }
      }
      else {
        Err("Layout must have a single field \"mappings\"".to_owned())
      }
    },
    _ => {
      Err("Layout JSON must be an object".to_owned())
    }
  }
}

fn parse_mapping_from_json(mapping_v: &Value) -> Result<Mapping, String> {
  match mapping_v {
    Object(mapping_values) => {
      if has_at_least_keys(mapping_values, &vec!["from", "to"]) {
        let from = parse_from(mapping_values.get("from").unwrap())?;
        let to = parse_to(mapping_values.get("to").unwrap())?;
        let repeat = parse_repeat(&mapping_values.get("repeat"))?;
        let absorbing = parse_absorbing(&mapping_values.get("absorbing"))?;
        
        Ok(Mapping {
          from, to, repeat, absorbing
        })
      }
      else {
        return Err("Mapping must have \"from\" and \"to\"".to_owned())
      }
    },
    _ => {
      return Err("Each \"mapping\" must be an object".to_owned())
    }
  }
}

fn parse_from(from_v: &Value) -> Result<FromKeys, String> {
  if let Array(from_elems) = from_v {
    if from_elems.len() == 0 {
      Err("Can't map from zero keys, i.e. []".to_owned())
    }
    else {
      Ok(FromKeys {
        modifiers: parse_from_modifiers(&from_elems[0..from_elems.len()-1])?,
        key: parse_from_key(&from_elems[from_elems.len()-1])?
      })
    }
  }
  else {
    Ok(FromKeys {
      modifiers: vec![],
      key: parse_from_key(from_v)?
    })
  }
}

fn parse_from_modifiers(mod_vs: &[Value]) -> Result<Vec<Modifier>, String> {
  let mut res = vec![];
  
  for v in mod_vs.iter() {
    res.push(parse_from_modifier(v)?);
  }
    
  Ok(res)
}

fn parse_from_modifier(v: &Value) -> Result<Modifier, String> {
  if let j::String(text) = v {
    if text.starts_with("@") {
      Ok(Modifier::Alias(text.to_owned()))
    }
    else {
      Ok(Modifier::Key(parse_key_code(text)?))
    }
  }
  else {
    Err(format!("Modifier must be a string, found {}", v))
  }
}

fn parse_from_key(key_v: &Value) -> Result<FromKey, String> {
  if let j::String(text) = key_v {
    parse_from_key_text(text)
  }
  else if let j::Object(obj) = key_v {
    parse_from_key_obj(obj)
  }
  else {
    Err(format!("`from` key must be a string or an object, found {}", key_v))
  }
}

fn parse_from_row(elems: &Map<String, Value>) -> Result<FromKey, String> {
  if has_exactly_keys(elems, &vec!["row"]) {
    let row_obj = elems.get("row").unwrap();
    if let j::String(row_text) = row_obj {
      Ok(FromKey::Row(row_text.clone()))
    }
    else {
      Err(format!("`row` must be a string, found {}", row_obj))
    }
  }
  else {
    Err("Row must be specified by single key, `row`, which is the first key in theh row".to_owned())
  }
}

fn parse_from_key_text(from_text: &str) -> Result<FromKey, String> {
  Ok(FromKey::Single(parse_key_code(from_text)?))
}

fn parse_from_key_obj(obj: &Map<String, Value>) -> Result<FromKey, String> {
  if has_exactly_keys(obj, &vec!["row"]) {
    Ok(parse_from_row(obj)?)
  }
  else {
    Err(format!("Don't understand `from` object with keys {}, expected possibly key `row`",
        keys_string(obj)))
  }
}

fn parse_to(to_v: &Value) -> Result<ToKeys, String> {
  todo!()
}

fn parse_key_code(text: &str) -> Result<KeyCode, String> {
  if text.starts_with("@") {
    Err(format!("A real key was expected, but alias modifier {} was found", text))
  }
  else {
    KeyCode::from_str(&text).map_err(|_| format!("Unknown key code: {}", text))
  }
}

fn parse_modifier(text: &str) -> Result<Modifier, String> {
  if text.starts_with("@") {
    Ok(Modifier::Alias(text.to_owned()))
  }
  else {
    Ok(Modifier::Key(KeyCode::from_str(&text).map_err(|_| format!("Unknown key: {}", text))?))
  }
}

fn parse_repeat(v: &Option<&Value>) -> Result<Repeat, String> {
  todo!()
}

fn parse_absorbing(v: &Option<&Value>) -> Result<Vec<Modifier>, String> {
  todo!()
}

fn keys_string(values: &Map<String, Value>) -> String {
  let mut v1: Vec<&str> = values.keys().map(|s|s.as_str()).collect();
  v1.join(", ")
}

fn has_exactly_keys(values: &Map<String, Value>, check: &Vec<&str>) -> bool {
  let mut v1: Vec<&str> = values.keys().map(|s|s.as_str()).collect();
  let mut v2: Vec<&str> = check.iter().map(|s|*s).collect();
  v1.sort();
  v2.sort();
  v1 == v2
}

fn has_at_least_keys(values: &Map<String, Value>, check: &Vec<&str>) -> bool {
  for key in check {
    if !values.contains_key(*key) {
      return false;
    }
  }
  true
}

