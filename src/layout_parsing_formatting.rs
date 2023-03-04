
use std::str::FromStr;
use key_codes::KeyCode;
use serde_json::{Value, Map};
use Value::{Object, Array};
use crate::{fancy_keys::{Layout, Mapping, Modifier, FromKeys, FromKey, ToKeys, TerminalToKey, Repeat}, key_codes};
use serde_json::Value as j;
use serde_json::json;

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
  if let j::Array(to_elems) = to_v {
    parse_to_array(to_elems)
  }
  else {
    Ok(ToKeys {
      initial: vec![],
      terminal: parse_to_terminal(to_v)?
    })
  }
}

fn parse_to_terminal(to_v: &Value) -> Result<TerminalToKey, String> {
  if let j::String(to_text) = to_v {
    parse_to_text(to_text)
  }
  else if let j::Object(to_attrs) = to_v {
    parse_to_obj(to_attrs)
  }
  else {
    Err(format!("`to` should be a string, array, or object; found {}", to_v))
  }
  
}

fn parse_to_text(to_text: &str) -> Result<TerminalToKey, String> {
  if to_text.starts_with("@") {
    Ok(TerminalToKey::Alias(to_text.to_owned()))
  }
  else {
    Ok(TerminalToKey::Physical(KeyCode::from_str(&to_text).map_err(|_| format!("Unknown key code: {}", to_text))?))
  }
}

