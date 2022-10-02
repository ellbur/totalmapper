
use std::str::FromStr;

use key_codes::KeyCode;
use serde_json::{Value, Map};
use Value::{Object, Array};
use crate::{keys::{Layout, Mapping, AliasDefinitionMapping, Modifier}, key_codes};
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
        let from = mapping_values.get("from").unwrap();
        let to = mapping_values.get("to").unwrap();
        
        if let j::String(to_text) = to {
          if to_text.starts_with("@") {
            return parse_alias_definition(mapping_values);
          }
        }
        else if let Array(to_items) = to {
          if !to_items.is_empty() {
            if let j::String(to_text) = to_items.last().unwrap() {
              if to_text.starts_with("@") {
                return parse_alias_definition(mapping_values);
              }
            }
          }
        }
        
        if let Object(from_values) = from {
          if from_values.contains_key("row") {
            return parse_row_to_letters(mapping_values);
          }
        }
        else if let Array(from_items) = from {
          if !from_items.is_empty() {
            let last_from_item = from_items.last().unwrap();
            if let Object(last_from_values) = last_from_item {
              if last_from_values.contains_key("row") {
                return parse_row_to_letters(mapping_values);
              }
            }
          }
        }
        
        parse_basic_mapping(mapping_values)
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

fn parse_row_to_letters(values: &Map<String, Value>) -> Result<Mapping, String> {
  todo!()
}

fn parse_basic_mapping(values: &Map<String, Value>) -> Result<Mapping, String> {
  todo!()
}

fn parse_modifier(text: &str) -> Result<Modifier, String> {
  if text.starts_with("@") {
    Ok(Modifier::Alias(text.to_owned()))
  }
  else {
    Ok(Modifier::Key(KeyCode::from_str(&text).map_err(|_| format!("Unknown key: {}", text))?))
  }
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

