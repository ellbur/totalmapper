
use crate::fancy_keys::AliasMapping;
use crate::keys as s;
use crate::fancy_keys as f;
use crate::key_codes::KeyCode;
use std::collections::HashMap;

pub fn convert(f: &f::Layout) -> Result<s::Layout, String> {
  let mut res = Vec::new();
  let mut from_table: HashMap<FromSet, Vec<usize>> = HashMap::new();
  
  let alias_mappings = find_alias_mappings(f);
  
  for fm in &f.mappings {
    let sms = convert_mapping(&alias_mappings, fm)?;
    for sm in sms {
      let from_set = FromSet::new(&sm.from);
      match from_table.get_mut(&from_set) {
        Some(v) => v.push(res.len()),
        None => {
          let v = vec![res.len()];
          from_table.insert(from_set.clone(), v);
        }
      };
      res.push(sm);
    }
  }
  
  for fm in &f.mappings {
    adjust_repeats(&mut res, &from_table, &alias_mappings, fm)?;
  }
  
  Ok(s::Layout {
    mappings: res
  })
}

fn adjust_repeats<'a>(res: &mut Vec<s::Mapping>, from_table: &HashMap<FromSet, Vec<usize>>, alias_mappings: &'a HashMap<String, Vec<&'a f::AliasMapping>>, fm: &f::Mapping) -> Result<(), String> {
  match fm {
    f::Mapping::RepeatOnlySingle(single) => {
      let modifier_combinations = build_combinations(alias_mappings, &single.from.modifiers)?;
      for modifier_combination in iterate_combinations(&modifier_combinations) {
        let mut from = modifier_combination.from_modifiers().clone();
        from.push(single.from.key.clone());

        let repeat = match &single.repeat {
          f::SingleRepeat::Normal => s::Repeat::Normal,
          f::SingleRepeat::Disabled => s::Repeat::Disabled,
          f::SingleRepeat::Special { keys, delay_ms, interval_ms } => s::Repeat::Special {
            keys: modifier_combination.translate_single_to_keys(&keys)?,
            delay_ms: *delay_ms,
            interval_ms: *interval_ms
          }
        };

        let from_set = FromSet::new(&from);
        if let Some(is) = from_table.get(&from_set) {
          for i in is {
            let sm = &mut res[*i];
            sm.repeat = repeat.clone();
          }
        }
        else {
          res.push(s::Mapping { from: from.clone(), to: from, repeat, absorbing: vec![] });
        }
      }
    },
    _ => ()
  };
  Ok(())
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct FromSet {
  keys: Vec<KeyCode>
}
impl FromSet {
  fn new(keys: &[KeyCode]) -> FromSet {
    if !keys.is_empty() {
      let mut res: Vec<KeyCode> = keys[..keys.len()-1].iter().map(|k| *k).collect();
      res.sort();
      res.push(*keys.last().unwrap());
      FromSet { keys: res }
    }
    else {
      FromSet { keys: vec![] }
    }
  }
}

fn convert_mapping<'a>(alias_mappings: &HashMap<String, Vec<&'a f::AliasMapping>>, m: &f::Mapping) -> Result<Vec<s::Mapping>, String> {
  match m {
    f::Mapping::Alias(alias) => Ok(convert_alias(alias)),
    f::Mapping::Single(single) => convert_single(alias_mappings, single),
    f::Mapping::Row(row) => convert_row(alias_mappings, row),
    f::Mapping::RepeatOnlySingle(_) => Ok(vec![]),
  }
}

fn convert_alias(alias: &f::AliasMapping) -> Vec<s::Mapping> {
  // This test tries to be clever about whethere the user
  // expects modifiers to pass-through.
  if !is_just_one_modifier(&alias.from.keys) {
    vec![s::Mapping {
      from: alias.from.keys.clone(),
      to: alias.to.initial.clone(),
      repeat: s::Repeat::Normal,
      absorbing: vec![]
    }]
  }
  else {
    vec![]
  }
}

