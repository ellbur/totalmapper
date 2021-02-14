
// vim: shiftwidth=2
 
use serde::{Deserialize, Serialize};
pub use crate::key_codes::KeyCode; 

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
  Pressed(KeyCode),
  Released(KeyCode)
}

pub use Event::Pressed;
pub use Event::Released;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
  pub from: Vec<KeyCode>,
  pub to: Vec<KeyCode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layout {
  pub mappings: Vec<Mapping>,
  #[serde(default = "Vec::new")]
  pub no_repeat_keys: Vec<KeyCode>
}