fn parse_to_array(to_elems: &[Value]) -> Result<ToKeys, String> {
  if to_elems.len() == 0 {
    Ok(ToKeys {
      initial: vec![],
      terminal: TerminalToKey::Null
    })
  }
  else {
    Ok(ToKeys {
      initial: parse_to_initial(&to_elems[0..to_elems.len()-1])?,
      terminal: parse_to_terminal(&to_elems[to_elems.len()-1])?
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

fn parse_to_obj(to_attrs: &Map<String, Value>) -> Result<TerminalToKey, String> {
  if has_exactly_keys(to_attrs, &vec!["letters"]) {
    let letters = to_attrs.get("letters").unwrap();
    if let j::String(letters_text) = letters {
      Ok(TerminalToKey::Letters(letters_text.to_owned()))
    }
    else {
      Err(format!("`letters` must be a string, found {}", letters))
    }
  }
  else {
    Err(format!("`to` object of unrecognized form {}, expected, for example, `letters`", j::Object(to_attrs.clone())))
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

fn parse_repeat(v: &Option<&Value>) -> Result<Repeat, String> {
  if let Some(v) = v {
    if let j::String(text) = v {
      if text.to_lowercase() == "normal" {
        Ok(Repeat::Normal)
      }
      else if text.to_lowercase() == "disabled" {
        Ok(Repeat::Disabled)
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
            
            Ok(Repeat::Special {
              keys: parse_repeat_keys(keys)?,
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
    Ok(Repeat::Normal)
  }
}

fn parse_repeat_keys(v: &Value) -> Result<ToKeys, String> {
  parse_to(v)
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

pub fn format_layout_as_json(layout: &Layout) -> Value {
  let mappings: Vec<Value> = layout.mappings.iter().map(format_mapping).collect();
  
  let mut keys = Map::new();
  keys.insert("mappings".to_owned(), j::Array(mappings));
  
  j::Object(keys)
}

fn format_mapping(mapping: &Mapping) -> Value {
  let mut keys = Map::new();
  
  keys.insert("from".to_owned(), format_from(&mapping.from));
  keys.insert("to".to_owned(), format_to(&mapping.to));
  if let Some(repeat) = format_repeat(&mapping.repeat) {
    keys.insert("repeat".to_owned(), repeat);
  }
  if let Some(absorbing) = format_absorbing(&mapping.absorbing) {
    keys.insert("absorbing".to_owned(), absorbing);
  }
  
  j::Object(keys)
}

fn format_from(from: &FromKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &from.modifiers {
    elems.push(format_modifier(m));
  }
  
  elems.push(format_key(&from.key));
  
  if elems.len() == 1 {
    elems.remove(0)
  }
  else {
    j::Array(elems)
  }
}

fn format_modifier(m: &Modifier) -> Value {
  match m {
    Modifier::Key(k) => j::String(format!("{}", k)),
    Modifier::Alias(a) => j::String(a.clone())
  }
}

fn format_key(k: &FromKey) -> Value {
  match k {
    FromKey::Single(k) => j::String(format!("{}", k)),
    FromKey::Row(row) => format_row(row)
  }
}

fn format_row(row: &str) -> Value {
  let mut keys = Map::new();

  keys.insert("row".to_owned(), j::String(row.to_owned()));
  
  j::Object(keys)
}

fn format_to(to: &ToKeys) -> Value {
  let mut elems = Vec::new();
  
  for m in &to.initial {
    elems.push(format_modifier(m));
  }
  
  match &to.terminal {
    TerminalToKey::Null => elems.clear(),
    TerminalToKey::Physical(k) => elems.push(j::String(format!("{}", k))),
    TerminalToKey::Alias(s) => elems.push(j::String(s.clone())),
    TerminalToKey::Letters(s) => elems.push(format_letters(&s))
  }
  
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

fn format_repeat(repeat: &Repeat) -> Option<Value> {
  match repeat {
    Repeat::Normal => None,
    Repeat::Disabled => Some(j::String("Disabled".to_owned())),
    Repeat::Special { keys, delay_ms, interval_ms } => Some(format_repeat_special(keys, *delay_ms, *interval_ms))
  }
}

fn format_repeat_special(keys: &ToKeys, delay_ms: i32, interval_ms: i32) -> Value {
  let mut elems1 = Map::new();
  let mut elems2 = Map::new();
  
  elems2.insert("keys".to_owned(), format_to(keys));
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
    
    Some(j::Array(res))
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;
  use crate::fancy_keys::{Layout, Mapping, FromKeys, FromKey, Modifier, ToKeys, TerminalToKey, Repeat};
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
        Mapping { from: FromKeys { modifiers: vec![], key: FromKey::Single(CAPSLOCK) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Alias("@symbol".to_owned()) }, repeat: Repeat::Normal, absorbing: vec![] },
        Mapping { from: FromKeys { modifiers: vec![], key: FromKey::Single(RIGHTALT) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Alias("@symbol".to_owned()) }, repeat: Repeat::Normal, absorbing: vec![] }
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
    assert_eq!(parsed, Layout {
      mappings: vec![
        Mapping { from: FromKeys { modifiers: vec![], key: FromKey::Single(CAPSLOCK) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Alias("@symbol".to_owned()) }, repeat: Repeat::Normal, absorbing: vec![] },
        Mapping { from: FromKeys { modifiers: vec![], key: FromKey::Single(RIGHTALT) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Alias("@symbol".to_owned()) }, repeat: Repeat::Normal, absorbing: vec![] },
      
        Mapping { from: FromKeys { modifiers: vec![Modifier::Alias("@symbol".to_owned())], key: FromKey::Row("Q".to_owned()) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Letters(" {}% \\*][|".to_owned()) }, repeat: Repeat::Normal, absorbing: vec![] },
        Mapping { from: FromKeys { modifiers: vec![Modifier::Alias("@symbol".to_owned())], key: FromKey::Row("A".to_owned()) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Letters("   = &)(/_$".to_owned()) }, repeat: Repeat::Normal, absorbing: vec![] },
        Mapping { from: FromKeys { modifiers: vec![Modifier::Alias("@symbol".to_owned())], key: FromKey::Row("Z".to_owned()) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Letters("\"    !+#".to_owned()) }, repeat: Repeat::Normal, absorbing: vec![] },
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
        Mapping { from: FromKeys { modifiers: vec![], key: FromKey::Single(COMMA) }, to: ToKeys { initial: vec![], terminal: TerminalToKey::Physical(W) }, repeat: Repeat::Special { keys: ToKeys { initial: vec![Modifier::Key(LEFTCTRL)], terminal: TerminalToKey::Physical(F24) }, delay_ms: 180, interval_ms: 30 }, absorbing: vec![] }
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

