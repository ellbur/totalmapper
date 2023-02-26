
// vim: shiftwidth=2
 
use serde::{Deserialize, Serialize};
pub use crate::key_codes::KeyCode; 
use std::default::Default;
pub use crate::events::Event;
pub use Event::Pressed;
pub use Event::Released;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
  pub from: Vec<KeyCode>,
  pub to: Vec<KeyCode>,
  #[serde(default = "normal_repeat")]
  pub repeat: Repeat,
  #[serde(default = "Vec::new")]
  pub absorbing: Vec<KeyCode>
}

impl Default for Mapping {
  fn default() -> Self {
    Mapping {
      from: vec![],
      to: vec![],
      repeat: Repeat::Normal,
      absorbing: vec![]
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layout {
  pub mappings: Vec<Mapping>
}

