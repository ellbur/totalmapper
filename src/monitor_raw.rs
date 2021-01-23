
// vim: shiftwidth=2

use std::path::Path;
use nix::Error::Sys;
use nix::errno::Errno::ENODEV;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, write};
use nix::Error;
use libc::{input_event};
use std::mem::size_of;
use uinput_sys::{ui_set_evbit, EV_SYN, EV_KEY, EV_MSC, ui_dev_create, ui_set_keybit, KEY_MAX};
use crate::struct_ser::StructSerializer;
use std::os::unix::io::RawFd;
use nix::ioctl_write_int;
use crate::keys::{Event};
use num_traits::FromPrimitive;
use crate::struct_de::StructDeserializer;

pub fn run_monitor_raw(dev_file: &str) {
  match run_monitor_raw_err(dev_file) {
    Err(msg) => println!("{}", msg),
    Ok(()) => ()
  }
}

fn run_monitor_raw_err(dev_file: &str) -> Result<(), String> {
  let fd = match open(dev_file, OFlag::O_RDONLY, Mode::empty()) {
    Ok(fd) => Ok(fd),
    Err(e) => Err(format!("Error opening device file: {}", e))
  }?;
  
  loop {
    let size = size_of::<input_event>();
    let mut buf: Vec<u8> = vec![0; size];
    
    match read(fd, &mut buf) {
      Err(e) => {
        match e {
          Sys(ENODEV) => return Ok(()),
          _ => return Err(format!("Failed to read from device: {}", e))
        }
      }
      Ok(_) => {
        let mut struct_de = StructDeserializer::new(&buf);
        let sec = struct_de.read_i64().unwrap();
        let usec = struct_de.read_i64().unwrap();
        let type_ = struct_de.read_u16().unwrap();
        let code = struct_de.read_u16().unwrap();
        let value = struct_de.read_i32().unwrap();
        
        println!("[{}, {}], {}, {}, {}", sec, usec, type_, code, value);
      }
    }
  }
}

