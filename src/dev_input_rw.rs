
use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};
use nix::errno::Errno;
// vim: shiftwidth=2
 
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, write};
use nix::Error;
use libc::input_event;
use std::mem::size_of;
use uinput_sys::{ui_set_evbit, EV_SYN, EV_KEY, EV_MSC, ui_dev_create, ui_set_keybit};
use crate::struct_ser::StructSerializer;
use std::os::unix::io::RawFd;
use crate::keys::Event;
use num_traits::FromPrimitive;
use std::path::Path;
use ioctls::{eviocgkey, eviocgrab};

pub struct DevInputReader {
  pub fd: RawFd
}

pub enum Exclusion {
  NoExclusion,
  #[allow(dead_code)]
  ImmediateExclusion,
  // Waits for all keys to be released. Could block for a long time.
  WaitReleaseAndExclude
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
  
  pub fn open(path: &Path, exclusion: Exclusion, nonblock: bool) -> Result<DevInputReader, Error> {
    let fd = open(path, if nonblock {OFlag::O_RDONLY | OFlag::O_NONBLOCK} else {OFlag::O_RDONLY}, Mode::empty())?;
    
    match exclusion {
      Exclusion::NoExclusion => { },
      Exclusion::ImmediateExclusion => {
        unsafe {
          if eviocgrab(fd, &*Box::new(1)) == -1 {
            return Err(Error::last());
          }
        }
      },
      Exclusion::WaitReleaseAndExclude => {
        do_exclusion_loop(fd)?;
        unsafe {
          if eviocgrab(fd, &*Box::new(1)) == -1 {
            return Err(Error::last());
          }
        }
      }
    };
    
    Ok(DevInputReader { fd })
  }
}

fn do_exclusion_loop(fd: RawFd) -> Result<(), Error> {
  let num_bytes = ( (uinput_sys::KEY_MAX + 7) / 8 ) as usize;
  let mut bytes = vec![0u8; num_bytes];

  loop {
    unsafe {
      // Get which keys are currently pressed
      if eviocgkey(fd, bytes.as_mut_ptr(), bytes.len()) == -1 {
        return Err(Error::last());
      }
    }

    let all_zero = bytes.iter().all(|x| *x == 0);

    if all_zero {
      break;
    }
    else {
      // Don't check again until the next key event; otherwise, it is pointless to check.
      wait_for_any_activity(fd)?;
      bytes.fill(0);
    }
  }

  Ok(())
}

fn wait_for_any_activity(fd: RawFd) -> Result<(), Error> {
  let size = size_of::<input_event>();
  let mut buf: Vec<u8> = vec![0; size];
  match read(fd, &mut buf) {
    Err(e) => match e {
      Error::Sys(code) => match code {
        nix::errno::Errno::EAGAIN => {
          let mut poll = Poll::new().unwrap();
          poll.registry().register(&mut SourceFd(&fd), Token(0), Interest::READABLE).unwrap();
          let mut events = Events::with_capacity(24);
          match poll.poll(&mut events, None) {
            Err(e) => {
              return Err(Error::Sys(Errno::from_i32(e.raw_os_error().unwrap_or(0))));
            },
            Ok(_) => ()
          };
        },
        _ => {
          return Err(e);
        }
      },
      _ => {
        return Err(e);
      }
    },
    Ok(_) => ()
  };
  
  Ok(())
}

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

