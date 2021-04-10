
// Convenience functions for creating mappings for 
// US keyboards

// vim: shiftwidth=2

use crate::keys::{KeyCode, Mapping, Repeat};
use std::collections::HashMap;
use KeyCode::*;
use lazy_static::lazy_static;
use std::default::Default;

pub struct USKeyboardLayout {
  pub tilde: char,
  pub tilde_shift: char,
  pub tilde_alt_gr: char,
  pub tilde_shift_alt_gr: char,
  
  pub row_1: String,
  pub row_1_shift: String,
  pub row_1_alt_gr: String,
  pub row_1_shift_alt_gr: String,
  
  pub row_q: String,
  pub row_q_shift: String,
  pub row_q_alt_gr: String,
  pub row_q_shift_alt_gr: String,
  
  pub row_a: String,
  pub row_a_shift: String,
  pub row_a_alt_gr: String,
  pub row_a_shift_alt_gr: String,
  
  pub row_z: String,
  pub row_z_shift: String,
  pub row_z_alt_gr: String,
  pub row_z_shift_alt_gr: String,
}

lazy_static! {
  static ref CHAR_ACCESS_MAP: HashMap<char, SinkKey> = _char_access_map();
}

struct SinkKey {
  sh: bool,
  k: KeyCode
}

fn adapt(sink_key: &SinkKey, shift_key: &KeyCode) -> Vec<KeyCode> {
  if !sink_key.sh {
    vec![sink_key.k]
  }
  else if *shift_key == LEFTSHIFT {
    vec![LEFTSHIFT, sink_key.k]
  }
  else if *shift_key == RIGHTSHIFT {
    vec![RIGHTSHIFT, sink_key.k]
  }
  else {
    vec![LEFTSHIFT, sink_key.k]
  }
}

