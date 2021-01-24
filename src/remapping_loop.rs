
// vim: shiftwidth=2

use crate::keys::Layout;
use nix::Error;
use nix::Error::Sys;
use nix::errno::Errno::ENODEV;
use crate::key_transforms;
use crate::keyboard_listing::{filter_keyboards, list_keyboards};
use crate::dev_input_rw::{DevInputReader, DevInputWriter};
use std::thread::{spawn, JoinHandle};
use std::path::{Path, PathBuf};
use nix::errno::Errno::EAGAIN;
use mio::{Interest, Poll, Token, Events};
use mio::unix::SourceFd;
use std::time;

pub fn do_remapping_loop_all_devices(layout: &Layout) -> Result<(), String> {
  match list_keyboards() {
    Err(e) => Err(format!("Failed to get the list of keyboards: {}", e)),
    Ok(devs) => {
      do_remapping_loop_these_devices(&devs, layout, &None)
    }
  }
}

pub fn do_remapping_loop_multiple_devices(devices: &Vec<&str>, skip_non_keyboard: bool, layout: &Layout, tablet_mode_switch_device: &Option<&str>) -> Result<(), String> {
  if skip_non_keyboard {
    match filter_keyboards(devices) {
      Err(e) => Err(format!("Failed to get list of devices: {}", e)),
      Ok(devs) => do_remapping_loop_these_devices(
        &devs.into_iter().map(|p| Path::new(p).to_path_buf()).collect(),
        layout,
        &tablet_mode_switch_device.map(|p| Path::new(p).to_path_buf())
      )
    }
  }
  else {
    do_remapping_loop_these_devices(
      &devices.into_iter().map(|p| Path::new(p).to_path_buf()).collect(),
      layout,
      &tablet_mode_switch_device.map(|p| Path::new(p).to_path_buf())
    )
  }
}

pub fn do_remapping_loop_these_devices(devices: &Vec<PathBuf>, layout: &Layout, tablet_mode_switch_device: &Option<PathBuf>) -> Result<(), String> {
  let mut rws: Vec<RW> = Vec::new();
  
  for p in devices {
    let r = match DevInputReader::open(p.as_path(), true, true) {
      Err(e) => Err(format!("Failed to open {:?} for reading: {}", p, e)),
      Ok(r) => Ok(r)
    }?;
    
    let w = match DevInputWriter::open() {
      Err(e) => Err(format!("Failed to open /dev/uinput for writing: {}", e)),
      Ok(w) => Ok(w)
    }?;
    
    rws.push(RW {
      r: r,
      w: w
    });
  }
  
  let mut threads: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
  for mut rw in rws.drain(..) {
    let local_layout = layout.clone();
    threads.push(spawn(move || {
      do_remapping_loop_one_device(&mut rw, local_layout)
    }));
  }
  
  for th in threads {
    match th.join() {
      Err(_) => Err(format!("Joining the thread failed.")),
      Ok(r) => match r {
        Err(e) => Err(format!("Mapping failed: {}", e)),
        Ok(u) => Ok(u)
      }
    }?;
  }
  
  Ok(())
}

struct RW {
  r: DevInputReader,
  w: DevInputWriter
}
  
fn do_remapping_loop_one_device(rw: &mut RW, layout: Layout) -> Result<(), Error> {
  let mut mapper = key_transforms::Mapper::for_layout(&layout);
  
  const KEYBOARD: Token = Token(0);
  const TABLET_SWITCH: Token = Token(1);
  
  let mut poll = Poll::new().unwrap();
  poll.registry().register(&mut SourceFd(&rw.r.fd), KEYBOARD, Interest::READABLE).unwrap();
  
  let mut events = Events::with_capacity(24);

  loop {
    poll.poll(&mut events, None).unwrap();
    
    for event in events.iter() {
      match event.token() {
        KEYBOARD => {
          loop {
            match rw.r.next() {
              Err(Error::Sys(EAGAIN)) => {
                break;
              }
              Err(Error::Sys(ENODEV)) => {
                return Ok(());
              }
              Err(e) => {
                return Err(e);
              }
              Ok(ev_in) => {
                let evs_out = mapper.step(ev_in);

                for ev in evs_out {
                  rw.w.send(ev)?;
                }
              }
            }
          }
        }
        Token(_) => {
        }
      }
    }
  }
}

