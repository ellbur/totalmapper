
// vim: shiftwidth=2

use std::collections::HashMap;
use crate::keys::{Layout, Mapping, KeyCode, Repeat};
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
  
  let mut char_mappings = make_us_mappings(rows, &vec![CAPSLOCK, RIGHTALT], false, false);
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
  
  let mut char_mappings = make_us_mappings(rows, &vec![CAPSLOCK, RIGHTALT], false, false);
  
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
    row_q_shift: ":<>PYF  RL?^".to_string(),
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
  
  let mut char_mappings = make_us_mappings(rows, &vec![CAPSLOCK, RIGHTALT, LEFTMETA], true, true);
  
  // Safe keys to use: F19 F20 F21 F24
  let mut other_mappings = vec![
    Mapping { from: vec![U],     to: vec![G], repeat: Repeat::Special { keys: vec![F19], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    Mapping { from: vec![M],     to: vec![M], repeat: Repeat::Special { keys: vec![LEFTCTRL, F19], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    
    Mapping { from: vec![I],     to: vec![C], repeat: Repeat::Special { keys: vec![F20], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    Mapping { from: vec![K],     to: vec![T], repeat: Repeat::Special { keys: vec![LEFTCTRL, F20], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    
    Mapping { from: vec![J],     to: vec![H], repeat: Repeat::Special { keys: vec![F21], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    Mapping { from: vec![L],     to: vec![N], repeat: Repeat::Special { keys: vec![LEFTCTRL, F21], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    
    Mapping { from: vec![N],     to: vec![B], repeat: Repeat::Special { keys: vec![F24], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    Mapping { from: vec![COMMA], to: vec![W], repeat: Repeat::Special { keys: vec![LEFTCTRL, F24], delay_ms: 180, interval_ms: 30 }, ..Default::default() },
    
    Mapping { from: vec![H], to: vec![D], repeat: Repeat::Disabled, ..Default::default() },
    Mapping { from: vec![SEMICOLON], to: vec![S], repeat: Repeat::Disabled, ..Default::default() },
    
    Mapping { from: vec![LEFTSHIFT, U], to: vec![LEFTSHIFT, G], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, I], to: vec![LEFTSHIFT, C], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, H], to: vec![LEFTSHIFT, D], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, J], to: vec![LEFTSHIFT, H], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, K], to: vec![LEFTSHIFT, T], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, L], to: vec![LEFTSHIFT, N], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, SEMICOLON], to: vec![LEFTSHIFT, S], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, N], to: vec![LEFTSHIFT, B], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, M], to: vec![LEFTSHIFT, M], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, COMMA], to: vec![LEFTSHIFT, W], repeat: Repeat::Disabled, absorbing: vec![LEFTSHIFT], ..Default::default() },
    
    Mapping { from: vec![RIGHTSHIFT, U], to: vec![RIGHTSHIFT, G], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, I], to: vec![RIGHTSHIFT, C], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, H], to: vec![RIGHTSHIFT, D], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, J], to: vec![RIGHTSHIFT, H], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, K], to: vec![RIGHTSHIFT, T], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, L], to: vec![RIGHTSHIFT, N], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, SEMICOLON], to: vec![RIGHTSHIFT, S], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, N], to: vec![RIGHTSHIFT, B], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, M], to: vec![RIGHTSHIFT, M], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, COMMA], to: vec![RIGHTSHIFT, W], repeat: Repeat::Disabled, absorbing: vec![RIGHTSHIFT], ..Default::default() },
    
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
    
    Mapping { from: vec![LEFTSHIFT, TAB, J], to: vec![LEFTSHIFT, LEFT], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, I], to: vec![LEFTSHIFT, UP], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, K], to: vec![LEFTSHIFT, DOWN], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, L], to: vec![LEFTSHIFT, RIGHT], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, H], to: vec![LEFTSHIFT, HOME], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, SEMICOLON], to: vec![LEFTSHIFT, END], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, U], to: vec![LEFTSHIFT, PAGEUP], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, M], to: vec![LEFTSHIFT, PAGEDOWN], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, N], to: vec![LEFTSHIFT, LEFTCTRL, LEFT], ..Default::default() }, 
    Mapping { from: vec![LEFTSHIFT, TAB, COMMA], to: vec![LEFTSHIFT, LEFTCTRL, RIGHT], ..Default::default() },
    
    Mapping { from: vec![RIGHTSHIFT, TAB, J], to: vec![RIGHTSHIFT, LEFT], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, I], to: vec![RIGHTSHIFT, UP], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, K], to: vec![RIGHTSHIFT, DOWN], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, L], to: vec![RIGHTSHIFT, RIGHT], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, H], to: vec![RIGHTSHIFT, HOME], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, SEMICOLON], to: vec![RIGHTSHIFT, END], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, U], to: vec![RIGHTSHIFT, PAGEUP], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, M], to: vec![RIGHTSHIFT, PAGEDOWN], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, N], to: vec![RIGHTSHIFT, LEFTCTRL, LEFT], ..Default::default() }, 
    Mapping { from: vec![RIGHTSHIFT, TAB, COMMA], to: vec![RIGHTSHIFT, LEFTCTRL, RIGHT], ..Default::default() },
    
    Mapping { from: vec![CAPSLOCK], to: vec![], ..Default::default() },
    Mapping { from: vec![RIGHTALT], to: vec![], ..Default::default() },
    Mapping { from: vec![LEFTMETA], to: vec![], ..Default::default() },
    
    Mapping { from: vec![LEFTMETA, Q], to: vec![ESC], absorbing: vec![LEFTMETA], ..Default::default() },
    Mapping { from: vec![RIGHTALT, Q], to: vec![ESC], absorbing: vec![RIGHTALT], ..Default::default() },
    Mapping { from: vec![CAPSLOCK, Q], to: vec![ESC], absorbing: vec![CAPSLOCK], ..Default::default() },
    
    Mapping { from: vec![BACKSLASH], to: vec![TAB], ..Default::default() },
    Mapping { from: vec![LEFTSHIFT, BACKSLASH], to: vec![LEFTSHIFT, TAB], ..Default::default() },
    Mapping { from: vec![RIGHTSHIFT, BACKSLASH], to: vec![RIGHTSHIFT, TAB], ..Default::default() },
    Mapping { from: vec![GRAVE], to: vec![LEFTMETA], ..Default::default() },
    
    Mapping { from: vec![SPACE], to: vec![SPACE], repeat: Repeat::Disabled, ..Default::default() },
    
    Mapping { from: vec![LEFTALT, GRAVE, J], to: vec![LEFTALT, LEFTMETA, H], repeat: Repeat::Normal, ..Default::default() },
    Mapping { from: vec![LEFTALT, GRAVE, L], to: vec![LEFTALT, LEFTMETA, N], repeat: Repeat::Normal, ..Default::default() },
    
    Mapping { from: vec![LEFTALT, J], to: vec![LEFTALT, H], repeat: Repeat::Normal, ..Default::default() },
    Mapping { from: vec![LEFTALT, L], to: vec![LEFTALT, N], repeat: Repeat::Normal, ..Default::default() },
  ];
  
  let mut all_mappings = Vec::new();
  all_mappings.append(&mut other_mappings);
  all_mappings.append(&mut char_mappings);
  
  Layout {
    mappings: all_mappings,
  }
}

