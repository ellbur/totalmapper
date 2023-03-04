
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
    { "from": {"row": "`"}, "to": {"letters": " 17531902468`"} },
    { "from": {"row": "Q"}, "to": {"letters": ";,.pyf  rl~@"} },
    { "from": {"row": "A"}, "to": {"letters": "aoeui     -"} },
    { "from": {"row": "Z"}, "to": {"letters": "'qjkx   vz"} },
    { "from": ["@shift", {"row": "Q"}], "to": {"letters": ":<>PYFGCRL?^"}, "absorbing": "@shift" },
    { "from": ["@shift", {"row": "A"}], "to": {"letters": "AOEUIDHTNS@"}, "absorbing": "@shift" },
    { "from": ["@shift", {"row": "Z"}], "to": {"letters": "\"QJKXBWVZ"}, "absorbing": "@shift" },
    { "from": ["@symbol", {"row": "Q"}], "to": {"letters": " {}% \\*][|~"} },
    { "from": ["@symbol", {"row": "A"}], "to": {"letters": "   = &)(/_$"} },
    { "from": ["@symbol", {"row": "Z"}], "to": {"letters": "     !+#"} },
    { "from": ["@shift", "@symbol", {"row": "Q"}], "to": ["RIGHTALT", {"letters": "   p"}], "absorbing": "@shift" },
    { "from": ["@shift", "@symbol", {"row": "A"}], "to": ["RIGHTALT", {"letters": "         s"}], "absorbing": "@shift" },
    { "from": ["@shift", "@symbol", "SPACE"], "to": ["RIGHTALT", "N"] },
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
    { "from": ["@movement", "COMMA"], "to": ["LEFTCTRL", "RIGHT"] },
    { "from": ["@shift", "@movement", "J"], "to": ["@shift", "LEFT"] }, 
    { "from": ["@shift", "@movement", "I"], "to": ["@shift", "UP"] }, 
    { "from": ["@shift", "@movement", "K"], "to": ["@shift", "DOWN"] }, 
    { "from": ["@shift", "@movement", "L"], "to": ["@shift", "RIGHT"] }, 
    { "from": ["@shift", "@movement", "H"], "to": ["@shift", "HOME"] }, 
    { "from": ["@shift", "@movement", "SEMICOLON"], "to": ["@shift", "END"] }, 
    { "from": ["@shift", "@movement", "U"], "to": ["@shift", "PAGEUP"] }, 
    { "from": ["@shift", "@movement", "M"], "to": ["@shift", "PAGEDOWN"] }, 
    { "from": ["@shift", "@movement", "N"], "to": ["@shift", "LEFTCTRL", "LEFT"] }, 
    { "from": ["@shift", "@movement", "COMMA"], "to": ["@shift", "LEFTCTRL", "RIGHT"] }
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

