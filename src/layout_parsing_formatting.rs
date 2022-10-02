
use serde_json::{Value, Map};
use Value::{Object, Array};
use crate::keys::Layout;

pub fn parse_layout_from_json(root: &Value) -> Result<Layout, String> {
  match root {
    Object(root_values) => {
      if has_keys(root_values, &vec!["mappings"]) {
        let mappings_v = root_values.get("mappings").unwrap();
        match mappings_v {
          Array(mapping_vs) => {
            let mut mappings = Vec::new();
              
            for mapping_v in mapping_vs {
              match mapping_v {
                Object(mapping_values) => {
                  if has_at_least_keys(mapping_values, &vec!["from", "to"]) {
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

fn has_keys(values: &Map<String, Value>, check: &Vec<&str>) -> bool {
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

