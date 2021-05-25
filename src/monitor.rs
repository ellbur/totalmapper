 
// vim: shiftwidth=2

use crate::dev_input_rw::{DevInputReader, Exclusion};
use std::path::Path;
use nix::Error::Sys;
use nix::errno::Errno::ENODEV;

pub fn run_monitor(dev_file: &str) {
  match run_monitor_err(dev_file) {
    Err(msg) => println!("{}", msg),
    Ok(()) => ()
  }
}

fn run_monitor_err(dev_file: &str) -> Result<(), String> {
  let mut r = match DevInputReader::open(Path::new(dev_file), Exclusion::NoExclusion, false) {
    Err(e) => Err(format!("Failed to open {:?} for reading: {}", dev_file, e)),
    Ok(r) => Ok(r)
  }?;
  
  loop {
    match r.next() {
      Err(e) => {
        match e {
          Sys(ENODEV) => return Ok(()),
          _ => return Err(format!("Failed to read from device: {}", e))
        }
      }
      Ok(ev) => {
        println!("{:?}", ev);
      }
    }
  }
}