fn convert_single<'a>(alias_mappings: &'a HashMap<String, Vec<&'a f::AliasMapping>>, single: &f::SingleMapping) -> Result<Vec<s::Mapping>, String> {
  let mut res = Vec::new();
  let modifier_combinations = build_combinations(alias_mappings, &single.from.modifiers)?;
  for modifier_combination in iterate_combinations(&modifier_combinations) {
    let mut from = modifier_combination.from_modifiers().clone();
    from.push(single.from.key.clone());

    let to = modifier_combination.translate_single_to_keys(&single.to)?;

    let repeat = match &single.repeat {
      f::SingleRepeat::Normal => s::Repeat::Normal,
      f::SingleRepeat::Disabled => s::Repeat::Disabled,
      f::SingleRepeat::Special { keys, delay_ms, interval_ms } => s::Repeat::Special {
        keys: modifier_combination.translate_single_to_keys(&keys)?,
        delay_ms: *delay_ms,
        interval_ms: *interval_ms
      }
    };

    let absorbing = modifier_combination.reify_modifiers(&single.absorbing)?;

    res.push(s::Mapping {
      from,
      to,
      repeat,
      absorbing
    });
  }
  Ok(res)
}

enum RowRepeatTemplate {
  Normal,
  Disabled,
  Special {
    modifiers: Vec<KeyCode>,
    terminal: Vec<char>,
    delay_ms: i32,
    interval_ms: i32
  }
}

fn convert_row<'t>(alias_mappings: &'t HashMap<String, Vec<&'t f::AliasMapping>>, row_mapping: &f::RowMapping) -> Result<Vec<s::Mapping>, String> {
  let mut res = Vec::new();
  let modifier_combinations = build_combinations(alias_mappings, &row_mapping.from.modifiers)?;
  for modifier_combination in iterate_combinations(&modifier_combinations) {
    let from_modifiers = modifier_combination.from_modifiers().clone();
    let to_modifiers = modifier_combination.reify_modifiers(&row_mapping.to.initial)?;
    
    let repeat_template = match &row_mapping.repeat {
      f::RowRepeat::Normal => RowRepeatTemplate::Normal,
      f::RowRepeat::Disabled => RowRepeatTemplate::Disabled,
      f::RowRepeat::Special { keys, delay_ms, interval_ms } => {
        let num_repeat_chars = keys.terminal.chars().count();
        let num_to_chars = row_mapping.to.terminal.chars().count();
        if num_repeat_chars > num_to_chars {
          return Err(format!("Row mapping has more letters in its `repeat` ({} = {}) than its `to` ({} = {}). This is not allowed because it is not clear how such keys should be mapped. Use individual mappings instead.",
            keys.terminal,
            num_repeat_chars,
            row_mapping.to.terminal,
            num_to_chars
          ));
        }
        
        RowRepeatTemplate::Special {
          modifiers: modifier_combination.reify_modifiers(&keys.initial)?,
          terminal: keys.terminal.chars().collect(),
          delay_ms: *delay_ms,
          interval_ms: *interval_ms
        }
      }
    };
    
    use crate::physical_keyboard_layouts::US_KEYBOARD_LAYOUT;
    let from_physical_row = US_KEYBOARD_LAYOUT.get(&row_mapping.from.row)
      .ok_or(format!("Don't have data for row {}", row_mapping.from.row))?;
      
    let has_right_shift = find_right_shift(&from_modifiers);
    let to_terminals: Vec<char> = row_mapping.to.terminal.chars().collect();
    
    for char_i in 0..to_terminals.len() {
      if char_i >= from_physical_row.len() {
        return Err(format!("Don't know which keycode is at index {} in row {:?}", char_i, row_mapping.from.row));
      }
      
      let to = convert_row_to(has_right_shift, &to_modifiers, &to_terminals, char_i)?;
      if let Some(to) = to {
        let mut from = from_modifiers.clone();
        from.push(from_physical_row[char_i]);
        
        let repeat = match &repeat_template {
          RowRepeatTemplate::Normal => s::Repeat::Normal,
          RowRepeatTemplate::Disabled => s::Repeat::Disabled,
          RowRepeatTemplate::Special { modifiers, terminal, delay_ms, interval_ms } => {
            match convert_row_to(has_right_shift, &modifiers, &terminal, char_i)? {
              None => s::Repeat::Normal,
              Some(keys) => s::Repeat::Special { keys, delay_ms: *delay_ms, interval_ms: *interval_ms }
            }
          }
        };

        let absorbing = modifier_combination.reify_modifiers(&row_mapping.absorbing)?;

        res.push(s::Mapping {
          from,
          to,
          repeat,
          absorbing
        });
      }
    }
  }
  Ok(res)
}

