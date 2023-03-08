
// vim: shiftwidth=2
 
pub use crate::key_codes::KeyCode; 
pub use crate::events::Event;
pub use Event::Pressed;
pub use Event::Released;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
  pub mappings: Vec<Mapping>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mapping {
  Single(SingleMapping),
  Alias(AliasMapping),
  Row(RowMapping)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasMapping {
  pub from: AliasFromKeys,
  pub to: AliasToKeys
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleMapping {
  pub from: SingleFromKeys,
  pub to: SingleToKeys,
  pub repeat: SingleRepeat,
  pub absorbing: Vec<Modifier>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowMapping {
  pub from: RowFromKeys,
  pub to: RowToKeys,
  pub repeat: RowRepeat,
  pub absorbing: Vec<Modifier>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleFromKeys {
  pub modifiers: Vec<Modifier>,
  pub key: KeyCode
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasFromKeys {
  pub keys: Vec<KeyCode>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowFromKeys {
  pub modifiers: Vec<Modifier>,
  pub row: Row
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
  USQuertyGrave,
  USQuerty1,
  USQuertyQ,
  USQuertyA,
  USQuertyZ,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
  Key(KeyCode),
  Alias(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleToKeys {
  pub initial: Vec<Modifier>,
  pub terminal: SingleTerminalToKey
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasToKeys {
  pub initial: Vec<KeyCode>,
  pub terminal: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowToKeys {
  pub initial: Vec<Modifier>,
  pub terminal: RowTerminalToKey
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleTerminalToKey {
  Physical(KeyCode),
  Null
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowTerminalToKey {
  Letters(String),
  Null
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleRepeat {
  Normal,
  Disabled,
  Special {
    keys: SingleToKeys,
    delay_ms: i32,
    interval_ms: i32
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowRepeat {
  Normal,
  Disabled,
  Special {
    keys: RowToKeys,
    delay_ms: i32,
    interval_ms: i32
  }
}

