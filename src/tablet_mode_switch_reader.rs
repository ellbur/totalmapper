
// vim: shiftwidth=2

use std::os::unix::io::RawFd;
use std::path::Path;
use nix::fcntl::{open, OFlag};
use nix::Error;
use nix::sys::stat::Mode;
use nix::unistd::read;
use libc::{input_event};
use std::mem::size_of;

pub struct TabletModeSwitchReader {
  pub fd: RawFd
}

#[derive(Debug)]
pub enum TableModeEvent {
  On, Off
}

impl TabletModeSwitchReader {
  pub fn open(path: &Path, nonblock: bool) -> Result<TabletModeSwitchReader, Error> {
    let fd = open(path, if nonblock {OFlag::O_RDONLY | OFlag::O_NONBLOCK} else {OFlag::O_RDONLY}, Mode::empty())?;
    
    Ok(TabletModeSwitchReader {
      fd: fd
    })
  }
  
  pub fn next(self: &mut TabletModeSwitchReader) -> Result<TableModeEvent, Error> {
    loop {
      let size = size_of::<input_event>();
      let mut buf: Vec<u8> = vec![0; size];
      read(self.fd, &mut buf)?;
      
      let type_ = u16::from_ne_bytes([buf[16], buf[17]]);
      let code = u16::from_ne_bytes([buf[18], buf[19]]);
      let value = i32::from_ne_bytes([buf[20], buf[21], buf[22], buf[23]]);
      
      if type_==5 && code==1 && value==1 {
        return Ok(TableModeEvent::On);
      }
      else if type_==5 && code==1 && value==0 {
        return Ok(TableModeEvent::Off);
      }
    }
  }
}

