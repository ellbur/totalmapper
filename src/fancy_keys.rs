
// vim: shiftwidth=2
 
pub use crate::key_codes::KeyCode; 

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
  Pressed(KeyCode),
  Released(KeyCode)
}

pub use Event::Pressed;
pub use Event::Released;

#[derive(Debug, Clone)]
pub struct Mapping {
  pub from: FromKeys,
  pub to: ToKeys,
  pub repeat: Repeat,
  pub absorbing: Vec<Modifier>
}

#[derive(Debug, Clone)]
pub struct FromKeys {
  pub modifiers: Vec<Modifier>,
  pub key: FromKey
}

#[derive(Debug, Clone)]
pub enum Modifier {
  Key(KeyCode),
  Alias(String)
}

#[derive(Debug, Clone)]
pub enum FromKey {
  Single(KeyCode),
  Row(String)
}

#[derive(Debug, Clone)]
pub struct ToKeys {
  pub initial: Vec<InitialToKey>,
  pub terminal: TerminalToKey
}

#[derive(Debug, Clone)]
pub enum InitialToKey {
  Physical(KeyCode),
  Alias(String)
}

#[derive(Debug, Clone)]
pub enum TerminalToKey {
  Physical(KeyCode),
  Alias(String),
  Letters(String)
}

#[derive(Debug, Clone)]
pub enum Repeat {
  Normal,
  Disabled,
  Special {
    keys: ToKeys,
    delay_ms: i32,
    interval_ms: i32
  }
}

#[derive(Debug, Clone)]
pub struct Layout {
  pub mappings: Vec<Mapping>
}

