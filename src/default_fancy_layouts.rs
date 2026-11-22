
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
    { "from": "LEFTCTRL", "to": "@ctrl" },
    { "from": "RIGHTCTRL", "to": "@ctrl" },
    { "from": "CAPSLOCK", "to": "@symbol" },
    { "from": "RIGHTALT", "to": "@symbol" },
    { "from": "TAB", "to": "@movement" },
    { "from": "BACKSLASH", "to": "TAB" },
    { "from": {"row": "1"}, "to": {"letters": "17531902468`"}, "repeat": "Disabled" },
    { "from": {"row": "Q"}, "to": {"letters": ";,.pyfgcrl~@"}, "repeat": "Disabled" },
    { "from": {"row": "A"}, "to": {"letters": "aoeuidhtns-"}, "repeat": "Disabled" },
    { "from": {"row": "Z"}, "to": {"letters": "'qjkxbmwvz"}, "repeat": "Disabled" },
    { "from": ["@shift", {"row": "Q"}], "to": {"letters": ":  PYFGCRL?^"}, "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", "W"], "to": ["@shift", "COMMA"], "absorbing": {"modifiers": "@shift", "except_next": "E"}, "repeat": "Disabled" },
    { "from": ["@shift", "E"], "to": ["@shift", "DOT"], "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", {"row": "A"}], "to": {"letters": "AOEUIDHTNS@"}, "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", {"row": "Z"}], "to": {"letters": "\"QJKXBMWVZ"}, "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@symbol", {"row": "Q"}], "to": {"letters": " {}% \\*][|~"}, "repeat": "Disabled" },
    { "from": ["@symbol", {"row": "A"}], "to": {"letters": "   = &)(/_$"}, "repeat": "Disabled" },
    { "from": ["@symbol", {"row": "Z"}], "to": {"letters": "     !+#"}, "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", {"row": "Q"}], "to": ["RIGHTALT", {"letters": ";,.pyfgcrl~@"}], "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", {"row": "A"}], "to": ["RIGHTALT", {"letters": "aoeuidhtns-"}], "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", {"row": "Z"}], "to": ["RIGHTALT", {"letters": "'qjkxbmwvz"}], "absorbing": "@shift", "repeat": "Disabled" },
    { "from": ["@shift", "@symbol", "SPACE"], "to": ["RIGHTALT", "N"], "repeat": "Disabled" },
    { "from": ["@symbol", "Q"], "to": "ESC", "absorbing": "@symbol", "repeat": "Disabled" },
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
    { "from": ["@movement", "O"], "to": "TAB" },
    { "from": ["@shift", "@movement", "O"], "to": ["@shift", "TAB"] },
    { "from": ["@movement", "SPACE"], "to": "ENTER", "repeat": "Disabled", "requires_fresh": "@movement" },
    { "from": ["@shift", "@movement", "SPACE"], "to": ["@shift", "ENTER"], "repeat": "Disabled", "requires_fresh": "@movement" },
    { "from": ["@ctrl", "@movement", "SPACE"], "to": ["@ctrl", "ENTER"], "repeat": "Disabled", "requires_fresh": "@movement" },
    { "from": ["@symbol", "@movement", "J"], "to": "BACKSPACE" },
    { "from": ["@symbol", "@movement", "L"], "to": "DELETE" },
    { "from": ["@symbol", "@movement", "N"], "to": ["LEFTCTRL", "BACKSPACE"] },
    { "from": ["@symbol", "@movement", "COMMA"], "to": ["LEFTCTRL", "DELETE"] },
    { "from": "GRAVE", "to": "LEFTMETA" },
    { "from": ["GRAVE", {"row": "1"}], "to": ["LEFTMETA", {"letters": "17531902468`"}], "repeat": "Disabled" },
    { "from": ["GRAVE", {"row": "Q"}], "to": ["LEFTMETA", {"letters": ";,.pyfgcrl~@"}], "repeat": "Disabled" },
    { "from": ["GRAVE", {"row": "A"}], "to": ["LEFTMETA", {"letters": "aoeuidhtns-"}], "repeat": "Disabled" },
    { "from": ["GRAVE", {"row": "Z"}], "to": ["LEFTMETA", {"letters": "'qjkxbmwvz"}], "repeat": "Disabled" },
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
  use crate::{layout_parsing_formatting::{parse_layout_from_json, format_layout_as_json}, default_fancy_layouts::SUPER_DVORAK};
  use std::str::FromStr;
  
  fn check_consistency(layout_json: &str) {
    let json = serde_json::Value::from_str(layout_json).unwrap();
    let restringed1 = json.to_string();
    let layout = parse_layout_from_json(&json).unwrap();
    let formatted = format_layout_as_json(&layout);
    let restringed2 = formatted.to_string();

    if restringed1 != restringed2 {
      use colored::Colorize;
      use similar::{ChangeTag, TextDiff};
      let diff = TextDiff::from_chars(&restringed1, &restringed2);

      for change in diff.iter_all_changes() {
        let s = change.value();
        print!("{}",
          match change.tag() {
            ChangeTag::Equal => s.normal(),
            ChangeTag::Insert => s.green(),
            ChangeTag::Delete => s.red(),
          }
        );
      }
      println!("");
      println!("");
    }
    assert_eq!(restringed1, restringed2);
  }
  
  #[test]
  fn test_consistent_formatting_1() {
    check_consistency(super::CAPS_LOCK_FOR_MOVEMENT);
  }
  
  #[test]
  fn test_consistent_formatting_2() {
    check_consistency(super::EASY_SYMBOLS);
  }
  
  #[test]
  fn test_consistent_formatting_3() {
    check_consistency(super::CAPS_Q_FOR_ESC);
  }
  
  #[test]
  fn test_consistent_formatting_4() {
    check_consistency(super::EASY_SYMBOLS_TAB_FOR_MOVEMENT);
  }
  
  #[test]
  fn test_consistent_formatting_5() {
    check_consistency(super::SUPER_DVORAK);
  }
  
  #[test]
  fn super_dvorak_test_1() {
    let json = serde_json::Value::from_str(SUPER_DVORAK).unwrap();
    use crate::events::Event;
    use Event::Pressed;
    use crate::key_codes::KeyCode::*;
    let fancy_layout = parse_layout_from_json(&json).unwrap();
    let simple_layout = crate::fancy_layout_interpreting::convert(&fancy_layout).unwrap();
    let mut mapper = crate::key_transforms::Mapper::for_layout(&simple_layout);
    assert_eq!(mapper.step(Pressed(LEFTSHIFT)).events, vec![Pressed(LEFTSHIFT)]);
    assert_eq!(mapper.step(Pressed(TAB)).events, vec![]);
    assert_eq!(mapper.step(Pressed(I)).events, vec![Pressed(UP)]);
  }

  fn run_super_dvorak(input: &[crate::events::Event]) -> Vec<crate::events::Event> {
    let json = serde_json::Value::from_str(SUPER_DVORAK).unwrap();
    let fancy_layout = parse_layout_from_json(&json).unwrap();
    let simple_layout = crate::fancy_layout_interpreting::convert(&fancy_layout).unwrap();
    let mut mapper = crate::key_transforms::Mapper::for_layout(&simple_layout);
    let mut output = vec![];
    for ev in input {
      output.extend(mapper.step(ev.clone()).events);
    }
    output
  }

  #[test]
  fn super_dvorak_angle_brackets_without_lifting_shift() {
    use crate::events::Event::{Pressed, Released};
    use crate::key_codes::KeyCode::*;
    // Shift held, W then E  =>  <>
    assert_eq!(
      run_super_dvorak(&[Pressed(LEFTSHIFT), Pressed(W), Released(W), Pressed(E), Released(E)]),
      vec![Pressed(LEFTSHIFT), Pressed(COMMA), Released(COMMA), Pressed(DOT), Released(DOT)]
    );
    // Shift held, W then P  =>  <p (shift still absorbed for other keys)
    assert_eq!(
      run_super_dvorak(&[Pressed(LEFTSHIFT), Pressed(W), Released(W), Pressed(R)]),
      vec![Pressed(LEFTSHIFT), Pressed(COMMA), Released(COMMA), Released(LEFTSHIFT), Pressed(P), Released(P)]
    );
  }

  fn assert_super_dvorak(input: &[crate::events::Event], expected: &[crate::events::Event]) {
    assert_eq!(run_super_dvorak(input), expected.to_vec(), "input: {:?}", input);
  }

  #[test]
  fn super_dvorak_tab_alternative() {
    use crate::events::Event::{Pressed as P, Released as R};
    use crate::key_codes::KeyCode::*;
    assert_super_dvorak(&[P(TAB), P(O), R(O), R(TAB)], &[P(TAB), R(TAB)]);
    // Shift+Tab, with Shift pressed before or after Tab
    assert_super_dvorak(&[P(LEFTSHIFT), P(TAB), P(O), R(O), R(TAB), R(LEFTSHIFT)],
      &[P(LEFTSHIFT), P(TAB), R(TAB), R(LEFTSHIFT)]);
    assert_super_dvorak(&[P(TAB), P(LEFTSHIFT), P(O), R(O), R(LEFTSHIFT), R(TAB)],
      &[P(LEFTSHIFT), P(TAB), R(TAB), R(LEFTSHIFT)]);
  }

  #[test]
  fn super_dvorak_enter_alternative() {
    use crate::events::Event::{Pressed as P, Released as R};
    use crate::key_codes::KeyCode::*;
    // Enter does not repeat: it is released immediately even while Space is held
    assert_super_dvorak(&[P(TAB), P(SPACE)], &[P(ENTER), R(ENTER)]);
    // Pressing Space again keeps Tab fresh
    assert_super_dvorak(&[P(TAB), P(SPACE), R(SPACE), P(SPACE), R(SPACE), R(TAB)],
      &[P(ENTER), R(ENTER), P(ENTER), R(ENTER)]);
    // After navigating, Tab is no longer fresh, so Space is just a space
    assert_super_dvorak(&[P(TAB), P(J), R(J), P(SPACE), R(SPACE), R(TAB)],
      &[P(LEFT), R(LEFT), P(SPACE), R(SPACE)]);
    assert_super_dvorak(&[P(TAB), P(SPACE), R(SPACE), P(J), R(J), P(SPACE), R(SPACE), R(TAB)],
      &[P(ENTER), R(ENTER), P(LEFT), R(LEFT), P(SPACE), R(SPACE)]);
    // Shift+Enter and Ctrl+Enter, with the modifier pressed before or after Tab
    assert_super_dvorak(&[P(LEFTSHIFT), P(TAB), P(SPACE), R(SPACE), R(TAB), R(LEFTSHIFT)],
      &[P(LEFTSHIFT), P(ENTER), R(ENTER), R(LEFTSHIFT)]);
    assert_super_dvorak(&[P(TAB), P(RIGHTSHIFT), P(SPACE), R(SPACE), R(RIGHTSHIFT), R(TAB)],
      &[P(RIGHTSHIFT), P(ENTER), R(ENTER), R(RIGHTSHIFT)]);
    assert_super_dvorak(&[P(LEFTCTRL), P(TAB), P(SPACE), R(SPACE), R(TAB), R(LEFTCTRL)],
      &[P(LEFTCTRL), P(ENTER), R(ENTER), R(LEFTCTRL)]);
    assert_super_dvorak(&[P(TAB), P(RIGHTCTRL), P(SPACE), R(SPACE), R(RIGHTCTRL), R(TAB)],
      &[P(RIGHTCTRL), P(ENTER), R(ENTER), R(RIGHTCTRL)]);
    // Plain space is unaffected
    assert_super_dvorak(&[P(SPACE), R(SPACE)], &[P(SPACE), R(SPACE)]);
  }

  #[test]
  fn super_dvorak_delete_alternatives() {
    use crate::events::Event::{Pressed as P, Released as R};
    use crate::key_codes::KeyCode::*;
    // Backspace repeats: it stays held until J is released
    assert_super_dvorak(&[P(CAPSLOCK), P(TAB), P(J)], &[P(BACKSPACE)]);
    assert_super_dvorak(&[P(CAPSLOCK), P(TAB), P(J), R(J), P(J), R(J), R(TAB), R(CAPSLOCK)],
      &[P(BACKSPACE), R(BACKSPACE), P(BACKSPACE), R(BACKSPACE)]);
    // Either order of CapsLock and Tab, or Right Alt instead of CapsLock
    assert_super_dvorak(&[P(TAB), P(CAPSLOCK), P(J), R(J), R(CAPSLOCK), R(TAB)],
      &[P(BACKSPACE), R(BACKSPACE)]);
    assert_super_dvorak(&[P(RIGHTALT), P(TAB), P(J), R(J), R(TAB), R(RIGHTALT)],
      &[P(RIGHTALT), R(RIGHTALT), P(BACKSPACE), R(BACKSPACE)]);
    assert_super_dvorak(&[P(CAPSLOCK), P(TAB), P(L), R(L), R(TAB), R(CAPSLOCK)],
      &[P(DELETE), R(DELETE)]);
    assert_super_dvorak(&[P(CAPSLOCK), P(TAB), P(N), R(N), R(TAB), R(CAPSLOCK)],
      &[P(LEFTCTRL), P(BACKSPACE), R(BACKSPACE), R(LEFTCTRL)]);
    assert_super_dvorak(&[P(CAPSLOCK), P(TAB), P(COMMA), R(COMMA), R(TAB), R(CAPSLOCK)],
      &[P(LEFTCTRL), P(DELETE), R(DELETE), R(LEFTCTRL)]);
    // The movement and symbol layers on their own are unchanged
    assert_super_dvorak(&[P(TAB), P(J), R(J), R(TAB)], &[P(LEFT), R(LEFT)]);
    assert_super_dvorak(&[P(CAPSLOCK), P(J), R(J), R(CAPSLOCK)],
      &[P(LEFTSHIFT), P(K0), R(K0), R(LEFTSHIFT)]);
  }
}

