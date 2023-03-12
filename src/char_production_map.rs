
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::keys::KeyCode;
use KeyCode::*;

lazy_static! {
  pub static ref CHAR_ACCESS_MAP: HashMap<char, SinkKey> = _char_access_map();
}

pub struct SinkKey {
  pub sh: bool,
  pub k: KeyCode
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


