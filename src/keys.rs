
// vim: shiftwidth=2
 
use serde::{Deserialize, Serialize};
pub use crate::key_codes::KeyCode; 

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout(pub Vec<Mapping>);