fn find_right_shift(from: &Vec<KeyCode>) -> bool {
  for k in from {
    if *k == KeyCode::RIGHTSHIFT {
      return true;
    }
  }
  return false;
}

fn convert_row_to(has_right_shift: bool, modifiers: &Vec<KeyCode>, terminals: &Vec<char>, char_i: usize) -> Result<Option<Vec<KeyCode>>, String> {
  use crate::char_production_map::CHAR_ACCESS_MAP;
  if char_i >= terminals.len() {
    Ok(None)
  }
  else {
    let ch = terminals[char_i];
    // Space is considered unmapped
    if ch == ' ' {
      Ok(None)
    }
    else {
      match CHAR_ACCESS_MAP.get(&ch) {
        None => {
          Err(format!("Don't know how to produce char '{}' on a US keyboard", ch))
        }
        Some(sk) => {
          let mut to = modifiers.clone();
          if sk.sh {
            to.push(if has_right_shift {KeyCode::RIGHTSHIFT} else {KeyCode::LEFTSHIFT});
          }
          to.push(sk.k);
          Ok(Some(to))
        }
      }
    }
  }
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

struct AliasCombinationIterable<'t> {
  modifiers: &'t Vec<f::Modifier>,
  alias_quantities: Vec<usize>,
  alias_found_mappings: Vec<&'t Vec<&'t AliasMapping>>,
  alias_map: HashMap<String, usize>
}

struct AliasCombinationIterator<'s, 't> {
  iterable: &'s AliasCombinationIterable<'t>,
  combinations: MultiplyIter<'s>
}

fn build_combinations<'t>(alias_mappings: &'t HashMap<String, Vec<&'t AliasMapping>>, modifiers: &'t Vec<f::Modifier>) -> Result<AliasCombinationIterable<'t>, String> {
  let mut alias_quantities = Vec::new();
  let mut alias_found_mappings = Vec::new();
  let mut alias_map = HashMap::new();
  
  for i in 0..modifiers.len() {
    let m = &modifiers[i];
    match m {
      f::Modifier::Alias(alias) => {
        let mappings = alias_mappings.get(alias).ok_or(format!("Alias {} is undefined", alias))?;
        let i = alias_quantities.len();
        alias_quantities.push(mappings.len());
        alias_found_mappings.push(mappings);
        alias_map.insert(alias.clone(), i);
      },
      _ => ()
    }
  }
  
  Ok(AliasCombinationIterable {
    modifiers,
    alias_quantities: alias_quantities.clone(),
    alias_found_mappings,
    alias_map
  })
}
  
fn iterate_combinations<'s, 't>(iterable: &'s AliasCombinationIterable<'t>) -> AliasCombinationIterator<'s, 't> {
  AliasCombinationIterator { iterable, combinations: multiply(&iterable.alias_quantities) }
}

struct AliasCombination<'s, 't> {
  it: &'s AliasCombinationIterable<'t>,
  tuple: Vec<usize>
}

impl <'s, 't> AliasCombination<'s, 't> {
  fn from_modifiers(&self) -> Vec<KeyCode> {
    let mut thing = Vec::new();
    let mut j = 0;
    for i in 0..self.it.modifiers.len() {
      let m = &self.it.modifiers[i];
      match m {
        f::Modifier::Alias(_) => {
          let keys = &self.it.alias_found_mappings[j][self.tuple[j]].from.keys;
          thing.extend(keys);
          j += 1;
        },
        f::Modifier::Key(k) => {
          thing.push(k.clone());
        }
      }
    }
    thing
  }
  
  fn translate_single_to_keys(&self, to: &f::SingleToKeys) -> Result<Vec<KeyCode>, String> {
    Ok(match to.terminal {
      f::SingleTerminalToKey::Physical(terminal) => {
        let mut to = self.reify_modifiers(&to.initial)?;
        to.push(terminal);
        to
      },
      f::SingleTerminalToKey::Null => {
        vec![]
      }
    })
  }
  
