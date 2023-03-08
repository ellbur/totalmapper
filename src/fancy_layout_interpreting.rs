
use crate::keys as s;
use crate::fancy_keys as f;
use crate::key_codes::KeyCode;
use std::collections::HashMap;

pub fn convert(f: &f::Layout) -> Result<s::Layout, String> {
  let mut res = Vec::new();
  
  Ok(s::Layout {
    mappings: res
  })
}

struct AliasDefinition {
  from: Vec<f::Modifier>,
  to: AliasTo
}

struct AliasTo {
  additional_modifiers: Vec<KeyCode>,
  target: String
}

fn find_alias_definitions(f: &f::Layout) -> Result<HashMap<String, Vec<AliasDefinition>>, String> {
  use f::*;
  
  let mut res = HashMap::new();
  
  for m in &f.mappings {
    todo!()
  }
  
  Ok(res)
}

#[cfg(test)]
mod tests {
  
}

