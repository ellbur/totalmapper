
use crate::{fancy_keys::Row, key_codes::KeyCode};
use lazy_static::lazy_static;
use std::collections::HashMap;
use KeyCode::*;

lazy_static! {
  pub static ref US_KEYBOARD_LAYOUT: HashMap<Row, &'static [KeyCode]> = _us_keyboard_layout();
}

lazy_static! {
  static ref US_ROW_GRAVE: Vec<KeyCode> = vec![GRAVE, K1, K2, K3, K4, K5, K6, K7, K8, K9, K0, MINUS, EQUAL];
  static ref US_ROW_Q: Vec<KeyCode> = vec![Q, W, E, R, T, Y, U, I, O, P, LEFTBRACE, RIGHTBRACE];
  static ref US_ROW_A: Vec<KeyCode> = vec![A, S, D, F, G, H, J, K, L, SEMICOLON, APOSTROPHE];
  static ref US_ROW_Z: Vec<KeyCode> = vec![Z, X, C, V, B, N, M, COMMA, DOT, SLASH];
}

fn _us_keyboard_layout() -> HashMap<Row, &'static [KeyCode]> {
  let mut res = HashMap::new();
  
  res.insert(Row::USQuertyGrave.clone(), &US_ROW_GRAVE[..]);
  res.insert(Row::USQuerty1.clone(), &US_ROW_GRAVE[1..]);
  res.insert(Row::USQuertyQ.clone(), &US_ROW_Q[..]);
  res.insert(Row::USQuertyA.clone(), &US_ROW_A[..]);
  res.insert(Row::USQuertyZ.clone(), &US_ROW_Z[..]);
  
  res
}

