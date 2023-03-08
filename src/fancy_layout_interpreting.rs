
use crate::fancy_keys::AliasMapping;
use crate::keys as s;
use crate::fancy_keys as f;
use crate::key_codes::KeyCode;
use std::collections::HashMap;

pub fn convert(f: &f::Layout) -> Result<s::Layout, String> {
  let mut res = Vec::new();
  
  let alias_mappings = find_alias_mappings(f);
  
  for m in &f.mappings {
    match m {
      f::Mapping::Alias(alias) => {
        // This test tries to be clever about whethere the user
        // expects modifiers to pass-through.
        if !is_just_one_modifier(&alias.from.keys) {
          res.push(s::Mapping {
            from: alias.from.keys.clone(),
            to: alias.to.initial.clone(),
            repeat: s::Repeat::Normal,
            absorbing: vec![]
          })
        }
      },
      f::Mapping::Single(single) => {
        let modifier_combinations = find_all_alias_combinations(&alias_mappings, &single.from.modifiers)?;
        for modifier_combination in modifier_combinations {
          let mut from = modifier_combination.clone();
          from.push(single.from.key.clone());
          
          let to = translate_single_to_keys(&single.from.modifiers, &modifier_combination, &single.to)?;
          
          let repeat = match &single.repeat {
            f::SingleRepeat::Normal => s::Repeat::Normal,
            f::SingleRepeat::Disabled => s::Repeat::Disabled,
            f::SingleRepeat::Special { keys, delay_ms, interval_ms } => s::Repeat::Special {
              keys: translate_single_to_keys(&single.from.modifiers, &modifier_combination, &keys)?,
              delay_ms: *delay_ms,
              interval_ms: *interval_ms
            }
          };
          
          let absorbing = reify_modifiers(&single.from.modifiers, &modifier_combination, &single.absorbing)?;
          
          res.push(s::Mapping {
            from,
            to,
            repeat,
            absorbing
          });
        }
      },
      f::Mapping::Row(row) => {
        todo!()
      }
    }  
  }
  
  Ok(s::Layout {
    mappings: res
  })
}

fn is_just_one_modifier(ks: &Vec<KeyCode>) -> bool {
  if ks.len() == 1 {
    is_modifier(&ks[0])
  }
  else {
    false
  }
}

fn is_modifier(k: &KeyCode) -> bool {
  use crate::key_codes::KeyCode::*;
  match k {
    LEFTSHIFT => true,
    RIGHTSHIFT => true,
    LEFTALT => true,
    RIGHTALT => true,
    LEFTCTRL => true,
    RIGHTCTRL => true,
    LEFTMETA => true,
    RIGHTMETA => true,
    _ => false
  }
}

fn translate_single_to_keys(lhs_template: &Vec<f::Modifier>, lhs_instance: &Vec<KeyCode>, to: &f::SingleToKeys) -> Result<Vec<KeyCode>, String> {
  Ok(match to.terminal {
    f::SingleTerminalToKey::Physical(terminal) => {
      let mut to = reify_modifiers(lhs_template, lhs_instance, &to.initial)?;
      to.push(terminal);
      to
    },
    f::SingleTerminalToKey::Null => {
      vec![]
    }
  })
}

fn find_all_alias_combinations(alias_mappings: &HashMap<String, Vec<AliasMapping>>, modifiers: &Vec<f::Modifier>) -> Result<Vec<Vec<KeyCode>>, String> {
  todo!()
}

fn reify_modifiers(lhs_template: &Vec<f::Modifier>, lhs_instance: &Vec<KeyCode>, modifiers: &Vec<f::Modifier>) -> Result<Vec<KeyCode>, String> {
  todo!()
}

fn find_alias_mappings(f: &f::Layout) -> HashMap<String, Vec<AliasMapping>> {
  use f::*;
  
  let mut res = HashMap::new();
  
  for m in &f.mappings {
    match m {
      Mapping::Alias(alias) => {
        match res.get_mut(&alias.to.terminal) {
          None => {
            res.insert(alias.to.terminal.clone(), vec![alias.clone()]);
          },
          Some(list) => {
            list.push(alias.clone())
          }
        }
      },
      _ => ()
    }
  }
  
  res
}

#[cfg(test)]
mod tests {
  
}

