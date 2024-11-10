
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
}

