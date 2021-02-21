
// vim: shiftwidth=2

use std::collections::HashMap;
use crate::keys::{Layout, Mapping, KeyCode};
use KeyCode::*;
use crate::layout_generation::{USKeyboardLayout, make_us_mappings};
use lazy_static::lazy_static;
use std::default::Default;

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
      Mapping { from: vec![CAPSLOCK], to: vec![], ..Default::default() },
      Mapping { from: vec![CAPSLOCK, J], to: vec![LEFT], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, I], to: vec![UP], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, K], to: vec![DOWN], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, L], to: vec![RIGHT], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, H], to: vec![HOME], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, SEMICOLON], to: vec![END], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, U], to: vec![PAGEUP], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, M], to: vec![PAGEDOWN], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, N], to: vec![LEFTCTRL, LEFT], ..Default::default() }, 
      Mapping { from: vec![CAPSLOCK, COMMA], to: vec![LEFTCTRL, RIGHT], ..Default::default() }
    ]
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
    Mapping { from: vec![CAPSLOCK], to: vec![], ..Default::default() },
  ];
  
  let mut mappings = Vec::new();
  mappings.append(&mut other_mappings);
  mappings.append(&mut char_mappings);
  
  Layout {
    mappings: mappings
  }
}

fn _caps_q_for_esc() -> Layout {
  Layout {
    mappings: vec![
      Mapping { from: vec![CAPSLOCK], to: vec![], ..Default::default() },
      Mapping { from: vec![CAPSLOCK, Q], to: vec![ESC], ..Default::default() }
    ]
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
    Mapping { from: vec![TAB], to: vec![], ..Default::default() },
    Mapping { from: vec![TAB, J], to: vec![LEFT], ..Default::default() }, 
    Mapping { from: vec![TAB, I], to: vec![UP], ..Default::default() }, 
    Mapping { from: vec![TAB, K], to: vec![DOWN], ..Default::default() }, 
    Mapping { from: vec![TAB, L], to: vec![RIGHT], ..Default::default() }, 
    Mapping { from: vec![TAB, H], to: vec![HOME], ..Default::default() }, 
    Mapping { from: vec![TAB, SEMICOLON], to: vec![END], ..Default::default() }, 
    Mapping { from: vec![TAB, U], to: vec![PAGEUP], ..Default::default() }, 
    Mapping { from: vec![TAB, M], to: vec![PAGEDOWN], ..Default::default() }, 
    Mapping { from: vec![TAB, N], to: vec![LEFTCTRL, LEFT], ..Default::default() }, 
    Mapping { from: vec![TAB, COMMA], to: vec![LEFTCTRL, RIGHT], ..Default::default() },
    
    Mapping { from: vec![CAPSLOCK], to: vec![], ..Default::default() },
    Mapping { from: vec![CAPSLOCK, Q], to: vec![ESC], ..Default::default() },
    Mapping { from: vec![BACKSLASH], to: vec![TAB], ..Default::default() },
  ];
  
  let mut all_mappings = Vec::new();
  all_mappings.append(&mut other_mappings);
  all_mappings.append(&mut char_mappings);
  
  Layout {
    mappings: all_mappings
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
    Mapping { from: vec![U], to: vec![G, F14], ..Default::default() },
    Mapping { from: vec![I], to: vec![C, F15], ..Default::default() },
    Mapping { from: vec![H], to: vec![D, F16], ..Default::default() },
    Mapping { from: vec![J], to: vec![H, F17], ..Default::default() },
    Mapping { from: vec![K], to: vec![T, F18], ..Default::default() },
    Mapping { from: vec![L], to: vec![N, T], ..Default::default() },
    Mapping { from: vec![SEMICOLON], to: vec![S, F20], ..Default::default() },
    Mapping { from: vec![N], to: vec![B, F21], ..Default::default() },
    Mapping { from: vec![M], to: vec![M, F22], ..Default::default() },
    Mapping { from: vec![COMMA], to: vec![W, F23], ..Default::default() },
    
    Mapping { from: vec![TAB], to: vec![], ..Default::default() },
    Mapping { from: vec![TAB, J], to: vec![LEFT], ..Default::default() }, 
    Mapping { from: vec![TAB, I], to: vec![UP], ..Default::default() }, 
    Mapping { from: vec![TAB, K], to: vec![DOWN], ..Default::default() }, 
    Mapping { from: vec![TAB, L], to: vec![RIGHT], ..Default::default() }, 
    Mapping { from: vec![TAB, H], to: vec![HOME], ..Default::default() }, 
    Mapping { from: vec![TAB, SEMICOLON], to: vec![END], ..Default::default() }, 
    Mapping { from: vec![TAB, U], to: vec![PAGEUP], ..Default::default() }, 
    Mapping { from: vec![TAB, M], to: vec![PAGEDOWN], ..Default::default() }, 
    Mapping { from: vec![TAB, N], to: vec![LEFTCTRL, LEFT], ..Default::default() }, 
    Mapping { from: vec![TAB, COMMA], to: vec![LEFTCTRL, RIGHT], ..Default::default() },
    
    Mapping { from: vec![CAPSLOCK], to: vec![], ..Default::default() },
    Mapping { from: vec![RIGHTALT], to: vec![], ..Default::default() },
    Mapping { from: vec![LEFTMETA], to: vec![], ..Default::default() },
    
    Mapping { from: vec![LEFTMETA, Q], to: vec![ESC], ..Default::default() },
    Mapping { from: vec![RIGHTALT, Q], to: vec![ESC], ..Default::default() },
    Mapping { from: vec![CAPSLOCK, Q], to: vec![ESC], ..Default::default() },
    
    Mapping { from: vec![BACKSLASH], to: vec![TAB], ..Default::default() },
    Mapping { from: vec![GRAVE], to: vec![LEFTMETA], ..Default::default() },
  ];
  
  let mut all_mappings = Vec::new();
  all_mappings.append(&mut other_mappings);
  all_mappings.append(&mut char_mappings);
  
  Layout {
    mappings: all_mappings,
  }
}

