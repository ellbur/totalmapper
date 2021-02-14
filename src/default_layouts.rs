
// vim: shiftwidth=2

use std::collections::HashMap;
use crate::keys::{Layout, Mapping, KeyCode};
use KeyCode::*;
use crate::layout_generation::{USKeyboardLayout, make_us_mappings};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref DEFAULT_LAYOUTS: HashMap<String, &'static Layout> = {
    vec![
     ("caps-for-movement".to_string(), &*CAPS_LOCK_FOR_MOVEMENT),
     ("easy-symbols".to_string(), &*EASY_SYMBOLS),
     ("caps-q-for-esc".to_string(), &*CAPS_Q_FOR_ESC),
     ("easy-symbols-tab-for-movement".to_string(), &*EASY_SYMBOLS_TAB_FOR_MOVEMENT),
     ("super-dvorak".to_string(), &*SUPER_DVORAK),
    ].into_iter().collect()
  };
}
  
lazy_static! {
  pub static ref CAPS_LOCK_FOR_MOVEMENT: Layout = _caps_lock_for_movement();
  pub static ref EASY_SYMBOLS: Layout = _easy_symbols();
  pub static ref CAPS_Q_FOR_ESC: Layout = _caps_q_for_esc();
  pub static ref EASY_SYMBOLS_TAB_FOR_MOVEMENT: Layout = _easy_symbols_tab_for_movement();
  pub static ref SUPER_DVORAK: Layout = _super_dvorak();
}

fn _caps_lock_for_movement() -> Layout {
  Layout {
    mappings: vec![
      Mapping { from: vec![CAPSLOCK], to: vec![] },
      Mapping { from: vec![CAPSLOCK, J], to: vec![LEFT] }, 
      Mapping { from: vec![CAPSLOCK, I], to: vec![UP] }, 
      Mapping { from: vec![CAPSLOCK, K], to: vec![DOWN] }, 
      Mapping { from: vec![CAPSLOCK, L], to: vec![RIGHT] }, 
      Mapping { from: vec![CAPSLOCK, H], to: vec![HOME] }, 
      Mapping { from: vec![CAPSLOCK, SEMICOLON], to: vec![END] }, 
      Mapping { from: vec![CAPSLOCK, U], to: vec![PAGEUP] }, 
      Mapping { from: vec![CAPSLOCK, M], to: vec![PAGEDOWN] }, 
      Mapping { from: vec![CAPSLOCK, N], to: vec![LEFTCTRL, LEFT] }, 
      Mapping { from: vec![CAPSLOCK, COMMA], to: vec![LEFTCTRL, RIGHT] }
    ],
    no_repeat_keys: Vec::new()
  } 
}

fn _easy_symbols() -> Layout {
  let rows = USKeyboardLayout {
    tilde: ' ',
    tilde_shift: ' ',
    tilde_alt_gr: ' ',
    tilde_shift_alt_gr: ' ',
    
    row_1: "".to_string(),
    row_1_shift: "".to_string(),
    row_1_alt_gr: "".to_string(),
    row_1_shift_alt_gr: "".to_string(),
    
    row_q: "".to_string(),
    row_q_shift: "".to_string(),
    row_q_alt_gr: " {}% \\*][|".to_string(),
    row_q_shift_alt_gr: "".to_string(),
    
    row_a: "".to_string(),
    row_a_shift: "".to_string(),
    row_a_alt_gr: "   = &)(/_$".to_string(),
    row_a_shift_alt_gr: "".to_string(),
    
    row_z: "".to_string(),
    row_z_shift: "".to_string(),
    row_z_alt_gr: "     !+#".to_string(),
    row_z_shift_alt_gr: "".to_string(),
  };
  
  let mut char_mappings = make_us_mappings(rows, &vec![CAPSLOCK, RIGHTALT]);
  let mut other_mappings = vec![
    Mapping { from: vec![CAPSLOCK], to: vec![] },
  ];
  
  let mut mappings = Vec::new();
  mappings.append(&mut other_mappings);
  mappings.append(&mut char_mappings);
  
  Layout {
    mappings: mappings,
    no_repeat_keys: Vec::new()
  }
}

fn _caps_q_for_esc() -> Layout {
  Layout {
    mappings: vec![
      Mapping { from: vec![CAPSLOCK], to: vec![] },
      Mapping { from: vec![CAPSLOCK, Q], to: vec![ESC] }
    ],
    no_repeat_keys: Vec::new()
  }
}