  fn reify_modifiers(&self, modifiers: &Vec<f::Modifier>) -> Result<Vec<KeyCode>, String> {
    let mut res = Vec::new();
    
    for m in modifiers {
      match m {
        f::Modifier::Key(k) => res.push(*k),
        f::Modifier::Alias(alias) => {
          match self.it.alias_map.get(alias) {
            None => return Err(format!("Alias used on RHS of mapping that does not appear on LHS: {}", alias)),
            Some(&i) => {
              let keys = &self.it.alias_found_mappings[i][self.tuple[i]].from.keys;
              res.extend(keys);
            }
          }
        }
      }
    }
    
    Ok(res)
  }
}

impl <'s, 't> Iterator for AliasCombinationIterator<'s, 't> {
  type Item = AliasCombination<'s, 't>;
  
  fn next(&mut self) -> Option<AliasCombination<'s, 't>> {
    Some(AliasCombination {
      it: &self.iterable,
      tuple: self.combinations.next()?
    })
  }
}

struct MultiplyIter<'s> {
  quantities: &'s Vec<usize>,
  position: Vec<usize>,
  done: bool
}

fn multiply<'s>(quantities: &'s Vec<usize>) -> MultiplyIter<'s> {
  MultiplyIter::new(quantities)
}

impl <'s> MultiplyIter<'s> {
  fn new(quantities: &'s Vec<usize>) -> MultiplyIter<'s> {
    let mut position = Vec::new();
    for _ in 0..quantities.len() {
      position.push(0);
    }
    
    MultiplyIter {
      quantities,
      position,
      done: false
    }
  }
}
  
impl <'s> std::iter::Iterator for MultiplyIter<'s> {
  type Item = Vec<usize>;
  
