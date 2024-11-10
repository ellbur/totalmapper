
use crate::key_codes::KeyCode;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
  Pressed(KeyCode),
  Released(KeyCode)
}

