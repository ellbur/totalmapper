
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref DEFAULT_LAYOUTS: HashMap<String, &'static str> = {
    vec![
     ("caps-for-movement".to_string(), &*CAPS_LOCK_FOR_MOVEMENT),
     ("easy-symbols".to_string(), &*EASY_SYMBOLS),
     ("caps-q-for-esc".to_string(), &*CAPS_Q_FOR_ESC),
     ("easy-symbols-tab-for-movement".to_string(), &*EASY_SYMBOLS_TAB_FOR_MOVEMENT),
     ("super-dvorak".to_string(), &*SUPER_DVORAK),
    ].into_iter().collect()
  };
}

pub static CAPS_LOCK_FOR_MOVEMENT: &'static str = r#"{
  "mappings": [
    { "from": "CAPSLOCK", "to": [] },
    { "from": ["CAPSLOCK", "J"], "to": "LEFT" },
    { "from": ["CAPSLOCK", "I"], "to": "UP" },
    { "from": ["CAPSLOCK", "K"], "to": "DOWN" },
    { "from": ["CAPSLOCK", "L"], "to": "RIGHT" },
    { "from": ["CAPSLOCK", "H"], "to": "HOME" },
    { "from": ["CAPSLOCK", "SEMICOLON"], "to": "END" },
    { "from": ["CAPSLOCK", "U"], "to": "PAGEUP" },
    { "from": ["CAPSLOCK", "M"], "to": "PAGEDOWN" },
    { "from": ["CAPSLOCK", "N"], "to": ["LEFTCTRL", "LEFT"] },
    { "from": ["CAPSLOCK", "COMMA"], "to": ["LEFTCTRL", "RIGHT"] }
  ]
}"#;

pub static EASY_SYMBOLS: &'static str = r#"{
  "mappings": [
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" },
    { "from": ["@symbol", {"row": "Q"}], "to": {"letters": " {}% \\*][|~"} },
    { "from": ["@symbol", {"row": "A"}], "to": {"letters": "   = &)(/_$"} },
    { "from": ["@symbol", {"row": "Z"}], "to": {"letters": "     !+#"} }
  ]
}"#;

pub static CAPS_Q_FOR_ESC: &'static str = r#"{
  "mappings": [
    { "from": "CAPSLOCK", "to": [] },
    { "from": ["CAPSLOCK", "Q"], "to": "ESC" }
  ]
}"#;

pub static EASY_SYMBOLS_TAB_FOR_MOVEMENT: &'static str = r#"{
  "mappings": [
    { "from": "LEFTSHIFT", "to": "@shift" },
    { "from": "RIGHTSHIFT", "to": "@shift" },
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" },
    { "from": "TAB", "to": "@movement" },
    { "from": "BACKSLASH", "to": "TAB" },
    { "from": ["@symbol", {"row": "Q"}], "to": {"letters": " {}% \\*][|~"} },
    { "from": ["@symbol", {"row": "A"}], "to": {"letters": "   = &)(/_$"} },
    { "from": ["@symbol", {"row": "Z"}], "to": {"letters": "     !+#"} },
    { "from": ["@symbol", "Q"], "to": "ESC" },
    { "from": ["@movement", "J"], "to": "LEFT" }, 
    { "from": ["@movement", "I"], "to": "UP" }, 
    { "from": ["@movement", "K"], "to": "DOWN" }, 
    { "from": ["@movement", "L"], "to": "RIGHT" }, 
    { "from": ["@movement", "H"], "to": "HOME" }, 
    { "from": ["@movement", "SEMICOLON"], "to": "END" }, 
    { "from": ["@movement", "U"], "to": "PAGEUP" }, 
    { "from": ["@movement", "M"], "to": "PAGEDOWN" }, 
    { "from": ["@movement", "N"], "to": ["LEFTCTRL", "LEFT"] }, 
    { "from": ["@movement", "COMMA"], "to": ["LEFTCTRL", "RIGHT"] }
  ]
}"#;

