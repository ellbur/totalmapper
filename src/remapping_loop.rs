
// vim: shiftwidth=2

use crate::keys::Layout;
use nix::Error;
use nix::Error::Sys;
use nix::errno::Errno::ENODEV;
use crate::dev_input_rw;
use crate::key_transforms;

pub fn do_remapping_loop_all_devices() -> Result<(), Error> {
  do_remapping_loop_multiple_devices()
}

pub fn do_remapping_loop_multiple_devices(devices: &[&str]) -> Result<(), Error> {
}

pub fn do_remapping_loop(device_path: String, layout: &Layout) -> Result<(), Error> {
  let mut r = dev_input_rw::DevInputReader::open(&device_path, true)?;
  let mut w = dev_input_rw::DevInputWriter::open()?;

  let mut mapper = key_transforms::Mapper::for_layout(&layout);
  
  loop {
    match r.next() {
      Err(e) => {
        match e {
          Sys(ENODEV) => return Ok(()),
          _ => return Err(e)
        }
      }
      Ok(ev_in) => {
        let evs_out = mapper.step(ev_in);
        
        for ev in evs_out {
          w.send(ev)?;
        }
      }
    }
  }
}