fn _char_access_map() -> HashMap<char, SinkKey> {
  let mut res = HashMap::new();
  
  res.insert('0', SinkKey { sh: false, k: K0 });
  res.insert('1', SinkKey { sh: false, k: K1 });
  res.insert('2', SinkKey { sh: false, k: K2 });
  res.insert('3', SinkKey { sh: false, k: K3 });
  res.insert('4', SinkKey { sh: false, k: K4 });
  res.insert('5', SinkKey { sh: false, k: K5 });
  res.insert('6', SinkKey { sh: false, k: K6 });
  res.insert('7', SinkKey { sh: false, k: K7 });
  res.insert('8', SinkKey { sh: false, k: K8 });
  res.insert('9', SinkKey { sh: false, k: K9 });
  
  res.insert('a', SinkKey { sh: false, k: A });
  res.insert('b', SinkKey { sh: false, k: B });
  res.insert('c', SinkKey { sh: false, k: C });
  res.insert('d', SinkKey { sh: false, k: D });
  res.insert('e', SinkKey { sh: false, k: E });
  res.insert('f', SinkKey { sh: false, k: F });
  res.insert('g', SinkKey { sh: false, k: G });
  res.insert('h', SinkKey { sh: false, k: H });
  res.insert('i', SinkKey { sh: false, k: I });
  res.insert('j', SinkKey { sh: false, k: J });
  res.insert('k', SinkKey { sh: false, k: K });
  res.insert('l', SinkKey { sh: false, k: L });
  res.insert('m', SinkKey { sh: false, k: M });
  res.insert('n', SinkKey { sh: false, k: N });
  res.insert('o', SinkKey { sh: false, k: O });
  res.insert('p', SinkKey { sh: false, k: P });
  res.insert('q', SinkKey { sh: false, k: Q });
  res.insert('r', SinkKey { sh: false, k: R });
  res.insert('s', SinkKey { sh: false, k: S });
  res.insert('t', SinkKey { sh: false, k: T });
  res.insert('u', SinkKey { sh: false, k: U });
  res.insert('v', SinkKey { sh: false, k: V });
  res.insert('w', SinkKey { sh: false, k: W });
  res.insert('x', SinkKey { sh: false, k: X });
  res.insert('y', SinkKey { sh: false, k: Y });
  res.insert('z', SinkKey { sh: false, k: Z });
  
  res.insert('A', SinkKey { sh: true, k: A });
  res.insert('B', SinkKey { sh: true, k: B });
  res.insert('C', SinkKey { sh: true, k: C });
  res.insert('D', SinkKey { sh: true, k: D });
  res.insert('E', SinkKey { sh: true, k: E });
  res.insert('F', SinkKey { sh: true, k: F });
  res.insert('G', SinkKey { sh: true, k: G });
  res.insert('H', SinkKey { sh: true, k: H });
  res.insert('I', SinkKey { sh: true, k: I });
  res.insert('J', SinkKey { sh: true, k: J });
  res.insert('K', SinkKey { sh: true, k: K });
  res.insert('L', SinkKey { sh: true, k: L });
  res.insert('M', SinkKey { sh: true, k: M });
  res.insert('N', SinkKey { sh: true, k: N });
  res.insert('O', SinkKey { sh: true, k: O });
  res.insert('P', SinkKey { sh: true, k: P });
  res.insert('Q', SinkKey { sh: true, k: Q });
  res.insert('R', SinkKey { sh: true, k: R });
  res.insert('S', SinkKey { sh: true, k: S });
  res.insert('T', SinkKey { sh: true, k: T });
  res.insert('U', SinkKey { sh: true, k: U });
  res.insert('V', SinkKey { sh: true, k: V });
  res.insert('W', SinkKey { sh: true, k: W });
  res.insert('X', SinkKey { sh: true, k: X });
  res.insert('Y', SinkKey { sh: true, k: Y });
  res.insert('Z', SinkKey { sh: true, k: Z });
  
  res.insert('!', SinkKey { sh: true, k: K1 });
  res.insert('@', SinkKey { sh: true, k: K2 });
  res.insert('#', SinkKey { sh: true, k: K3 });
  res.insert('$', SinkKey { sh: true, k: K4 });
  res.insert('%', SinkKey { sh: true, k: K5 });
  res.insert('^', SinkKey { sh: true, k: K6 });
  res.insert('&', SinkKey { sh: true, k: K7 });
  res.insert('*', SinkKey { sh: true, k: K8 });
  res.insert('(', SinkKey { sh: true, k: K9 });
  res.insert(')', SinkKey { sh: true, k: K0 });
  
  res.insert(',',  SinkKey { sh: false, k: COMMA });
  res.insert('.',  SinkKey { sh: false, k: DOT });
  res.insert('`',  SinkKey { sh: false, k: GRAVE });
  res.insert('-',  SinkKey { sh: false, k: MINUS });
  res.insert('=',  SinkKey { sh: false, k: EQUAL });
  res.insert('[',  SinkKey { sh: false, k: LEFTBRACE });
  res.insert(']',  SinkKey { sh: false, k: RIGHTBRACE });
  res.insert(';',  SinkKey { sh: false, k: SEMICOLON });
  res.insert('\'', SinkKey { sh: false, k: APOSTROPHE });
  res.insert('/',  SinkKey { sh: false, k: SLASH });
  res.insert('\\', SinkKey { sh: false, k: BACKSLASH });
  
  res.insert('~',  SinkKey { sh: true, k: GRAVE });
  res.insert('_',  SinkKey { sh: true, k: MINUS });
  res.insert('+',  SinkKey { sh: true, k: EQUAL });
  res.insert('{',  SinkKey { sh: true, k: LEFTBRACE });
  res.insert('}',  SinkKey { sh: true, k: RIGHTBRACE });
  res.insert(':',  SinkKey { sh: true, k: SEMICOLON });
  res.insert('"',  SinkKey { sh: true, k: APOSTROPHE });
  res.insert('<',  SinkKey { sh: true, k: COMMA });
  res.insert('>',  SinkKey { sh: true, k: DOT });
  res.insert('?',  SinkKey { sh: true, k: SLASH });
  res.insert('|',  SinkKey { sh: true, k: BACKSLASH });
  
  res
}