pub static SUPER_DVORAK: &'static str = r#"{
  "mappings": [
    { "from": "LEFTSHIFT", "to": "@shift" },
    { "from": "RIGHTSHIFT", "to": "@shift" },
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" },
    { "from": "TAB", "to": "@movement" },
    { "from": "BACKSLASH", "to": "TAB" },
    { "from": {"row": "`"}, "to": {"letters": " 17531902468`"}, "repeat": "Disabled" },
    { "from": {"row": "Q"}, "to": {"letters": ";,.pyfgcrl~@"}, "repeat": "Disabled" },
    { "from": {"row": "A"}, "to": {"letters": "aoeuidhtns-"}, "repeat": "Disabled" },
    { "from": {"row": "Z"}, "to": {"letters": "'qjkxbmwvz"}, "repeat": "Disabled" },
    { "from": ["@shift", {"row": "Q"}], "to": {"letters": ":<>PYFGCRL?^"}, "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", {"row": "A"}], "to": {"letters": "AOEUIDHTNS@"}, "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", {"row": "Z"}], "to": {"letters": "\"QJKXBWVZ"}, "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@symbol", {"row": "Q"}], "to": {"letters": " {}% \\*][|~"}, "repeat": "Disabled" },
    { "from": ["@symbol", {"row": "A"}], "to": {"letters": "   = &)(/_$"}, "repeat": "Disabled" },
    { "from": ["@symbol", {"row": "Z"}], "to": {"letters": "     !+#"}, "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", {"row": "Q"}], "to": ["RIGHTALT", {"letters": ";,.pyfgcrl~@"}], "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", {"row": "A"}], "to": ["RIGHTALT", {"letters": "aoeuidhtns-s"}], "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", {"row": "Z"}], "to": ["RIGHTALT", {"letters": "'qjkxbmwvz"}], "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", "SPACE"], "to": ["RIGHTALT", "N"], "repeat": "Disabled" },
    { "from": ["@symbol", "Q"], "to": "ESC", "repeat": "Disabled" },
    { "from": ["@movement", "J"], "to": "LEFT" }, 
    { "from": ["@movement", "I"], "to": "UP" }, 
    { "from": ["@movement", "K"], "to": "DOWN" }, 
    { "from": ["@movement", "L"], "to": "RIGHT" }, 
    { "from": ["@movement", "H"], "to": "HOME" }, 
    { "from": ["@movement", "SEMICOLON"], "to": "END" }, 
    { "from": ["@movement", "U"], "to": "PAGEUP" }, 
    { "from": ["@movement", "M"], "to": "PAGEDOWN" }, 
    { "from": ["@movement", "N"], "to": ["LEFTCTRL", "LEFT"] }, 
    { "from": ["@movement", "COMMA"], "to": ["LEFTCTRL", "RIGHT"] },
    { "from": "J", "repeat": {"Special": {"keys": "F21", "delay_ms": 180, "interval_ms": 30}} },
    { "from": "I", "repeat": {"Special": {"keys": "F20", "delay_ms": 180, "interval_ms": 30}} },
    { "from": "K", "repeat": {"Special": {"keys": ["LEFTCTRL", "F20"], "delay_ms": 180, "interval_ms": 30}} },
    { "from": "L", "repeat": {"Special": {"keys": ["LEFTCTRL", "F21"], "delay_ms": 180, "interval_ms": 30}} },
    { "from": "U", "repeat": {"Special": {"keys": "F19", "delay_ms": 180, "interval_ms": 30}} },
    { "from": "M", "repeat": {"Special": {"keys": ["LEFTCTRL", "F19"], "delay_ms": 180, "interval_ms": 30}} },
    { "from": "N", "repeat": {"Special": {"keys": "F24", "delay_ms": 180, "interval_ms": 30}} },
    { "from": "COMMA", "repeat": {"Special": {"keys": ["LEFTCTRL", "F24"], "delay_ms": 180, "interval_ms": 30}} }
  ]
}"#;

#[cfg(test)]
mod tests {
  use crate::layout_parsing_formatting::{parse_layout_from_json, format_layout_as_json};
  use std::str::FromStr;
  
  fn check_consistency(layout_json: &str) {
    let json = serde_json::Value::from_str(layout_json).unwrap();
    let restringed1 = json.to_string();
    let layout = parse_layout_from_json(&json).unwrap();
    let formatted = format_layout_as_json(&layout);
    let restringed2 = formatted.to_string();

    if restringed1 != restringed2 {
      println!("{}", restringed1);
      println!("{}", restringed2);
    }
    assert_eq!(restringed1, restringed2);
  }
  
  #[test]
  fn test_consistent_formatting() {
    check_consistency(super::CAPS_LOCK_FOR_MOVEMENT);
    check_consistency(super::EASY_SYMBOLS);
    check_consistency(super::CAPS_Q_FOR_ESC);
    check_consistency(super::EASY_SYMBOLS_TAB_FOR_MOVEMENT);
    check_consistency(super::SUPER_DVORAK);
  }
}

