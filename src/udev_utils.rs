
// vim: shiftwidth=2

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use crate::keys::Layout;

fn convert_io_error<T>(whats_happening: &str, res: Result<T, std::io::Error>) -> Result<T, String> {
  match res {
    Ok(t) => Ok(t),
    Err(e) => Err(format!("Error {}: {}", whats_happening, e))
  }
}

fn convert_json_error<T>(whats_happening: &str, res: Result<T, serde_json::Error>) -> Result<T, String> {
  match res {
    Ok(t) => Ok(t),
    Err(e) => Err(format!("Error {}: {}", whats_happening, e))
  }
}

pub fn add_systemd_service(layout: &Layout) -> Result<(), String> {
  write_layout_to_global_config(layout)?;
  write_udev_rule()?;
  write_systemd_service()?;
  refresh_udev()?;
  refresh_systemd()?;
  Ok(())
}

fn write_layout_to_global_config(layout: &Layout) -> Result<(), String> {
  let file_out = convert_io_error(
    "saving layout to /etc/totalmapper.json",
    OpenOptions::new()
      .truncate(true).read(false).create(true).write(true)
      .open("/etc/totalmapper.json")
  )?;
  
  let buffered_out = std::io::BufWriter::new(file_out);
  
  convert_json_error(
    "saving layout to /etc/totalmapper.json",
    serde_json::to_writer_pretty(
      buffered_out,
      layout
    )
  )?;
  
  Ok(())
}

pub fn write_udev_rule() -> Result<(), String> {
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
  
  match out_file.write(
    "KERNEL==\"event*\", ACTION==\"add\", TAG+=\"systemd\", ENV{SYSTEMD_WANTS}=\"totalmapper@%N.service\"\n".as_bytes()
  ) {
    Err(err) => return Err(format!("{}", err)),
    Ok(_) => ()
  };
  
  Ok(())
}

pub fn write_systemd_service() -> Result<(), String> {
  let path = "/etc/systemd/system/totalmapper@.service";
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
  
  match out_file.write(
    "[Unit]\n\
    StopWhenUnneeded=true\n\
    Description=Totalmapper\n\
    \n\
    [Service]\n\
    Type=simple\n\
    User=nobody\n\
    Group=input\n\
    ExecStart=/usr/bin/totalmapper remap --layout-file /etc/totalmapper.json --only-if-keyboard --dev-file /%I\n".as_bytes()
  ) {
    Err(err) => return Err(format!("{}", err)),
    Ok(_) => ()
  };
  
  Ok(())
}

pub fn refresh_udev() -> Result<(), String> {
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

pub fn refresh_systemd() -> Result<(), String> {
  match Command::new("/usr/bin/systemctl").args(&["daemon-reload"]).status() {
    Err(e) => Err(format!("Failed to reload systemd: {}", e)),
    Ok(_) => Ok(())
  }?;
  
  Ok(())
}