fn _easy_symbols_tab_for_movement() -> Layout {
  let rows = USKeyboardLayout {
    tilde: ' ',
    tilde_shift: ' ',
    tilde_alt_gr: ' ',
    tilde_shift_alt_gr: ' ',
    
    row_1: "".to_string(),
    row_1_shift: "".to_string(),
    row_1_alt_gr: "".to_string(),
    row_1_shift_alt_gr: "".to_string(),
    
    row_q: "".to_string(),
    row_q_shift: "".to_string(),
    row_q_alt_gr: " {}% \\*][|".to_string(),
    row_q_shift_alt_gr: "".to_string(),
    
    row_a: "".to_string(),
    row_a_shift: "".to_string(),
    row_a_alt_gr: "   = &)(/_$".to_string(),
    row_a_shift_alt_gr: "".to_string(),
    
    row_z: "".to_string(),
    row_z_shift: "".to_string(),
    row_z_alt_gr: "     !+#".to_string(),
    row_z_shift_alt_gr: "".to_string(),
  };
  
  let mut char_mappings = make_us_mappings(rows, &vec![CAPSLOCK, RIGHTALT]);
  
  let mut other_mappings = vec![
    Mapping { from: vec![TAB], to: vec![] },
    Mapping { from: vec![TAB, J], to: vec![LEFT] }, 
    Mapping { from: vec![TAB, I], to: vec![UP] }, 
    Mapping { from: vec![TAB, K], to: vec![DOWN] }, 
    Mapping { from: vec![TAB, L], to: vec![RIGHT] }, 
    Mapping { from: vec![TAB, H], to: vec![HOME] }, 
    Mapping { from: vec![TAB, SEMICOLON], to: vec![END] }, 
    Mapping { from: vec![TAB, U], to: vec![PAGEUP] }, 
    Mapping { from: vec![TAB, M], to: vec![PAGEDOWN] }, 
    Mapping { from: vec![TAB, N], to: vec![LEFTCTRL, LEFT] }, 
    Mapping { from: vec![TAB, COMMA], to: vec![LEFTCTRL, RIGHT] },
    
    Mapping { from: vec![CAPSLOCK], to: vec![] },
    Mapping { from: vec![CAPSLOCK, Q], to: vec![ESC] },
    Mapping { from: vec![BACKSLASH], to: vec![TAB] },
  ];
  
  let mut all_mappings = Vec::new();
  all_mappings.append(&mut other_mappings);
  all_mappings.append(&mut char_mappings);
  
  Layout {
    mappings: all_mappings,
    no_repeat_keys: Vec::new()
  }
}

fn _super_dvorak() -> Layout {
  let rows = USKeyboardLayout {
    tilde: ' ',
    tilde_shift: ' ',
    tilde_alt_gr: ' ',
    tilde_shift_alt_gr: ' ',
    
    row_1: "17531902468`".to_string(),
    row_1_shift: "".to_string(),
    row_1_alt_gr: "".to_string(),
    row_1_shift_alt_gr: "".to_string(),
    
    row_q: ";,.pyf  rl~@".to_string(),
    row_q_shift: ":<>       ?^".to_string(),
    row_q_alt_gr: " {}% \\*][|".to_string(),
    row_q_shift_alt_gr: "".to_string(),
    
    row_a: "aoeui     -".to_string(),
    row_a_shift: "AOEUI     @".to_string(),
    row_a_alt_gr: "   = &)(/_$".to_string(),
    row_a_shift_alt_gr: "".to_string(),
    
    row_z: "'qjkx   vz".to_string(),
    row_z_shift: "\"QJKX   VZ".to_string(),
    row_z_alt_gr: "     !+#".to_string(),
    row_z_shift_alt_gr: "".to_string(),
  };
  
  let mut char_mappings = make_us_mappings(rows, &vec![CAPSLOCK, RIGHTALT, LEFTMETA]);
  
  let mut other_mappings = vec![
    Mapping { from: vec![U], to: vec![F14, G] },
    Mapping { from: vec![I], to: vec![F15, C] },
    Mapping { from: vec![H], to: vec![F16, D] },
    Mapping { from: vec![J], to: vec![F17, H] },
    Mapping { from: vec![K], to: vec![F18, T] },
    Mapping { from: vec![L], to: vec![F19, N] },
    Mapping { from: vec![SEMICOLON], to: vec![F20, S] },
    Mapping { from: vec![N], to: vec![F21, B] },
    Mapping { from: vec![M], to: vec![F22, M] },
    Mapping { from: vec![COMMA], to: vec![F23, W] },
    
    Mapping { from: vec![TAB], to: vec![] },
    Mapping { from: vec![TAB, J], to: vec![LEFT] }, 
    Mapping { from: vec![TAB, I], to: vec![UP] }, 
    Mapping { from: vec![TAB, K], to: vec![DOWN] }, 
    Mapping { from: vec![TAB, L], to: vec![RIGHT] }, 
    Mapping { from: vec![TAB, H], to: vec![HOME] }, 
    Mapping { from: vec![TAB, SEMICOLON], to: vec![END] }, 
    Mapping { from: vec![TAB, U], to: vec![PAGEUP] }, 
    Mapping { from: vec![TAB, M], to: vec![PAGEDOWN] }, 
    Mapping { from: vec![TAB, N], to: vec![LEFTCTRL, LEFT] }, 
    Mapping { from: vec![TAB, COMMA], to: vec![LEFTCTRL, RIGHT] },
    
    Mapping { from: vec![CAPSLOCK], to: vec![] },
    Mapping { from: vec![RIGHTALT], to: vec![] },
    Mapping { from: vec![LEFTMETA], to: vec![] },
    
    Mapping { from: vec![LEFTMETA, Q], to: vec![ESC] },
    Mapping { from: vec![RIGHTALT, Q], to: vec![ESC] },
    Mapping { from: vec![CAPSLOCK, Q], to: vec![ESC] },
    
    Mapping { from: vec![BACKSLASH], to: vec![TAB] },
    Mapping { from: vec![GRAVE], to: vec![LEFTMETA] },
  ];
  
  let mut all_mappings = Vec::new();
  all_mappings.append(&mut other_mappings);
  all_mappings.append(&mut char_mappings);
  
  Layout {
    mappings: all_mappings,
    no_repeat_keys: vec![GRAVE, K1, K2, K3, K4, K5, K6, K7, K8, K9, K0, MINUS, EQUAL, Q, W, E, R, T, Y, U, I, O, P, LEFTBRACE, RIGHTBRACE, BACKSLASH, A, S, D, F, G, H, J, K, L, SEMICOLON, APOSTROPHE, Z, X, C, V, B, N, M, COMMA, DOT, SLASH]
  }
}

