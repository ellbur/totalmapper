
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

pub enum Mapping {
  Basic(BasicMapping),
  AliasDefinition(AliasDefinitionMapping),
  RowToLetters(RowToLettersMapping)
}

pub enum Modifier {
  KeyModifier(KeyCode),
  AliasModifier(String)
}

#[derive(Debug, Clone)]
pub struct BasicMapping {
  pub fromModifiers: Vec<Modifier>,
  pub fromKey: KeyCode,
  pub to: Vec<KeyCode>,
  pub repeat: Repeat,
  pub absorbing: Vec<KeyCode>
}

impl Default for BasicMapping {
  fn default() -> Self {
    BasicMapping {
      from: vec![],
      to: vec![],
      repeat: Repeat::Normal,
      absorbing: vec![]
    }
  }
}

#[derive(Debug, Clone)]
pub struct AliasDefinitionMapping {
  pub fromModifiers: Vec<Modifier>,
  pub toAlsoKeys: Vec<KeyCode>,
  pub resultingModifier: String
}

#[derive(Debug, Clone)]
pub struct RowToLettersMapping {
  pub fromModifiers: Vec<Modifier>,
  pub fromRowFirstKey: String,
  pub toLetters: String
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

