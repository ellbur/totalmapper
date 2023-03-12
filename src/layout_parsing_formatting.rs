
use std::str::FromStr;
use key_codes::KeyCode;
use serde_json::{Value, Map};
use Value::{Object, Array};
use crate::{fancy_keys::{Layout, Mapping, SingleMapping, AliasMapping, RowMapping, Modifier, SingleFromKeys, RowFromKeys, SingleToKeys, RowToKeys, SingleTerminalToKey, SingleRepeat, RowRepeat, Row, AliasToKeys, AliasFromKeys, RepeatOnlySingleMapping}, key_codes};
use serde_json::Value as j;
use serde_json::json;
use lazy_static::lazy_static;
use std::collections::HashMap;

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
            
            let mut defined_alias_names = std::collections::HashSet::new();
            for m in &mappings {
              match m {
                Mapping::Alias(alias) => {
                  defined_alias_names.insert(alias.to.terminal.clone());
                }
                _ => ()
              }
            }
            
            for m in &mappings {
              let used_aliases = mapping_all_used_aliases(m);
              for a in &used_aliases {
                if !defined_alias_names.contains(a) {
                  return Err(format!("Error in mapping {}: alias {} is not defined", format_mapping(m), a));
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

fn just_mods(m: &Modifier) -> Option<String> {
  match m {
    Modifier::Alias(name) => Some(name.clone()),
    _ => None  
  }
}

fn mapping_all_used_aliases(m: &Mapping) -> Vec<String> {
  let none = vec![];
  match m {
    Mapping::Alias(_) => vec![],
    Mapping::Single(single) => {
      single.from.modifiers.iter().filter_map(just_mods)
        .chain(single.to.initial.iter().filter_map(just_mods))
        .chain((match &single.repeat {
          SingleRepeat::Special { keys, delay_ms: _, interval_ms: _ } => &keys.initial,
          _ => &none
        }).iter().filter_map(just_mods))
        .chain(single.absorbing.iter().filter_map(just_mods))
        .collect()
    },
    Mapping::Row(row) => {
      row.from.modifiers.iter().filter_map(just_mods)
        .chain(row.to.initial.iter().filter_map(just_mods))
        .chain((match &row.repeat {
          RowRepeat::Special { keys, delay_ms: _, interval_ms: _ } => &keys.initial,
          _ => &none
        }).iter().filter_map(just_mods))
        .chain(row.absorbing.iter().filter_map(just_mods))
        .collect()
    },
    Mapping::RepeatOnlySingle(single) => {
      single.from.modifiers.iter().filter_map(just_mods)
        .chain((match &single.repeat {
          SingleRepeat::Special { keys, delay_ms: _, interval_ms: _ } => &keys.initial,
          _ => &none
        }).iter().filter_map(just_mods))
        .collect()
    },
  }
}

fn parse_mapping_from_json(mapping_v: &Value) -> Result<Mapping, String> {
  match mapping_v {
    Object(mapping_values) => {
      if has_at_least_keys(mapping_values, &vec!["from", "to"]) {
        let from = parse_from(mapping_values.get("from").unwrap())?;
        match from {
          FromKeys::Single(from) => {
            let to = parse_single_or_alias_to(mapping_values.get("to").unwrap())?;
            match to {
              SingleOrAliasToKeys::Single(to) => {
                let repeat = parse_single_repeat(&mapping_values.get("repeat"))?;
                let absorbing = parse_absorbing(&mapping_values.get("absorbing"))?;
                for m in &absorbing {
                  if !from.modifiers.contains(m) {
                    return Err(format!("Error in mapping {}: absorbed modifier {} does not appear on `from` side", mapping_v, m));
                  }
                }
                
                Ok(Mapping::Single(SingleMapping {
                  from, to, repeat, absorbing
                }))
              },
              SingleOrAliasToKeys::Alias(to) => {
                if mapping_values.contains_key("repeat") { Err("`repeat` not allowed for alias mappings")?; }
                if mapping_values.contains_key("absorbing") { Err("`absorbing` not allowed for alias mappings")?; }
                Ok(Mapping::Alias(AliasMapping { from: single_to_alias_from(&from)?, to }))
              }
            }
          },
          FromKeys::Row(from) => {
            let to = parse_row_to(mapping_values.get("to").unwrap())?;
            let repeat = parse_row_repeat(&mapping_values.get("repeat"))?;
            match &repeat {
              RowRepeat::Special { keys, delay_ms: _, interval_ms: _ } => {
                let num_repeat_chars = keys.terminal.chars().count();
                let num_to_chars = to.terminal.chars().count();
                if num_repeat_chars > num_to_chars {
                  return Err(format!("Row mapping {} has more letters in its `repeat` ({} = {}) than its `to` ({} = {}). This is not allowed because it is not clear how such keys should be mapped. Use individual mappings instead.",
                    mapping_v,
                    keys.terminal,
                    num_repeat_chars,
                    to.terminal,
                    num_to_chars
                  ));
                }
              }
              _ => ()
            };
            
            let absorbing = parse_absorbing(&mapping_values.get("absorbing"))?;
            for m in &absorbing {
              if !from.modifiers.contains(m) {
                return Err(format!("Error in mapping {}: absorbed modifier {} does not appear on `from` side", mapping_v, m));
              }
            }

            Ok(Mapping::Row(RowMapping {
              from, to, repeat, absorbing
            }))
          }
        }
      }
      else if has_exactly_keys(mapping_values, &vec!["from", "repeat"]) {
        let from = parse_from(mapping_values.get("from").unwrap())?;
        match from {
          FromKeys::Single(from) => {
            let repeat = parse_single_repeat(&mapping_values.get("repeat"))?;
            Ok(Mapping::RepeatOnlySingle(RepeatOnlySingleMapping { from, repeat }))
          },
          FromKeys::Row(_) => {
            Err("Cannot have a repeat-only row mapping".to_owned())
          }
        }
      }
      else {
        return Err("Mapping must have \"from\" and \"to\" or \"from\" and \"repeat\" ".to_owned())
      }
    },
    _ => {
      return Err("Each \"mapping\" must be an object".to_owned())
    }
  }
}

fn single_to_alias_from(from: &SingleFromKeys) -> Result<AliasFromKeys, String> {
  let mut keys = Vec::new();
  
  for m in &from.modifiers {
    match m {
      Modifier::Key(key) => {
        keys.push(key.clone());
      },
      Modifier::Alias(_) => {
        return Err("Alias mapping cannot use alias modifier".to_owned());
      }
    }
  }
  
  keys.push(from.key.clone());
  
  Ok(AliasFromKeys { keys })
}

enum FromKeys {
  Single(SingleFromKeys),
  Row(RowFromKeys)
}

fn parse_from(from_v: &Value) -> Result<FromKeys, String> {
  if let Array(from_elems) = from_v {
    if from_elems.len() == 0 {
      Err("Can't map from zero keys, i.e. []".to_owned())
    }
    else {
      let modifiers = parse_from_modifiers(&from_elems[0..from_elems.len()-1])?;
      let key = parse_from_key(&from_elems[from_elems.len()-1])?;
      match key {
        FromKey::Single(key) => Ok(FromKeys::Single(SingleFromKeys { modifiers, key })),
        FromKey::Row(row) => Ok(FromKeys::Row(RowFromKeys { modifiers, row }))
      }
    }
  }
  else {
    let key = parse_from_key(from_v)?;
    match key {
      FromKey::Single(key) => Ok(FromKeys::Single(SingleFromKeys { modifiers: vec![], key })),
      FromKey::Row(row) => Ok(FromKeys::Row(RowFromKeys { modifiers: vec![], row }))
    }
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

enum FromKey {
  Single(KeyCode),
  Row(Row)
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
      Ok(FromKey::Row(parse_row(row_text)?))
    }
    else {
      Err(format!("`row` must be a string, found {}", row_obj))
    }
  }
  else {
    Err("Row must be specified by single key, `row`, which is the first key in theh row".to_owned())
  }
}

lazy_static! {
  static ref ROW_NAMES: HashMap<String, Row> = {
    use crate::fancy_keys::Row::*;
    vec![
     ("`".to_string(), USQuertyGrave),
     ("1".to_string(), USQuerty1),
     ("Q".to_string(), USQuertyQ),
     ("A".to_string(), USQuertyA),
     ("Z".to_string(), USQuertyZ),
    ].into_iter().collect()
  };
}

fn parse_row(text: &str) -> Result<Row, String> {
  match ROW_NAMES.get(&text.to_uppercase()) {
    None => {
      let expected: Vec<String> = ROW_NAMES.keys().map(|s| s.to_owned()).collect();
      let expected = expected.join(", ");
      Err(format!("Don't know row {}, expected one of {}", text, expected))
    },
    Some(row) => Ok(row.clone())
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

enum SingleOrAliasToKeys {
  Single(SingleToKeys),
  Alias(AliasToKeys)
}

fn parse_single_or_alias_to(to_v: &Value) -> Result<SingleOrAliasToKeys, String> {
  if let j::Array(to_elems) = to_v {
    parse_single_or_alias_to_array(to_elems)
  }
  else {
    let terminal = parse_single_or_alias_to_terminal(to_v)?;
    match terminal {
      SingleOrAliasToTerminal::Single(terminal) => Ok(SingleOrAliasToKeys::Single(SingleToKeys { initial: vec![], terminal })),
      SingleOrAliasToTerminal::Alias(terminal) => Ok(SingleOrAliasToKeys::Alias(AliasToKeys { initial: vec![], terminal }))
    }
  }
}

fn parse_single_to(to_v: &Value) -> Result<SingleToKeys, String> {
  if let j::Array(to_elems) = to_v {
    parse_single_to_array(to_elems)
  }
  else {
    Ok(SingleToKeys {
      initial: vec![],
      terminal: parse_single_to_terminal(to_v)?
    })
  }
}

fn parse_row_to(to_v: &Value) -> Result<RowToKeys, String> {
  if let j::Array(to_elems) = to_v {
    parse_row_to_array(to_elems)
  }
  else {
    Ok(RowToKeys {
      initial: vec![],
      terminal: parse_row_to_terminal(to_v)?
    })
  }
}

enum SingleOrAliasToTerminal {
  Single(SingleTerminalToKey),
  Alias(String)
}

fn parse_single_or_alias_to_terminal(to_v: &Value) -> Result<SingleOrAliasToTerminal, String> {
  if let j::String(to_text) = to_v {
    parse_single_or_alias_to_text(to_text)
  }
  else if let j::Object(_) = to_v {
    Err(format!("`to` object of unrecognized form {}", to_v))
  }
  else {
    Err(format!("`to` should be a string, array, or object; found {}", to_v))
  }
}

fn parse_single_to_terminal(to_v: &Value) -> Result<SingleTerminalToKey, String> {
  if let j::String(to_text) = to_v {
    parse_single_to_text(to_text)
  }
  else if let j::Object(_) = to_v {
    Err(format!("`to` object of unrecognized form {}", to_v))
  }
  else {
    Err(format!("`to` should be a string, array, or object; found {}", to_v))
  }
}

fn parse_row_to_terminal(to_v: &Value) -> Result<String, String> {
  if let j::Object(to_attrs) = to_v {
    parse_row_to_obj(to_attrs)
  }
  else {
    Err(format!("`to` should be object with key `letters`; found {}", to_v))
  }
}

fn parse_single_or_alias_to_text(to_text: &str) -> Result<SingleOrAliasToTerminal, String> {
  if to_text.starts_with("@") {
    Ok(SingleOrAliasToTerminal::Alias(to_text.to_owned()))
  }
  else {
    Ok(SingleOrAliasToTerminal::Single(SingleTerminalToKey::Physical(KeyCode::from_str(&to_text).map_err(|_| format!("Unknown key code: {}", to_text))?)))
  }
}

fn parse_single_to_text(to_text: &str) -> Result<SingleTerminalToKey, String> {
  if to_text.starts_with("@") {
    Err(format!("Alias {} not allowed in this position", to_text))
  }
  else {
    Ok(SingleTerminalToKey::Physical(KeyCode::from_str(&to_text).map_err(|_| format!("Unknown key code: {}", to_text))?))
  }
}

fn parse_single_or_alias_to_array(to_elems: &[Value]) -> Result<SingleOrAliasToKeys, String> {
  if to_elems.len() == 0 {
    Ok(SingleOrAliasToKeys::Single(SingleToKeys {
      initial: vec![],
      terminal: SingleTerminalToKey::Null
    }))
  }
  else {
    let terminal = parse_single_or_alias_to_terminal(&to_elems[to_elems.len()-1])?;
    
    match terminal {
      SingleOrAliasToTerminal::Single(terminal) => Ok(SingleOrAliasToKeys::Single(SingleToKeys {
        initial: parse_to_initial(&to_elems[0..to_elems.len()-1])?,
        terminal
      })),
      SingleOrAliasToTerminal::Alias(terminal) => Ok(SingleOrAliasToKeys::Alias(AliasToKeys {
        initial: parse_alias_to_initial(&to_elems[0..to_elems.len()-1])?,
        terminal
      })),
    }
  }
}

fn parse_single_to_array(to_elems: &[Value]) -> Result<SingleToKeys, String> {
  if to_elems.len() == 0 {
    Ok(SingleToKeys {
      initial: vec![],
      terminal: SingleTerminalToKey::Null
    })
  }
  else {
    Ok(SingleToKeys {
      initial: parse_to_initial(&to_elems[0..to_elems.len()-1])?,
      terminal: parse_single_to_terminal(&to_elems[to_elems.len()-1])?
    })
  }
}

fn parse_row_to_array(to_elems: &[Value]) -> Result<RowToKeys, String> {
  if to_elems.len() == 0 {
    Err("Cannot map row to an empty array, must map to { \"letters\": \"...\" }".to_owned())
  }
  else {
    Ok(RowToKeys {
      initial: parse_to_initial(&to_elems[0..to_elems.len()-1])?,
      terminal: parse_row_to_terminal(&to_elems[to_elems.len()-1])?
    })
  }
}

fn parse_to_initial(initial_elems: &[Value]) -> Result<Vec<Modifier>, String> {
  let mut res = vec![];
  
  for elem in initial_elems {
    res.push(parse_to_initial_elem(elem)?);
  }
  
  Ok(res)
}

fn parse_alias_to_initial(initial_elems: &[Value]) -> Result<Vec<KeyCode>, String> {
  let mut res = vec![];
  
  for elem in initial_elems {
    res.push(parse_key_code_j(elem)?);
  }
  
  Ok(res)
}

fn parse_to_initial_elem(elem: &Value) -> Result<Modifier, String> {
  if let j::String(text) = elem {
    if text.starts_with("@") {
      Ok(Modifier::Alias(text.to_owned()))
    }
    else {
      Ok(Modifier::Key(KeyCode::from_str(&text).map_err(|_| format!("Unknown key code: {}", text))?))
    }
  }
  else {
    Err(format!("Modifier must be a string, found {}", elem))
  }
}

fn parse_row_to_obj(to_attrs: &Map<String, Value>) -> Result<String, String> {
  if has_exactly_keys(to_attrs, &vec!["letters"]) {
    let letters = to_attrs.get("letters").unwrap();
    if let j::String(letters_text) = letters {
      Ok(letters_text.to_owned())
    }
    else {
      Err(format!("`letters` must be a string, found {}", letters))
    }
  }
  else {
    Err(format!("`to` object of unrecognized form {}, expected, for example, `letters`", j::Object(to_attrs.clone())))
  }
}

fn parse_key_code_j(v: &Value) -> Result<KeyCode, String> {
  if let j::String(text) = v {
    parse_key_code(text)
  }
  else {
    Err(format!("A string (keycode) was expected, but found {}", v))
  }
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

fn parse_single_repeat(v: &Option<&Value>) -> Result<SingleRepeat, String> {
  if let Some(v) = v {
    if let j::String(text) = v {
      if text.to_lowercase() == "normal" {
        Ok(SingleRepeat::Normal)
      }
      else if text.to_lowercase() == "disabled" {
        Ok(SingleRepeat::Disabled)
      }
      else {
        Err(format!("Unrecognized repeat style: {}", text))
      }
    }
    else if let j::Object(params) = v {
      if has_exactly_keys(params, &vec!["Special"]) {
        let special = params.get("Special").unwrap();
        if let j::Object(special) = special {
          if has_exactly_keys(special, &vec!["keys", "delay_ms", "interval_ms"]) {
            let keys = special.get("keys").unwrap();
            let delay_ms = special.get("delay_ms").unwrap();
            let interval_ms = special.get("interval_ms").unwrap();
            
            Ok(SingleRepeat::Special {
              keys: parse_single_repeat_keys(keys)?,
              delay_ms: parse_repeat_delay_ms(delay_ms)?,
              interval_ms: parse_repeat_interval_ms(interval_ms)?
            })
          }
          else {
            Err(format!("`Special` repeat must have attributes `keys`, `delay_ms`, and `interval_ms`, found {}", keys_string(special)))
          }
        }
        else {
          Err(format!("`Special` repeat must be an object, found {}", special))
        }
      }
      else {
        Err(format!("Unknown repeat style: {}", v))
      }
    }
    else {
      Err(format!("Unknown repeat style: {}", v))
    }
  }
  else {
    Ok(SingleRepeat::Normal)
  }
}

fn parse_row_repeat(v: &Option<&Value>) -> Result<RowRepeat, String> {
  if let Some(v) = v {
    if let j::String(text) = v {
      if text.to_lowercase() == "normal" {
        Ok(RowRepeat::Normal)
      }
      else if text.to_lowercase() == "disabled" {
        Ok(RowRepeat::Disabled)
      }
      else {
        Err(format!("Unrecognized repeat style: {}", text))
      }
    }
    else if let j::Object(params) = v {
      if has_exactly_keys(params, &vec!["Special"]) {
        let special = params.get("Special").unwrap();
        if let j::Object(special) = special {
          if has_exactly_keys(special, &vec!["keys", "delay_ms", "interval_ms"]) {
            let keys = special.get("keys").unwrap();
            let delay_ms = special.get("delay_ms").unwrap();
            let interval_ms = special.get("interval_ms").unwrap();
            
            Ok(RowRepeat::Special {
              keys: parse_row_repeat_keys(keys)?,
              delay_ms: parse_repeat_delay_ms(delay_ms)?,
              interval_ms: parse_repeat_interval_ms(interval_ms)?
            })
          }
          else {
            Err(format!("`Special` repeat must have attributes `keys`, `delay_ms`, and `interval_ms`, found {}", keys_string(special)))
          }
        }
        else {
          Err(format!("`Special` repeat must be an object, found {}", special))
        }
      }
      else {
        Err(format!("Unknown repeat style: {}", v))
      }
    }
    else {
      Err(format!("Unknown repeat style: {}", v))
    }
  }
  else {
    Ok(RowRepeat::Normal)
  }
}

fn parse_single_repeat_keys(v: &Value) -> Result<SingleToKeys, String> {
  parse_single_to(v)
}

fn parse_row_repeat_keys(v: &Value) -> Result<RowToKeys, String> {
  parse_row_to(v)
}

fn parse_repeat_delay_ms(v: &Value) -> Result<i32, String> {
  if let j::Number(n) = v {
    Ok(n.as_i64().ok_or(format!("Invalid delay_ms number: {}", v))? as i32)
  }
  else {
    Err(format!("delay_ms must be a number, found {}", v))
  }
}

fn parse_repeat_interval_ms(v: &Value) -> Result<i32, String> {
  if let j::Number(n) = v {
    Ok(n.as_i64().ok_or(format!("Invalid interval_ms number: {}", v))? as i32)
  }
  else {
    Err(format!("interval_ms must be a number, found {}", v))
  }
}

fn parse_absorbing(v: &Option<&Value>) -> Result<Vec<Modifier>, String> {
  if let Some(v) = v {
    if let j::Array(elems) = v {
      let mut res = vec![];
      for elem in elems {
        if let j::String(elem) = elem {
          res.push(parse_modifier(elem)?);
        }
        else {
          Err(format!("`absorbing` must be a list of modifiers, found {}", elem))?;
        }
      }
      Ok(res)
    }
    else if let j::String(elem) = v {
      Ok(vec![parse_modifier(elem)?])
    }
    else {
      Err(format!("`absorbing` must be a list of modifiers, found {}", v))
    }
  }
  else {
    Ok(vec![])
  }
}

fn keys_string(values: &Map<String, Value>) -> String {
  let v1: Vec<&str> = values.keys().map(|s|s.as_str()).collect();
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

#[cfg(test)]
pub fn format_layout_as_json(layout: &Layout) -> Value {
  let mappings: Vec<Value> = layout.mappings.iter().map(format_mapping).collect();
  
  let mut keys = Map::new();
  keys.insert("mappings".to_owned(), j::Array(mappings));
  
  j::Object(keys)
}

fn format_mapping(mapping: &Mapping) -> Value {
  match mapping {
    Mapping::Single(single) => format_single_mapping(single),
    Mapping::Alias(alias) => format_alias_mapping(alias),
    Mapping::Row(row) => format_row_mapping(row),
    Mapping::RepeatOnlySingle(single) => format_repeat_only_single_mapping(single),
  }
}

fn format_single_mapping(mapping: &SingleMapping) -> Value {
  let mut keys = Map::new();
  
  keys.insert("from".to_owned(), format_single_from(&mapping.from));
  keys.insert("to".to_owned(), format_single_to(&mapping.to));
  if let Some(repeat) = format_single_repeat(&mapping.repeat) {
    keys.insert("repeat".to_owned(), repeat);
  }
  if let Some(absorbing) = format_absorbing(&mapping.absorbing) {
    keys.insert("absorbing".to_owned(), absorbing);
  }
  
  j::Object(keys)
}

fn format_repeat_only_single_mapping(mapping: &RepeatOnlySingleMapping) -> Value {
  let mut keys = Map::new();
  
  keys.insert("from".to_owned(), format_single_from(&mapping.from));
  if let Some(repeat) = format_single_repeat(&mapping.repeat) {
    keys.insert("repeat".to_owned(), repeat);
  }
  
  j::Object(keys)
}

fn format_alias_mapping(mapping: &AliasMapping) -> Value {
  let mut keys = Map::new();
  
  keys.insert("from".to_owned(), format_alias_from(&mapping.from));
  keys.insert("to".to_owned(), format_alias_to(&mapping.to));
  
  j::Object(keys)
}

fn format_row_mapping(mapping: &RowMapping) -> Value {
  let mut keys = Map::new();
  
  keys.insert("from".to_owned(), format_row_from(&mapping.from));
  keys.insert("to".to_owned(), format_row_to(&mapping.to));
  if let Some(repeat) = format_row_repeat(&mapping.repeat) {
    keys.insert("repeat".to_owned(), repeat);
  }
  if let Some(absorbing) = format_absorbing(&mapping.absorbing) {
    keys.insert("absorbing".to_owned(), absorbing);
  }
  
  j::Object(keys)
}

fn format_single_from(from: &SingleFromKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &from.modifiers {
    elems.push(format_modifier(m));
  }
  
  elems.push(j::String(format!("{}", from.key)));
  
  if elems.len() == 1 {
    elems.remove(0)
  }
  else {
    j::Array(elems)
  }
}

fn format_alias_from(from: &AliasFromKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &from.keys {
    elems.push(j::String(format!("{}", m)));
  }
  
  if elems.len() == 1 {
    elems.remove(0)
  }
  else {
    j::Array(elems)
  }
}

fn format_row_from(from: &RowFromKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &from.modifiers {
    elems.push(format_modifier(m));
  }
  
  elems.push(format_row(&from.row));
  
  if elems.len() == 1 {
    elems.remove(0)
  }
  else {
    j::Array(elems)
  }
}

fn format_key_code(k: &KeyCode) -> Value {
  j::String(format!("{}", k))
}

fn format_modifier(m: &Modifier) -> Value {
  match m {
    Modifier::Key(k) => j::String(format!("{}", k)),
    Modifier::Alias(a) => j::String(a.clone())
  }
}

fn format_row(row: &Row) -> Value {
  let mut keys = Map::new();
  use crate::fancy_keys::Row::*;

  keys.insert("row".to_owned(), j::String(match row {
    USQuertyGrave => "`".to_owned(),
    USQuerty1 => "1".to_owned(),
    USQuertyQ => "Q".to_owned(),
    USQuertyA => "A".to_owned(),
    USQuertyZ => "Z".to_owned()
  }));
  
  j::Object(keys)
}

fn format_alias_to(to: &AliasToKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &to.initial {
    elems.push(format_key_code(m));
  }
  
  elems.push(j::String(to.terminal.clone()));
  
  if elems.len() == 1 {
    elems.remove(0)
  }
  else {
    j::Array(elems)
  }
}

fn format_single_to(to: &SingleToKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &to.initial {
    elems.push(format_modifier(m));
  }
  
  match &to.terminal {
    SingleTerminalToKey::Null => elems.clear(),
    SingleTerminalToKey::Physical(k) => elems.push(j::String(format!("{}", k))),
  }
  
  if elems.len() == 1 {
    elems.remove(0)
  }
  else {
    j::Array(elems)
  }
}

fn format_row_to(to: &RowToKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &to.initial {
    elems.push(format_modifier(m));
  }
  
  elems.push(format_letters(&to.terminal));
  
  if elems.len() == 1 {
    elems.remove(0)
  }
  else {
    j::Array(elems)
  }
}

fn format_letters(s: &str) -> Value {
  let mut keys = Map::new();
  
  keys.insert("letters".to_owned(), j::String(s.to_owned()));
  
  j::Object(keys)
}

fn format_single_repeat(repeat: &SingleRepeat) -> Option<Value> {
  match repeat {
    SingleRepeat::Normal => None,
    SingleRepeat::Disabled => Some(j::String("Disabled".to_owned())),
    SingleRepeat::Special { keys, delay_ms, interval_ms } => Some(format_single_repeat_special(keys, *delay_ms, *interval_ms))
  }
}

fn format_row_repeat(repeat: &RowRepeat) -> Option<Value> {
  match repeat {
    RowRepeat::Normal => None,
    RowRepeat::Disabled => Some(j::String("Disabled".to_owned())),
    RowRepeat::Special { keys, delay_ms, interval_ms } => Some(format_row_repeat_special(keys, *delay_ms, *interval_ms))
  }
}

fn format_single_repeat_special(keys: &SingleToKeys, delay_ms: i32, interval_ms: i32) -> Value {
  let mut elems1 = Map::new();
  let mut elems2 = Map::new();
  
  elems2.insert("keys".to_owned(), format_single_to(keys));
  elems2.insert("delay_ms".to_owned(), json!(delay_ms));
  elems2.insert("interval_ms".to_owned(), json!(interval_ms));
  
  elems1.insert("Special".to_owned(), j::Object(elems2));
  
  j::Object(elems1)
}

fn format_row_repeat_special(keys: &RowToKeys, delay_ms: i32, interval_ms: i32) -> Value {
  let mut elems1 = Map::new();
  let mut elems2 = Map::new();
  
  elems2.insert("keys".to_owned(), format_row_to(keys));
  elems2.insert("delay_ms".to_owned(), json!(delay_ms));
  elems2.insert("interval_ms".to_owned(), json!(interval_ms));
  
  elems1.insert("Special".to_owned(), j::Object(elems2));
  
  j::Object(elems1)
}

fn format_absorbing(absorbing: &Vec<Modifier>) -> Option<Value> {
  if absorbing.is_empty() {
    None
  }
  else {
    let mut res = Vec::new();
    
    for m in absorbing {
      res.push(format_modifier(m));
    }
    
    if res.len() == 1 {
      Some(res.remove(0))
    }
    else {
      Some(j::Array(res))
    }
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use crate::fancy_keys::{Layout, Mapping, SingleMapping, RowMapping, SingleFromKeys, RowFromKeys, Modifier, SingleToKeys, RowToKeys, SingleTerminalToKey, SingleRepeat, RowRepeat, AliasMapping, AliasFromKeys, AliasToKeys};
  use super::{parse_layout_from_json, format_layout_as_json};
  use crate::key_codes::KeyCode::*;

  #[test]
  fn test_parsing_1() {
    let text = r#"{
  "mappings": [
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" }
  ]
}"#;
    let json = serde_json::Value::from_str(text).unwrap();
    let parsed = parse_layout_from_json(&json).unwrap();
    assert_eq!(parsed, Layout {
      mappings: vec![
        Mapping::Alias(AliasMapping { from: AliasFromKeys { keys: vec![CAPSLOCK] }, to: AliasToKeys { initial: vec![], terminal: "@symbol".to_owned() } }),
        Mapping::Alias(AliasMapping { from: AliasFromKeys { keys: vec![RIGHTALT] }, to: AliasToKeys { initial: vec![], terminal: "@symbol".to_owned() } }),
      ]
    });
  }

  #[test]
  fn test_parsing_2() {
    let text = r#"{
  "mappings": [
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" },
    { "from": ["@symbol", {"row": "Q"}], "to": {"letters": " {}% \\*][|"} },
    { "from": ["@symbol", {"row": "A"}], "to": {"letters": "   = &)(/_$"} },
    { "from": ["@symbol", {"row": "Z"}], "to": {"letters": "\"    !+#"} }
  ]
}"#;
    let json = serde_json::Value::from_str(text).unwrap();
    let parsed = parse_layout_from_json(&json).unwrap();
    use crate::fancy_keys::Row::*;
    assert_eq!(parsed, Layout {
      mappings: vec![
        Mapping::Alias(AliasMapping { from: AliasFromKeys { keys: vec![CAPSLOCK] }, to: AliasToKeys { initial: vec![], terminal: "@symbol".to_owned() } }),
        Mapping::Alias(AliasMapping { from: AliasFromKeys { keys: vec![RIGHTALT] }, to: AliasToKeys { initial: vec![], terminal: "@symbol".to_owned() } }),
      
        Mapping::Row(RowMapping { from: RowFromKeys { modifiers: vec![Modifier::Alias("@symbol".to_owned())], row: USQuertyQ }, to: RowToKeys { initial: vec![], terminal: " {}% \\*][|".to_owned() }, repeat: RowRepeat::Normal, absorbing: vec![] }),
        Mapping::Row(RowMapping { from: RowFromKeys { modifiers: vec![Modifier::Alias("@symbol".to_owned())], row: USQuertyA }, to: RowToKeys { initial: vec![], terminal: "   = &)(/_$".to_owned() }, repeat: RowRepeat::Normal, absorbing: vec![] }),
        Mapping::Row(RowMapping { from: RowFromKeys { modifiers: vec![Modifier::Alias("@symbol".to_owned())], row: USQuertyZ }, to: RowToKeys { initial: vec![], terminal: "\"    !+#".to_owned() }, repeat: RowRepeat::Normal, absorbing: vec![] }),
      ]
    });
  }

  #[test]
  fn test_parsing_3() {
    let text = r#"{
  "mappings": [
    {"from":["COMMA"], "to":["W"], "repeat":{"Special":{"keys":["LEFTCTRL","F24"], "delay_ms":180, "interval_ms":30}}, "absorbing":[]}
  ]
}"#;
    let json = serde_json::Value::from_str(text).unwrap();
    let parsed = parse_layout_from_json(&json).unwrap();
    assert_eq!(parsed, Layout {
      mappings: vec![
        Mapping::Single(SingleMapping { from: SingleFromKeys { modifiers: vec![], key: COMMA }, to: SingleToKeys { initial: vec![], terminal: SingleTerminalToKey::Physical(W) }, repeat: SingleRepeat::Special { keys: SingleToKeys { initial: vec![Modifier::Key(LEFTCTRL)], terminal: SingleTerminalToKey::Physical(F24) }, delay_ms: 180, interval_ms: 30 }, absorbing: vec![] })
      ]
    });
  }

  #[test]
  fn test_formatting_1() {
    let text = r#"{
  "mappings": [
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" },
    { "from": ["@symbol", {"row": "Q"}], "to": {"letters": " {}% \\*][|"} },
    { "from": ["@symbol", {"row": "A"}], "to": {"letters": "   = &)(/_$"} },
    { "from": ["@symbol", {"row": "Z"}], "to": {"letters": "\"    !+#"} }
  ]
}"#;
    let json = serde_json::Value::from_str(text).unwrap();
    let restringed1 = json.to_string();
    let layout = parse_layout_from_json(&json).unwrap();
    let formatted = format_layout_as_json(&layout);
    let restringed2 = formatted.to_string();

    if restringed1 != restringed2 {
      println!("{}", restringed1);
      println!("{}", restringed2);
    }
    assert_eq!(restringed1, restringed2);
  }

  #[test]
  fn test_formatting_2() {
    let text = r#"{
  "mappings": [
    {"from":"COMMA", "to":"W", "repeat":{"Special":{"keys":["LEFTCTRL","F24"], "delay_ms":180, "interval_ms":30}}}
  ]
}"#;
    let json = serde_json::Value::from_str(text).unwrap();
    let restringed1 = json.to_string();
    let layout = parse_layout_from_json(&json).unwrap();
    let formatted = format_layout_as_json(&layout);
    let restringed2 = formatted.to_string();

    if restringed1 != restringed2 {
      println!("{}", restringed1);
      println!("{}", restringed2);
    }
    assert_eq!(restringed1, restringed2);
  }
}

