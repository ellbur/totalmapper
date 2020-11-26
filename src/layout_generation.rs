
// Convenience functions for creating mappings for 
// US keyboards

// vim: shiftwidth=2

use crate::keys::{KeyCode, Mapping};
use std::collections::HashMap;
use KeyCode::*;
use lazy_static::lazy_static;

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
  static ref CHAR_ACCESS_MAP: HashMap<char, Vec<KeyCode>> = _char_access_map();
}

fn _char_access_map() -> HashMap<char, Vec<KeyCode>> {
  let mut res = HashMap::new();
  
  res.insert('0', vec![K0]);
  res.insert('1', vec![K1]);
  res.insert('2', vec![K2]);
  res.insert('3', vec![K3]);
  res.insert('4', vec![K4]);
  res.insert('5', vec![K5]);
  res.insert('6', vec![K6]);
  res.insert('7', vec![K7]);
  res.insert('8', vec![K8]);
  res.insert('9', vec![K9]);
  
  res.insert('a', vec![A]);
  res.insert('b', vec![B]);
  res.insert('c', vec![C]);
  res.insert('d', vec![D]);
  res.insert('e', vec![E]);
  res.insert('f', vec![F]);
  res.insert('g', vec![G]);
  res.insert('h', vec![H]);
  res.insert('i', vec![I]);
  res.insert('j', vec![J]);
  res.insert('k', vec![K]);
  res.insert('l', vec![L]);
  res.insert('m', vec![M]);
  res.insert('n', vec![N]);
  res.insert('o', vec![O]);
  res.insert('p', vec![P]);
  res.insert('q', vec![Q]);
  res.insert('r', vec![R]);
  res.insert('s', vec![S]);
  res.insert('t', vec![T]);
  res.insert('u', vec![U]);
  res.insert('v', vec![V]);
  res.insert('w', vec![W]);
  res.insert('x', vec![X]);
  res.insert('y', vec![Y]);
  res.insert('z', vec![Z]);
  
  res.insert('A', vec![LEFTSHIFT, A]);
  res.insert('B', vec![LEFTSHIFT, B]);
  res.insert('C', vec![LEFTSHIFT, C]);
  res.insert('D', vec![LEFTSHIFT, D]);
  res.insert('E', vec![LEFTSHIFT, E]);
  res.insert('F', vec![LEFTSHIFT, F]);
  res.insert('G', vec![LEFTSHIFT, G]);
  res.insert('H', vec![LEFTSHIFT, H]);
  res.insert('I', vec![LEFTSHIFT, I]);
  res.insert('J', vec![LEFTSHIFT, J]);
  res.insert('K', vec![LEFTSHIFT, K]);
  res.insert('L', vec![LEFTSHIFT, L]);
  res.insert('M', vec![LEFTSHIFT, M]);
  res.insert('N', vec![LEFTSHIFT, N]);
  res.insert('O', vec![LEFTSHIFT, O]);
  res.insert('P', vec![LEFTSHIFT, P]);
  res.insert('Q', vec![LEFTSHIFT, Q]);
  res.insert('R', vec![LEFTSHIFT, R]);
  res.insert('S', vec![LEFTSHIFT, S]);
  res.insert('T', vec![LEFTSHIFT, T]);
  res.insert('U', vec![LEFTSHIFT, U]);
  res.insert('V', vec![LEFTSHIFT, V]);
  res.insert('W', vec![LEFTSHIFT, W]);
  res.insert('X', vec![LEFTSHIFT, X]);
  res.insert('Y', vec![LEFTSHIFT, Y]);
  res.insert('Z', vec![LEFTSHIFT, Z]);
  
  res.insert('!', vec![LEFTSHIFT, K1]);
  res.insert('@', vec![LEFTSHIFT, K2]);
  res.insert('#', vec![LEFTSHIFT, K3]);
  res.insert('$', vec![LEFTSHIFT, K4]);
  res.insert('%', vec![LEFTSHIFT, K5]);
  res.insert('^', vec![LEFTSHIFT, K6]);
  res.insert('&', vec![LEFTSHIFT, K7]);
  res.insert('*', vec![LEFTSHIFT, K8]);
  res.insert('(', vec![LEFTSHIFT, K9]);
  res.insert(')', vec![LEFTSHIFT, K0]);
  
  res.insert('~',  vec![LEFTSHIFT, GRAVE]);
  res.insert('`',  vec![GRAVE]);
  res.insert('-',  vec![MINUS]);
  res.insert('_',  vec![LEFTSHIFT, MINUS]);
  res.insert('=',  vec![EQUAL]);
  res.insert('+',  vec![LEFTSHIFT, EQUAL]);
  res.insert('[',  vec![LEFTBRACE]);
  res.insert(']',  vec![RIGHTBRACE]);
  res.insert('{',  vec![LEFTSHIFT, LEFTBRACE]);
  res.insert('}',  vec![RIGHTSHIFT, RIGHTBRACE]);
  res.insert(';',  vec![SEMICOLON]);
  res.insert(':',  vec![LEFTSHIFT, SEMICOLON]);
  res.insert('\'', vec![APOSTROPHE]);
  res.insert('"',  vec![LEFTSHIFT, APOSTROPHE]);
  res.insert(',',  vec![COMMA]);
  res.insert('.',  vec![DOT]);
  res.insert('<',  vec![LEFTSHIFT, COMMA]);
  res.insert('>',  vec![LEFTSHIFT, DOT]);
  res.insert('/',  vec![SLASH]);
  res.insert('?',  vec![LEFTSHIFT, SLASH]);
  res.insert('\\', vec![BACKSLASH]);
  res.insert('|',  vec![LEFTSHIFT, BACKSLASH]);
  
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

fn string_mappings(hardware_keys: &Vec<KeyCode>, desired_chars: String, shift_down: bool, alt_gr_down: bool, shift_keys: &Vec<KeyCode>, alt_gr_keys: &Vec<KeyCode>) -> Vec<Mapping> {
  let mut key_iter = hardware_keys.iter();
  let mut char_iter = desired_chars.chars();
  
  let mut res = Vec::new();
  
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
              Some(keys) => {
                if shift_down {
                  if alt_gr_down {
                    for sk in shift_keys {
                      for ak in alt_gr_keys {
                        res.push(Mapping { from: vec![*sk, *ak, *k], to: keys.clone() });
                      }
                    }
                  }
                  else {
                    for sk in shift_keys {
                      res.push(Mapping { from: vec![*sk, *k], to: keys.clone() });
                    }
                  }
                }
                else {
                  if alt_gr_down {
                    for ak in alt_gr_keys {
                      res.push(Mapping { from: vec![*ak, *k], to: keys.clone() });
                    }
                  }
                  else {
                    res.push(Mapping { from: vec![*k], to: keys.clone() });
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

pub fn make_us_mappings(layout: USKeyboardLayout, alt_gr_keys: &Vec<KeyCode>) -> Vec<Mapping> {
  let shift_keys = &*US_SHIFT_KEYS;
  let us_keycodes = &*US_KEYCODES;
  
  let mut mappings = Vec::new();
  
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde.to_string(),              false, false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde_shift.to_string(),        true,  false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde_alt_gr.to_string(),       false, true,  &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&vec![us_keycodes.tilde], layout.tilde_shift_alt_gr.to_string(), true,  true,  &shift_keys, alt_gr_keys));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1,              false, false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1_shift,        true,  false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1_alt_gr,       false, true,  &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_1, layout.row_1_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q,              false, false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q_shift,        true,  false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q_alt_gr,       false, true,  &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_q, layout.row_q_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a,              false, false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a_shift,        true,  false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a_alt_gr,       false, true,  &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_a, layout.row_a_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys));
  
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z,              false, false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z_shift,        true,  false, &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z_alt_gr,       false, true,  &shift_keys, alt_gr_keys));
  mappings.append(&mut string_mappings(&us_keycodes.row_z, layout.row_z_shift_alt_gr, true,  true,  &shift_keys, alt_gr_keys));
  
  mappings
}

