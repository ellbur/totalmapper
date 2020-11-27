
// vim: shiftwidth=2

pub fn add_udev_rule(default_layout_name: Option<&str>, layout_file_path: Option<&str>) -> Result<(), String> {
  let rule = udev_rule(default_layout_name, layout_file_path)?;
  
  Ok(())
}

pub fn udev_rule(default_layout_name: Option<&str>, layout_file_path: Option<&str>) -> Result<String, String> {
  let lspec = layout_spec(default_layout_name, layout_file_path)?;
  Ok(format!("KERNEL==\"event*\", ACTION==\"add\", RUN+=\"/usr/bin/totalmapper remap {} --dev-file %N\"", lspec))
}

pub fn layout_spec(default_layout_name: Option<&str>, layout_file_path: Option<&str>) -> Result<String, String> {
  match (default_layout_name, layout_file_path) {
    (None, None) => {
      Err("Error: no layout specified. Use --default-layout or --layout-file.".to_string())
    },
    (Some(_), Some(_)) => {
      Err("Error: use either --default-layout or --layout-file, not both.".to_string())
    },
    (Some(name), None) => {
      Ok(format!("--default-layout {}", name))
    },
    (None, Some(path)) => {
      Ok(format!("--layout-file {}", path))
    }
  }
}

