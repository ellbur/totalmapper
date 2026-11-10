
#[cfg(test)]
mod tests {
  use crate::fancy_layout_interpreting::convert;
  use crate::layout_parsing_formatting::parse_layout_from_json;
  use crate::key_transforms::Mapper;
  use crate::keys::Event::{Pressed, Released};
  use crate::keys::KeyCode::*;

  #[test]
  fn test_shatur_issue_1() {
    let layout_text = r#"
      {
        "mappings": [
          { "from": [ "LEFTMETA", "1" ], "to": ["LEFTSHIFT", "BACKSLASH"] },
          { "from": [ "LEFTMETA" ], "to": [] }
        ]
      }
    "#;
    let fancy_layout =parse_layout_from_json(&serde_json::from_str(&layout_text).unwrap()).unwrap();
    let layout = convert(&fancy_layout).unwrap();

    println!("{:?}", layout);

    {
      let mut mapper = Mapper::for_layout(&layout);
      let input = vec![Pressed(LEFTMETA), Pressed(K1)];
      let mut output = vec![];
      for ev in &input {
        output.extend(mapper.step((*ev).clone()).events);
      }
      println!("input: {:?}", input);
      println!("output: {:?}", output);

      assert_eq!(output,
        vec![Pressed(LEFTSHIFT), Pressed(BACKSLASH)]
      );
    }
  }

  fn run(layout_text: &str, input: &[crate::keys::Event]) -> Vec<crate::keys::Event> {
    let fancy_layout = parse_layout_from_json(&serde_json::from_str(layout_text).unwrap()).unwrap();
    let layout = convert(&fancy_layout).unwrap();
    let mut mapper = Mapper::for_layout(&layout);
    let mut output = vec![];
    for ev in input {
      output.extend(mapper.step(ev.clone()).events);
    }
    output
  }

  const ABSORBING_EXCEPT_NEXT_LAYOUT: &str = r#"
    {
      "mappings": [
        { "from": "LEFTSHIFT", "to": "@shift" },
        { "from": "RIGHTSHIFT", "to": "@shift" },
        { "from": ["@shift", {"row": "A"}], "to": {"letters": "ASDFGHJKL"}, "absorbing": "@shift" },
        { "from": ["@shift", "DOT"], "to": ["@shift", "DOT"], "absorbing": "@shift" },
        { "from": ["@shift", "COMMA"], "to": ["@shift", "COMMA"],
          "absorbing": { "modifiers": "@shift", "except_next": "DOT" } }
      ]
    }
  "#;

  #[test]
  fn absorbing_except_next_end_to_end() {
    // Shift+, then .  =>  <>
    assert_eq!(
      run(ABSORBING_EXCEPT_NEXT_LAYOUT, &[Pressed(RIGHTSHIFT), Pressed(COMMA), Released(COMMA), Pressed(DOT)]),
      vec![Pressed(RIGHTSHIFT), Pressed(COMMA), Released(COMMA), Pressed(DOT)]
    );
    // Shift+, then A  =>  <a
    assert_eq!(
      run(ABSORBING_EXCEPT_NEXT_LAYOUT, &[Pressed(RIGHTSHIFT), Pressed(COMMA), Released(COMMA), Pressed(A)]),
      vec![Pressed(RIGHTSHIFT), Pressed(COMMA), Released(COMMA), Released(RIGHTSHIFT), Pressed(A)]
    );
  }
}
