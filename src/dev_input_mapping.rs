
// vim: shiftwidth=2
 
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, write};
use nix::Error;
use libc::{input_event};
use std::mem::size_of;
use uinput_sys::{ui_set_evbit, EV_SYN, EV_KEY, EV_MSC, ui_dev_create, ui_set_keybit, KEY_MAX, KEY_7};
use crate::struct_ser::StructSerializer;
use std::os::unix::io::RawFd;
use nix::ioctl_write_int;

pub struct TypeCodeValue {
  pub type_: u16,
  pub code: u16,
  pub value: i32,
}

pub struct DevInputReader {
  fd: RawFd
}

impl DevInputReader {
  pub fn read_next(self: &mut DevInputReader) -> TypeCodeValue {
    let size = size_of::<input_event>();
    let mut buf: Vec<u8> = vec![0; size];
    read(self.fd, &mut buf).unwrap();
    
    let type_ = u16::from_ne_bytes([buf[16], buf[17]]);
    let code = u16::from_ne_bytes([buf[18], buf[19]]);
    let value = i32::from_ne_bytes([buf[20], buf[21], buf[22], buf[23]]);
    
    TypeCodeValue {
      type_: type_,
      code: code,
      value: value
    }
  }
  
  pub fn open(path: &str, exclusive: bool) -> Result<DevInputReader, Error> {
    let fd = open(path, OFlag::O_RDONLY, Mode::empty())?;
    
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

pub fn read_test() {
  println!("OK!");
  
  let input_path = "/dev/input/by-path/pci-0000:00:14.0-usb-0:1:1.0-event-kbd";
  
  let input_fd = open(input_path, OFlag::O_RDONLY, Mode::empty()).unwrap();
  
  loop {
    let size = size_of::<input_event>();
    let mut buf: Vec<u8> = vec![0; size];
    read(input_fd, &mut buf).unwrap();
    
    let type_ = u16::from_ne_bytes([buf[16], buf[17]]);
    let code = u16::from_ne_bytes([buf[18], buf[19]]);
    let value = i32::from_ne_bytes([buf[20], buf[21], buf[22], buf[23]]);
    
    println!("{:?} {:?} {:?}", type_, code, value);
  }
}

pub fn write_test() {
  let fdo = open("/dev/uinput", OFlag::O_WRONLY | OFlag::O_NONBLOCK, Mode::empty()).unwrap();

  unsafe {
    ui_set_evbit(fdo, EV_SYN);
    ui_set_evbit(fdo, EV_KEY);
    ui_set_evbit(fdo, EV_MSC);
  }
  
  for i in 0 .. KEY_MAX {
    unsafe { ui_set_keybit(fdo, i); }
  }
  
  {
    let mut user_dev_data = StructSerializer {
      sink: Vec::new()
    };
    
    user_dev_data.add_string_in_buf("write_test", 80);
    
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
  
  let send_type_code_value = |type_, code, value| {
    let mut input_event_data = StructSerializer {
      sink: Vec::new()
    };
    
    input_event_data.add_i64(0);
    input_event_data.add_i64(0);
    input_event_data.add_u16(type_);
    input_event_data.add_u16(code);
    input_event_data.add_i32(value);
    
    write(fdo, &input_event_data.sink)
  };
  
  let send_full_set = |code, value| {
    send_type_code_value(4, 4, code as u32)?;
    send_type_code_value(1, code, value)?;
    send_type_code_value(0, 0, 0)
  };
  
  let pressed = 1;
  let released = 0;
  
  for _i in 0 .. 10 {
    std::thread::sleep(std::time::Duration::new(1, 0));
    send_full_set(KEY_7 as u16, pressed).unwrap();
    send_full_set(KEY_7 as u16, released).unwrap();
  }
}

