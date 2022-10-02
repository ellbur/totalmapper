
// vim: shiftwidth=2
 
pub use crate::key_codes::KeyCode; 
use std::default::Default;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
  Pressed(KeyCode),
  Released(KeyCode)
}

pub use Event::Pressed;
pub use Event::Released;

#[derive(Debug, Clone)]
pub enum Mapping {
  Basic(BasicMapping),
  AliasDefinition(AliasDefinitionMapping),
  RowToLetters(RowToLettersMapping)
}

#[derive(Debug, Clone)]
pub enum Modifier {
  Key(KeyCode),
  Alias(String)
}

#[derive(Debug, Clone)]
pub struct BasicMapping {
  pub from_modifiers: Vec<Modifier>,
  pub from_key: KeyCode,
  pub to: Vec<KeyCode>,
  pub repeat: Repeat,
  pub absorbing: Vec<KeyCode>
}

impl Default for BasicMapping {
  fn default() -> Self {
    BasicMapping {
      from_modifiers: vec![],
      from_key: KeyCode::SPACE,
      to: vec![],
      repeat: Repeat::Normal,
      absorbing: vec![]
    }
  }
}

#[derive(Debug, Clone)]
pub struct AliasDefinitionMapping {
  pub from_modifiers: Vec<Modifier>,
  pub to_also_keys: Vec<KeyCode>,
  pub resulting_modifier: String
}

#[derive(Debug, Clone)]
pub struct RowToLettersMapping {
  pub from_modifiers: Vec<Modifier>,
  pub from_row_first_key: String,
  pub to_letters: String
}

#[derive(Debug, Clone)]
pub enum Repeat {
  Normal,
  Disabled,
  Special {
    keys: Vec<KeyCode>,
    delay_ms: i32,
    interval_ms: i32
  }
}

pub fn normal_repeat() -> Repeat {
  Repeat::Normal
}

#[derive(Debug, Clone)]
pub struct Layout {
  pub mappings: Vec<Mapping>
}

