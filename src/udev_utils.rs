
// vim: shiftwidth=2

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

pub fn add_udev_rule(default_layout_name: Option<&str>, layout_file_path: Option<&str>) -> Result<(), String> {
  let rule = udev_rule(default_layout_name, layout_file_path)?;
  
  let path = "/etc/udev/rules.d/80-totalmapper.rules";
  let mut out_file = match OpenOptions::new()
    .truncate(true).read(false).create(true).write(true)
    .open(path)
  {
    Err(err) => {
      match err.kind() {
        std::io::ErrorKind::PermissionDenied => {
          return Err(format!("Permission denied writing to {}. You likely must run this sub-command as root.", path));
        },
        _ => return Err(format!("{}", err))
      }
    },
    Ok(out_file) => out_file
  };
  
  match out_file.write(rule.as_bytes()) {
    Err(err) => return Err(format!("{}", err)),
    Ok(_) => ()
  };
  
  match out_file.write("\n".as_bytes()) {
    Err(err) => return Err(format!("{}", err)),
    Ok(_) => ()
  };
  
  match Command::new("/usr/bin/udevadm").args(&["control", "--reload"]).status() {
    Err(e) => Err(format!("Failed to run udevadm: {}", e)),
    Ok(_) => Ok(())
  }?;
  
  match Command::new("/usr/bin/udevadm").args(&["trigger"]).output() {
    Err(e) => Err(format!("Failed to run udevadm: {}", e)),
    Ok(_) => Ok(())
  }?;
  
  Ok(())
}

pub fn udev_rule(default_layout_name: Option<&str>, layout_file_path: Option<&str>) -> Result<String, String> {
  let lspec = layout_spec(default_layout_name, layout_file_path)?;
  Ok(format!("KERNEL==\"event*\", ACTION==\"add\", RUN+=\"/usr/bin/totalmapper remap --fork --log {} --only-if-keyboard --dev-file %N\"", lspec))
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
      if path.contains('"') || path.contains('\'') || path.contains('\\') {
        Err(format!("Cannot use layout file {} in a udev rule due to limitations on udev escaping.", path))
      }
      else {
        Ok(format!("--layout-file '{}'", path.escape_default()))
      }
    }
  }
}

