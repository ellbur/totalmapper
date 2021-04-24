
// vim: shiftwidth=2
 
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, write};
use nix::Error;
use libc::{input_event};
use std::mem::size_of;
use uinput_sys::{ui_set_evbit, EV_SYN, EV_KEY, EV_MSC, ui_dev_create, ui_set_keybit};
use crate::struct_ser::StructSerializer;
use std::os::unix::io::RawFd;
use nix::ioctl_write_int;
use crate::keys::{Event};
use num_traits::FromPrimitive;
use std::path::{Path};

pub struct DevInputReader {
  pub fd: RawFd
}

impl DevInputReader {
  pub fn next(self: &mut DevInputReader) -> Result<Event, Error> {
    loop {
      let size = size_of::<input_event>();
      let mut buf: Vec<u8> = vec![0; size];
      read(self.fd, &mut buf)?;
      
      let type_ = u16::from_ne_bytes([buf[16], buf[17]]);
      let code = u16::from_ne_bytes([buf[18], buf[19]]);
      let value = i32::from_ne_bytes([buf[20], buf[21], buf[22], buf[23]]);
      
      if type_ == 1 && (value == 0 || value == 1) {
        match FromPrimitive::from_u16(code) {
          Some(k) => match value {
            1 => return Ok(Event::Pressed(k)),
            0 => return Ok(Event::Released(k)),
            _ => ()
          },
          None => ()
        }
      }
    }
  }
  
  pub fn open(path: &Path, exclusive: bool, nonblock: bool) -> Result<DevInputReader, Error> {
    let fd = open(path, if nonblock {OFlag::O_RDONLY | OFlag::O_NONBLOCK} else {OFlag::O_RDONLY}, Mode::empty())?;
    
    if exclusive {
      unsafe {
        eviocgrab(fd, 1)?;
      }
    }
    
    Ok(DevInputReader {
      fd: fd
    })
  }
}

const EVIOCGRAB_NUM: u8 = b'E';
const EVIOCGRAB_SEQ: u8 = 0x90;

ioctl_write_int!(eviocgrab, EVIOCGRAB_NUM, EVIOCGRAB_SEQ);

pub struct DevInputWriter {
  fd: RawFd
}

impl DevInputWriter {
  pub fn open() -> Result<DevInputWriter, Error> {
    let fdo = open("/dev/uinput", OFlag::O_WRONLY | OFlag::O_NONBLOCK, Mode::empty())?;

    unsafe {
      ui_set_evbit(fdo, EV_SYN);
      ui_set_evbit(fdo, EV_KEY);
      ui_set_evbit(fdo, EV_MSC);
    }
    
    for i in 1 .. 562 {
      unsafe { ui_set_keybit(fdo, i); }
    }
    
    {
      let mut user_dev_data = StructSerializer {
        sink: Vec::new()
      };
      
      user_dev_data.add_string_in_buf("totalmapper", 80);
      
      user_dev_data.add_u16(3);
      user_dev_data.add_u16(1);
      user_dev_data.add_u16(1);
      user_dev_data.add_u16(1);
      
      user_dev_data.add_u32(0);
      
      user_dev_data.add_i32_array(&[0; 64]);
      user_dev_data.add_i32_array(&[0; 64]);
      user_dev_data.add_i32_array(&[0; 64]);
      user_dev_data.add_i32_array(&[0; 64]);
      
      write(fdo, &user_dev_data.sink).unwrap();
    }
    
    unsafe { ui_dev_create(fdo); }
  
    Ok(DevInputWriter { fd: fdo })
  }
  
  pub fn send(self: &mut DevInputWriter, evs: &Vec<Event>) -> Result<(), Error> {
    let mut input_event_data = StructSerializer {
      sink: Vec::new()
    };
    
    let mut send_type_code_value = |type_, code, value| {
      input_event_data.add_i64(0);
      input_event_data.add_i64(0);
      input_event_data.add_u16(type_);
      input_event_data.add_u16(code);
      input_event_data.add_i32(value);
    };
      
    for ev in evs {
      let k = match ev {
        Event::Pressed(k) => k,
        Event::Released(k) => k,
      };
      
      let value = match ev {
        Event::Pressed(_) => 1,
        Event::Released(_) => 0
      };
      
      let code = (*k) as u16;
      
      send_type_code_value(1, code, value);
    }
    send_type_code_value(0, 0, 0);
    
    write(self.fd, &input_event_data.sink)?;
    
    Ok(())
  }
}

