
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
pub struct Mapping {
  pub from: FromKeys,
  pub to: ToKeys,
  pub repeat: Repeat,
  pub absorbing: Vec<Modifier>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromKeys {
  pub modifiers: Vec<Modifier>,
  pub key: FromKey
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
  Key(KeyCode),
  Alias(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromKey {
  Single(KeyCode),
  Row(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToKeys {
  pub initial: Vec<Modifier>,
  pub terminal: TerminalToKey
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalToKey {
  Physical(KeyCode),
  Alias(String),
  Letters(String),
  Null
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Repeat {
  Normal,
  Disabled,
  Special {
    keys: ToKeys,
    delay_ms: i32,
    interval_ms: i32
  }
}

