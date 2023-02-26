
use crate::key_codes::KeyCode;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
  Pressed(KeyCode),
  Released(KeyCode)
}

