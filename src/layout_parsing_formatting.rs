
use std::str::FromStr;

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
        let from = parse_from(mapping_values.get("from").unwrap());
        let to = parse_to(mapping_values.get("to").unwrap());
        
        todo!()
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
  if let j::String(from_text) = from_v {
    Ok(FromKeys {
      modifiers: vec![],
      key: parse_from_key_text(from_text)?
    })
  }
  else if Object(from_obj) = from_v {
    if has_exactly_keys(from_obj, &vec!["row"]) {
      Ok(FromKeys {
        modifiers: vec![],
        key: parse_from_row(from_obj)
      })
    }
    else {
      Err(format!("Don't understand `from` object with keys {}, expected possibly key `row`",
          keys_string(from_obj)))
    }
  }
  else if Array(from_elems) = from_v {
    todo!()
  }
  else {
    todo!()
  }
}

fn parse_from_row(elems: Map<String, Value>) -> Result<FromKey, String> {
  todo!()
}

fn parse_from_key_text(from_text: &str) -> Result<FromKey, String> {
  Ok(FromKey::Single(parse_key_code(from_text)?))
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

fn parse_alias_definition(values: &Map<String, Value>) -> Result<Mapping, String> {
  if !has_exactly_keys(values, &vec!["from", "to"]) {
    return Err("Alias definition must include exactly properties \"from\" and \"to\"".to_owned());
  }
  
  let from = values.get("from").unwrap();
  let to = values.get("to").unwrap();
  
  Ok(Mapping::AliasDefinition(AliasDefinitionMapping {
    from_modifiers: {
      if let j::String(from_text) = from {
        vec![parse_modifier(from_text)?]
      }
      else if let Array(from_items) = from {
        from_items.iter().map(|item| {
          if let j::String(text) = item {
            parse_modifier(text)
          }
          else {
            Err("Items of \"from\" in alias definition must be strings".to_owned())
          }
        }).collect::<Result<_, _>>()?
      }
      else {
        return Err("\"from\" for an alias definition must be either a string (single key) or array (multiple keys)".to_owned())
      }
    },
    to_also_keys: {
      if let j::String(_) = to {
        vec![]
      }
      else if let Array(items) = to {
        items.iter().take(items.len()-1).map(|item| -> Result<KeyCode, String> {
          if let j::String(text) = item {
            KeyCode::from_str(&text).map_err(|_| format!("Unknown key code: {}", text))
          }
          else {
            Err("In alias definition, \"to\" items must be strings".to_owned())
          }
        }).collect::<Result<Vec<KeyCode>, String>>()?
      }
      else {
        return Err("\"to\" for an alias definition must be either a string or array".to_owned())
      }
    },
    resulting_modifier: {
      if let j::String(text) = to {
        text.to_owned()
      }
      else if let Array(items) = to {
        match items.last() {
          Some(last) => {
            if let j::String(text) = last {
              text.to_owned()
            }
            else {
              return Err("Alias definition \"to\" items must be strings".to_owned())
            }
          },
          None => return Err("Alias definition must have an alias".to_owned()),
        }
      }
      else {
        return Err("Alias definition \"to\" must be string or array".to_owned())
      }
    }
  }))
}

fn parse_modifier(text: &str) -> Result<Modifier, String> {
  if text.starts_with("@") {
    Ok(Modifier::Alias(text.to_owned()))
  }
  else {
    Ok(Modifier::Key(KeyCode::from_str(&text).map_err(|_| format!("Unknown key: {}", text))?))
  }
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