pub struct USKeyCodeMap {
  tilde: KeyCode,
  row_1: Vec<KeyCode>,
  row_q: Vec<KeyCode>,
  row_a: Vec<KeyCode>,
  row_z: Vec<KeyCode>,
}

lazy_static! {
  static ref US_KEYCODES: USKeyCodeMap = _us_keycodes();
}

fn _us_keycodes() -> USKeyCodeMap {
  USKeyCodeMap {
    tilde: GRAVE,
    row_1: vec![K1, K2, K3, K4, K5, K6, K7, K8, K9, K0, MINUS, EQUAL],
    row_q: vec![Q, W, E, R, T, Y, U, I, O, P, LEFTBRACE, RIGHTBRACE],
    row_a: vec![A, S, D, F, G, H, J, K, L, SEMICOLON, APOSTROPHE],
    row_z: vec![Z, X, C, V, B, N, M, COMMA, DOT, SLASH],
  }
}

lazy_static! {
  static ref US_SHIFT_KEYS: Vec<KeyCode> = vec![LEFTSHIFT, RIGHTSHIFT];
}

fn string_mappings(hardware_keys: &Vec<KeyCode>, desired_chars: String, shift_down: bool, alt_gr_down: bool, shift_keys: &Vec<KeyCode>, alt_gr_keys: &Vec<KeyCode>, disable_repeats: bool, absorb_shift: bool) -> Vec<Mapping> {
  let mut key_iter = hardware_keys.iter();
  let mut char_iter = desired_chars.chars();
  
  let mut res = Vec::new();
  
  let repeat = {
    if disable_repeats {
      Repeat::Disabled
    }
    else {
      Repeat::Normal
    }
  };
  
  let shift_absorbing = |sk: KeyCode| {
    if absorb_shift {
      vec![sk]
    }
    else {
      vec![]
    }
  };
  
  loop {
    let next_key = key_iter.next();
    let next_char = char_iter.next();
    
    match next_char {
      None => break,
      Some(ch) => match next_key {
        None => panic!("More chars than keys in the row"),
        Some(k) => {
          if ch != ' ' {
            match CHAR_ACCESS_MAP.get(&ch) {
              None => panic!("Don't know how to type char {:?}", ch),
              Some(sink_key) => {
                if shift_down {
                  if alt_gr_down {
                    for sk in shift_keys {
                      for ak in alt_gr_keys {
                        res.push(Mapping { from: vec![*sk, *ak, *k], to: adapt(sink_key, sk), repeat: repeat.clone(), absorbing: shift_absorbing(*sk), ..Default::default() });
                      }
                    }
                  }
                  else {
                    for sk in shift_keys {
                      res.push(Mapping { from: vec![*sk, *k], to: adapt(sink_key, sk), repeat: repeat.clone(), absorbing: shift_absorbing(*sk), ..Default::default() });
                    }
                  }
                }
                else {
                  if alt_gr_down {
                    for ak in alt_gr_keys {
                      res.push(Mapping { from: vec![*ak, *k], to: vec![sink_key.k], repeat: repeat.clone(), ..Default::default() });
                    }
                  }
                  else {
                    res.push(Mapping { from: vec![*k], to: vec![sink_key.k], repeat: repeat.clone(), ..Default::default() });
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  
  res
}

pub fn make_us_mappings(layout: USKeyboardLayout, alt_gr_keys: &Vec<KeyCode>, disable_repeats: bool, absorb_shift: bool) -> Vec<Mapping> {
  let shift_keys = &*US_SHIFT_KEYS;
  let us_keycodes = &*US_KEYCODES;
  
  let mut mappings = Vec::new();
  
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde.to_string(),              false, false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde_shift.to_string(),        true,  false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde_alt_gr.to_string(),       false, true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde_shift_alt_gr.to_string(), true,  true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1,              false, false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1_shift,        true,  false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1_alt_gr,       false, true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q,              false, false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q_shift,        true,  false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q_alt_gr,       false, true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a,              false, false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a_shift,        true,  false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a_alt_gr,       false, true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z,              false, false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z_shift,        true,  false, &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z_alt_gr,       false, true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys, disable_repeats, absorb_shift));
  
  mappings
}

