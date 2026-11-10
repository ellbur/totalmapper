
use std::fmt::Display;

// vim: shiftwidth=2
 
pub use crate::key_codes::KeyCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
  pub mappings: Vec<Mapping>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mapping {
  Single(SingleMapping),
  Alias(AliasMapping),
  Row(RowMapping),
  RepeatOnlySingle(RepeatOnlySingleMapping),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasMapping {
  pub from: AliasFromKeys,
  pub to: AliasToKeys
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleMapping {
  pub from: SingleFromKeys,
  pub to: SingleToKeys,
  pub repeat: SingleRepeat,
  pub absorbing: Absorbing
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatOnlySingleMapping {
  pub from: SingleFromKeys,
  pub repeat: SingleRepeat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowMapping {
  pub from: RowFromKeys,
  pub to: RowToKeys,
  pub repeat: RowRepeat,
  pub absorbing: Absorbing
}

/// Which modifiers a mapping "uses up" once it fires, so that they do not
/// apply to the following keystroke.
///
/// `except_next` lists physical keys that, if pressed immediately after this
/// mapping, still see the absorbed modifiers. For example, absorbing `@shift`
/// on Shift+COMMA (`<`) with `except_next: ["DOT"]` lets Shift+COMMA, DOT type
/// `<>` while Shift+COMMA, A types `<a`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Absorbing {
  pub modifiers: Vec<Modifier>,
  pub except_next: Vec<KeyCode>
}

impl Absorbing {
  pub fn modifiers(modifiers: Vec<Modifier>) -> Absorbing {
    Absorbing { modifiers, except_next: vec![] }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleFromKeys {
  pub modifiers: Vec<Modifier>,
  pub key: KeyCode
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasFromKeys {
  pub keys: Vec<KeyCode>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowFromKeys {
  pub modifiers: Vec<Modifier>,
  pub row: Row
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Row {
  USQuertyGrave,
  USQuerty1,
  USQuertyQ,
  USQuertyA,
  USQuertyZ,
}

impl Display for Row {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Row::USQuertyGrave => f.write_str("`"),
      Row::USQuerty1 => f.write_str("1"),
      Row::USQuertyQ => f.write_str("Q"),
      Row::USQuertyA => f.write_str("A"),
      Row::USQuertyZ => f.write_str("Z"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
  Key(KeyCode),
  Alias(String)
}

impl Display for Modifier {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Modifier::Key(k) => f.write_fmt(format_args!("{}", k)),
      Modifier::Alias(name) => f.write_str(name)
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleToKeys {
  pub initial: Vec<Modifier>,
  pub terminal: SingleTerminalToKey
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasToKeys {
  pub initial: Vec<KeyCode>,
  pub terminal: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowToKeys {
  pub initial: Vec<Modifier>,
  pub terminal: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleTerminalToKey {
  Physical(KeyCode),
  Null
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleRepeat {
  Normal,
  Disabled,
  Special {
    keys: SingleToKeys,
    delay_ms: i32,
    interval_ms: i32
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowRepeat {
  Normal,
  Disabled,
  Special {
    keys: RowToKeys,
    delay_ms: i32,
    interval_ms: i32
  }
}