  fn next(&mut self) -> Option<Vec<usize>> {
    if self.done {
      None
    }
    else {
      let mut found = false;
      let res = self.position.clone();
      for i in 0..self.quantities.len() {
        if self.position[i] < self.quantities[i]-1 {
          self.position[i] += 1;
          for j in 0..i { self.position[j] = 0 }
          found = true;
          break;
        }
      }
      if !found {
        self.done = true;
      }
      Some(res)
    }
  }
}

fn find_alias_mappings<'a>(f: &'a f::Layout) -> HashMap<String, Vec<&'a AliasMapping>> {
  use f::*;
  
  let mut res = HashMap::new();
  
  for m in &f.mappings {
    match m {
      Mapping::Alias(alias) => {
        match res.get_mut(&alias.to.terminal) {
          None => {
            res.insert(alias.to.terminal.clone(), vec![alias]);
          },
          Some(list) => {
            list.push(alias)
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
  use std::collections::HashMap;
  use f::KeyCode;
  use KeyCode::*;
  use crate::fancy_layout_interpreting::convert_row;

use super::{multiply, convert_row_to, convert};
  use super::{convert_single, f, s};
  use f::AliasMapping as AM;
  use f::AliasFromKeys as AFK;
  use f::AliasToKeys as ATK;
  use f::SingleMapping as SM;
  use f::SingleFromKeys as SFK;
  use f::SingleToKeys as STK;
  use f::RowMapping as RM;
  use f::RowFromKeys as RFK;
  use f::RowToKeys as RTK;
  use f::SingleTerminalToKey::Physical;
  use f::Modifier::Alias;
  use f::Modifier::Key;

  // https://stackoverflow.com/a/74854187/371739
  pub trait ToOwnedExt where Self : ToOwned {
    /// Simply an alias for `.to_owned()`.
    fn o(&self) -> <Self as ToOwned>::Owned {
      self.to_owned()
    }
  }
  impl<T: ?Sized> ToOwnedExt for T where T: ToOwned {}
  
  #[test]
  fn test_combinations_1() {
    let quantities = vec![2, 2];
    let res: Vec<Vec<usize>> = multiply(&quantities).collect();
    assert_eq!(res, vec![
      vec![0, 0],
      vec![1, 0],
      vec![0, 1],
      vec![1, 1],
    ]);
  }
  
  #[test]
  fn test_single_convert_1() {
    let mut alias_mappings = HashMap::new();
    let leftshift_shift = AM { from: AFK { keys: vec![LEFTSHIFT] }, to: ATK { initial: vec![], terminal: "@shift".o() } };
    let rightshift_shift = AM { from: AFK { keys: vec![RIGHTSHIFT] }, to: ATK { initial: vec![], terminal: "@shift".o() } };
    alias_mappings.insert("@shift".o(), vec![
      &leftshift_shift,
      &rightshift_shift
    ]);
    
    let single = SM {
      from: SFK { modifiers: vec![Alias("@shift".o())], key: E },
      to: STK { initial: vec![Alias("@shift".o())], terminal: Physical(DOT) },
      repeat: f::SingleRepeat::Special { keys: STK { initial: vec![Key(LEFTCTRL)], terminal: Physical(K3) }, delay_ms: 50, interval_ms: 30 },
      absorbing: vec![Alias("@shift".o())]
    };
    
    let res = convert_single(&alias_mappings, &single).unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], 
      s::Mapping { from: vec![LEFTSHIFT, E], to: vec![LEFTSHIFT, DOT], repeat: s::Repeat::Special { keys: vec![LEFTCTRL, K3], delay_ms: 50, interval_ms: 30 }, absorbing: vec![LEFTSHIFT]  }
    );
    assert_eq!(res[1],
      s::Mapping { from: vec![RIGHTSHIFT, E], to: vec![RIGHTSHIFT, DOT], repeat: s::Repeat::Special { keys: vec![LEFTCTRL, K3], delay_ms: 50, interval_ms: 30 }, absorbing: vec![RIGHTSHIFT]  }
    );
  }
  
  #[test]
  fn test_row_convert_1() {
    let mut alias_mappings = HashMap::new();
    let leftshift_shift = AM { from: AFK { keys: vec![LEFTSHIFT] }, to: ATK { initial: vec![], terminal: "@shift".o() } };
    let rightshift_shift = AM { from: AFK { keys: vec![RIGHTSHIFT] }, to: ATK { initial: vec![], terminal: "@shift".o() } };
    alias_mappings.insert("@shift".o(), vec![
      &leftshift_shift,
      &rightshift_shift
    ]);
    
    let row = RM {
      from: RFK { modifiers: vec![Alias("@shift".o())], row: f::Row::USQuertyA },
      to: RTK { initial: vec![], terminal: "AOEU".o() },
      repeat: f::RowRepeat::Special { keys: RTK { initial: vec![], terminal: "aoeu".o() }, delay_ms: 50, interval_ms: 30 },
      absorbing: vec![Alias("@shift".o())]
    };
    
    use s::Repeat::Special as SRS;
    use s::Mapping as SM;
    
    use KeyCode::LEFTSHIFT as LS;
    use KeyCode::RIGHTSHIFT as RS;
    
    let res = convert_row(&alias_mappings, &row).unwrap();
    assert_eq!(res.len(), 8);
    
    assert_eq!(res[0], SM { from: vec![LS, A], to: vec![LS, A], repeat: SRS { keys: vec![A], delay_ms: 50, interval_ms: 30 }, absorbing: vec![LS]  });
    assert_eq!(res[1], SM { from: vec![LS, S], to: vec![LS, O], repeat: SRS { keys: vec![O], delay_ms: 50, interval_ms: 30 }, absorbing: vec![LS]  });
    assert_eq!(res[2], SM { from: vec![LS, D], to: vec![LS, E], repeat: SRS { keys: vec![E], delay_ms: 50, interval_ms: 30 }, absorbing: vec![LS]  });
    assert_eq!(res[3], SM { from: vec![LS, F], to: vec![LS, U], repeat: SRS { keys: vec![U], delay_ms: 50, interval_ms: 30 }, absorbing: vec![LS]  });
    
    assert_eq!(res[4], SM { from: vec![RS, A], to: vec![RS, A], repeat: SRS { keys: vec![A], delay_ms: 50, interval_ms: 30 }, absorbing: vec![RS]  });
    assert_eq!(res[5], SM { from: vec![RS, S], to: vec![RS, O], repeat: SRS { keys: vec![O], delay_ms: 50, interval_ms: 30 }, absorbing: vec![RS]  });
    assert_eq!(res[6], SM { from: vec![RS, D], to: vec![RS, E], repeat: SRS { keys: vec![E], delay_ms: 50, interval_ms: 30 }, absorbing: vec![RS]  });
    assert_eq!(res[7], SM { from: vec![RS, F], to: vec![RS, U], repeat: SRS { keys: vec![U], delay_ms: 50, interval_ms: 30 }, absorbing: vec![RS]  });
  }
  
  #[test]
  fn test_row_convert_2() {
    let mut alias_mappings = HashMap::new();
    let leftshift_shift = AM { from: AFK { keys: vec![LEFTSHIFT] }, to: ATK { initial: vec![], terminal: "@shift".o() } };
    alias_mappings.insert("@shift".o(), vec![
      &leftshift_shift,
    ]);
    
    let row = RM {
      from: RFK { modifiers: vec![Alias("@shift".o())], row: f::Row::USQuertyA },
      to: RTK { initial: vec![], terminal: "A".o() },
      repeat: f::RowRepeat::Normal,
      absorbing: vec![]
    };
    
    use s::Mapping as SM;
    
    use KeyCode::LEFTSHIFT as LS;
    
    let res = convert_row(&alias_mappings, &row).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], SM { from: vec![LS, A], to: vec![LS, A], repeat: s::Repeat::Normal, absorbing: vec![]  });
  }
  
  #[test]
  fn test_convert_row_to_1() {
    // fn convert_row_to(has_right_shift: bool, modifiers: &Vec<KeyCode>, terminals: &Vec<char>, char_i: usize) -> Result<Option<Vec<KeyCode>>, String>
    let modifiers = vec![];
    let terminals = vec!['A'];
    let res = convert_row_to(false, &modifiers, &terminals, 0).unwrap().unwrap();
    assert_eq!(res, vec![LEFTSHIFT, A]);
  }
  
  #[test]
  fn test_repeat_only_1() {
    let layout_json = r#"{
  "mappings": [
    {"from": "LEFTSHIFT", "to": "@shift"},
    {"from": ["@shift", {"row": "A"}], "to": {"letters": "S"}},
    {"from": ["@shift", "A"], "repeat": {"Special": {"keys": "F24", "delay_ms": 180, "interval_ms": 30}}}
  ]
}"#;
    let layout_v = serde_json::from_str(layout_json).unwrap();
    let fancy_layout = crate::layout_parsing_formatting::parse_layout_from_json(&layout_v).unwrap();
    let simple_layout = convert(&fancy_layout).unwrap();
    assert_eq!(simple_layout.mappings.len(), 1);
    use s::Mapping as SM;
    use KeyCode::LEFTSHIFT as LS;
    assert_eq!(simple_layout.mappings[0], SM { from: vec![LS, A], to: vec![LS, S], repeat: s::Repeat::Special {
      keys: vec![F24], delay_ms: 180, interval_ms: 30 }, absorbing: vec![] });
  }

  #[test]
  fn test_caps_q_escape() {
    let layout_json = r#"{
  "mappings": [
    { "from": "CAPSLOCK", "to": [] },
    { "from": ["CAPSLOCK", "Q"], "to": "ESC" }
  ]
}"#;
    let layout_v = serde_json::from_str(layout_json).unwrap();
    let fancy_layout = crate::layout_parsing_formatting::parse_layout_from_json(&layout_v).unwrap();
    let simple_layout = convert(&fancy_layout).unwrap();
    assert_eq!(simple_layout.mappings.len(), 2);
    use s::Mapping as SM;
    assert_eq!(simple_layout.mappings[0], SM { from: vec![CAPSLOCK], to: vec![], repeat: s::Repeat::Normal, absorbing: vec![] });
    assert_eq!(simple_layout.mappings[1], SM { from: vec![CAPSLOCK, Q], to: vec![ESC], repeat: s::Repeat::Normal, absorbing: vec![] });
  }
}

