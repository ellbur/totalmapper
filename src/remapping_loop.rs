
// vim: shiftwidth=2

use crate::keys::Layout;
use nix::Error;
use nix::errno::Errno::ENODEV;
use crate::key_transforms;
use crate::keyboard_listing::{filter_keyboards, list_keyboards};
use crate::dev_input_rw::{DevInputReader, DevInputWriter};
use std::thread::{spawn, JoinHandle};
use std::path::{Path, PathBuf};
use nix::errno::Errno::EAGAIN;
use mio::{Interest, Poll, Token, Events};
use mio::unix::SourceFd;
use crate::tablet_mode_switch_reader::TabletModeSwitchReader;
use crate::tablet_mode_switch_reader::TableModeEvent::{On, Off};
use std::thread;
use std::time;
use crate::keys::KeyCode;
use time::{Duration, Instant};
use crate::keys::Event;
use crate::keys::Event::{Pressed, Released};
use crate::key_transforms::ResultingRepeat;
use crate::tablet_mode_switch_reader::TableModeEvent;

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
    
    let t = match tablet_mode_switch_device {
      None => Ok(None),
      Some(path) => match TabletModeSwitchReader::open(path, true) {
        Err(e) => Err(format!("Failed to open tablet mode device {:?} for reading: {}", path, e)),
        Ok(t) => Ok(Some(t))
      }
    }?;
    
    rws.push(RW {
      r: r,
      w: w,
      t: t
    });
  }
  
  let mut threads: Vec<JoinHandle<Result<(), String>>> = Vec::new();
  for rw in rws.drain(..) {
    let local_layout = layout.clone();
    threads.push(spawn(move || {
      let mut driver = RealDriver {
        rw: rw
      };
      do_remapping_loop_one_device(&mut driver, local_layout)
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
  w: DevInputWriter,
  t: Option<TabletModeSwitchReader>
}
  
#[derive(Debug)]
enum WorkingRepeat {
  Idle,
  Repeating {
    key: KeyCode,
    next_wakeup: Instant,
    interval_ms: i32
  },
}

#[derive(Debug)]
enum Device {
  Keyboard,
  Tablet
}

#[derive(Debug)]
enum PollResult {
  DeviceEvent(Vec<Device>),
  TimedOut,
  Interrupted
}

trait Driver {
  type PollRegistry;
  fn register_poll(&mut self) -> Result<Self::PollRegistry, String>;
  fn poll(&mut self, registry: &mut Self::PollRegistry, timeout: Option<Duration>) -> Result<PollResult, String>;
  fn next_keyboard(&mut self) -> Result<Next<Event>, String>;
  fn next_tablet(&mut self) -> Result<Next<TableModeEvent>, String>;
  fn send(&mut self, evs: &Vec<Event>) -> Result<(), String>;
}

struct RealDriver {
  rw: RW
}

struct RealPollRegistry {
  poll: Poll,
  events: Events
}

const KEYBOARD: Token = Token(0);
const TABLET_SWITCH: Token = Token(1);

#[derive(Debug)]
enum Next<T> {
  End,
  Busy,
  One(T)
}
  
impl Driver for RealDriver {
  type PollRegistry = RealPollRegistry;
  
  fn register_poll(&mut self) -> Result<RealPollRegistry, String> {
    let poll = Poll::new().unwrap();
    poll.registry().register(&mut SourceFd(&self.rw.r.fd), KEYBOARD, Interest::READABLE).unwrap();
    
    match &self.rw.t {
      None => (),
      Some(t) => {
        poll.registry().register(&mut SourceFd(&t.fd), TABLET_SWITCH, Interest::READABLE).unwrap();
      }
    }
    
    let events = Events::with_capacity(24);
    
    Ok(RealPollRegistry {
      poll: poll,
      events: events
    })
  }
  
  fn poll(&mut self, registry: &mut RealPollRegistry, timeout: Option<Duration>) -> Result<PollResult, String>  {
    match registry.poll.poll(&mut registry.events, timeout) {
      Ok(_) => {
        let mut res: Vec<Device> = Vec::new();
        
        for event in registry.events.iter() {
          match event.token() {
            KEYBOARD => {
              res.push(Device::Keyboard)
            },
            TABLET_SWITCH => {
              res.push(Device::Tablet)
            },
            Token(_) => {
            }
          }
        }
        
        if res.is_empty() {
          Ok(PollResult::TimedOut)
        }
        else {
          Ok(PollResult::DeviceEvent(res))
        }
      },
      Err(e) => {
        match e.kind() {
          std::io::ErrorKind::TimedOut => {
            Ok(PollResult::TimedOut)
          },
          std::io::ErrorKind::Interrupted => {
            Ok(PollResult::Interrupted)
          },
          _ => {
            Err(format!("poll failed: {}", e))
          }
        }
      }
    }
  }
  
  fn next_keyboard(&mut self) -> Result<Next<Event>, String> {
    match self.rw.r.next() {
      Err(Error::Sys(EAGAIN)) => Ok(Next::Busy),
      Err(Error::Sys(ENODEV)) => Ok(Next::End),
      Err(e) => Err(format!("read() from keyboard failed with {}", e)),
      Ok(ev) => Ok(Next::One(ev))
    }
  }
  
  fn next_tablet(&mut self) -> Result<Next<TableModeEvent>, String> {
    match &mut self.rw.t {
      Some(t) => {
        match t.next() {
          Err(Error::Sys(EAGAIN)) => Ok(Next::Busy),
          Err(Error::Sys(ENODEV)) => Ok(Next::End),
          Err(e) => Err(format!("read() from tablet mode switch failed with {}", e)),
          Ok(ev) => Ok(Next::One(ev))
        }
      },
      None => Ok(Next::End)
    }
  }
  
  fn send(&mut self, evs: &Vec<Event>) -> Result<(), String> {
    match self.rw.w.send(evs) {
      Err(e) => {
        Err(format!("write() to synthetic keyboard failed with {}", e))
      },
      Ok(_) => Ok(())
    }
  }
}

fn do_remapping_loop_one_device(driver: &mut impl Driver, layout: Layout) -> Result<(), String> {
  let mut mapper = key_transforms::Mapper::for_layout(&layout);
  let mut working_repeat: WorkingRepeat = WorkingRepeat::Idle;
  
  let mut poll = driver.register_poll()?;
  
  let mut in_tablet_mode: bool = false;
  
  loop {
    loop {
      let timeout = match working_repeat {
        WorkingRepeat::Idle => None,
        WorkingRepeat::Repeating { key: _, next_wakeup, interval_ms: _ } => {
          let now = Instant::now();
          if now >= next_wakeup {
            Some(Duration::from_millis(1))
          }
          else {
            Some(next_wakeup - now)
          }
        }
      };
      
      let mut restart_count: i32 = 0;
      match driver.poll(&mut poll, timeout)? {
        PollResult::TimedOut => {
          match working_repeat {
            WorkingRepeat::Idle => {
              // Well that's weird. I guess just keep going?
            },
            WorkingRepeat::Repeating { key, next_wakeup, interval_ms } => {
              if !in_tablet_mode {
                driver.send(&vec![Pressed(key), Released(key)])?;
                working_repeat = WorkingRepeat::Repeating {
                  key: key,
                  next_wakeup: next_wakeup + Duration::from_millis(interval_ms as u64),
                  interval_ms: interval_ms
                };
              }
              else {
                working_repeat = WorkingRepeat::Idle;
              }
            }
          };
        },
        PollResult::Interrupted => {
          restart_count += 1;
          if restart_count > 1 {
            // Avoid burning the CPU if we keep getting interrupted for some reason
            thread::sleep(Duration::from_millis(1000 * (1 << restart_count)));
          }
        },
        PollResult::DeviceEvent(dev_evs) => {
          for dev_ev in dev_evs {
            match dev_ev {
              Device::Keyboard => {
                loop {
                  match driver.next_keyboard()? {
                    Next::Busy => {
                      break;
                    }
                    Next::End => {
                      return Ok(());
                    }
                    Next::One(ev_in) => {
                      if !in_tablet_mode {
                        let step_out = mapper.step(ev_in);
                        let evs_out = step_out.events;

                        if !evs_out.is_empty() {
                          driver.send(&evs_out)?;
                        }
                        
                        working_repeat = match step_out.repeat {
                          Some(ResultingRepeat { key, delay_ms, interval_ms }) => WorkingRepeat::Repeating {
                            key: key,
                            next_wakeup: Instant::now() + Duration::from_millis(delay_ms as u64),
                            interval_ms: interval_ms
                          },
                          None => WorkingRepeat::Idle
                        };
                      }
                    }
                  }
                }
              },
              Device::Tablet => {
                loop {
                  match driver.next_tablet()? {
                    Next::Busy => {
                      break;
                    },
                    Next::End => {
                      return Ok(());
                    },
                    Next::One(ev_in) => {
                      match ev_in {
                        On => {
                          in_tablet_mode = true;
                          working_repeat = WorkingRepeat::Idle;
                          let release_events = mapper.release_all();
                          if !release_events.is_empty() {
                            driver.send(&release_events)?;
                          }
                        },
                        Off => {
                          in_tablet_mode = false;
                          working_repeat = WorkingRepeat::Idle;
                          let release_events = mapper.release_all();
                          if !release_events.is_empty() {
                            driver.send(&release_events)?;
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::VecDeque;
  use KeyCode::*;
  use std::default::Default;
  use crate::keys::{Layout, Mapping, KeyCode, Pressed, Released, Event, Repeat};
  
  #[derive(Debug)]
  enum TestOp {
    RegisterPoll,
    Poll {
      timeout: Option<Duration>,
      result: PollResult
    },
    NextKeyboard {
      result: Next<Event>
    },
    NextTablet {
      result: Next<TableModeEvent>
    },
    Send {
      evs: Vec<Event>
    }
  }
  
  struct TestPollRegistry {
  }
  
  struct TestDriver {
    ops: VecDeque<TestOp>
  }
  
  impl TestDriver {
    fn finish(&self) {
      assert!(self.ops.is_empty());
    }
  }
  
  impl Driver for TestDriver {
    type PollRegistry = TestPollRegistry;
    
    fn register_poll(&mut self) -> Result<TestPollRegistry, String> {
      match self.ops.pop_front() {
        None => {
          panic!("register_poll() on empty op list")
        },
        Some(TestOp::RegisterPoll) => {
          Ok(TestPollRegistry { })
        },
        Some(other) => {
          panic!("register_poll() called but should have called {:?}", other)
        }
      }
    }
    
    fn poll(&mut self, _registry: &mut Self::PollRegistry, timeout: Option<Duration>) -> Result<PollResult, String> {
      match self.ops.pop_front() {
        None => {
          panic!("poll() on empty op list")
        },
        Some(TestOp::Poll { timeout: timeout_should, result }) => {
          match (timeout, timeout_should) {
            (None, None) => (),
            (None, Some(_)) => {
            },
            (Some(_), None) => {
            },
            (Some(d1), Some(d2)) => {
              let error = (d1.as_millis() as i32) - (d2.as_millis() as i32);
              if error > 10 || error < -10 {
                panic!("Timeout was {:?}, should be {:?}", timeout, timeout_should);
              }
            }
          }
          Ok(result)
        },
        Some(other) =>  {
          panic!("poll() called but should have called {:?}", other)
        }
      }
    }
    
    fn next_keyboard(&mut self) -> Result<Next<Event>, String> {
      match self.ops.pop_front() {
        None => {
          panic!("next_keyboard() on empty op list")
        },
        Some(TestOp::NextKeyboard { result }) => {
          Ok(result)
        },
        Some(other) => {
          panic!("next_keyboard() called but should have called {:?}", other)
        }
      }
    }
    
    fn next_tablet(&mut self) -> Result<Next<TableModeEvent>, String> {
      match self.ops.pop_front() {
        None => {
          panic!("next_tablet() on empty op list")
        },
        Some(TestOp::NextTablet { result }) => {
          Ok(result)
        },
        Some(other) => {
          panic!("next_tablet() called but should have called {:?}", other)
        }
      }
    }
    
    fn send(&mut self, evs: &Vec<Event>) -> Result<(), String> {
      match self.ops.pop_front() {
        None => {
          panic!("send() on empty op list")
        },
        Some(TestOp::Send { evs: evs_should }) => {
          assert_eq!(*evs, evs_should);
          Ok(())
        },
        Some(other) => {
          panic!("send({:?}) called but should have called {:?}", evs, other)
        }
      }
    }
  }
  
  #[test]
  fn test_remapping_loop_basic() {
    let layout = Layout {
      mappings: vec![
        Mapping { from: vec![A], to: vec![B], ..Default::default() },
      ]
    };
    
    let mut ops: VecDeque<TestOp> = VecDeque::new();
    ops.push_back(TestOp::RegisterPoll);
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(A)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(B)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Released(A)) });
    ops.push_back(TestOp::Send { evs: vec![Released(B)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::End });
    
    let mut driver = TestDriver { ops: ops };
    do_remapping_loop_one_device(&mut driver, layout).unwrap();
    driver.finish();
  }
  
  #[test]
  fn test_remapping_loop_tablet() {
    let layout = Layout {
      mappings: vec![
        Mapping { from: vec![A], to: vec![B], ..Default::default() },
      ]
    };
    
    let mut ops: VecDeque<TestOp> = VecDeque::new();
    ops.push_back(TestOp::RegisterPoll);
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Tablet]) });
    ops.push_back(TestOp::NextTablet { result: Next::One(TableModeEvent::On) });
    ops.push_back(TestOp::NextTablet { result: Next::Busy });
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(A)) });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Released(A)) });
    ops.push_back(TestOp::NextKeyboard { result: Next::End });
    
    let mut driver = TestDriver { ops: ops };
    do_remapping_loop_one_device(&mut driver, layout).unwrap();
    driver.finish();
  }
  
  #[test]
  fn test_remapping_loop_repeat_1() {
    let layout = Layout {
      mappings: vec![
        Mapping { from: vec![A], to: vec![A], repeat: Repeat::Disabled, ..Default::default() },
        Mapping { from: vec![B], to: vec![B], repeat: Repeat::Special { key: C, delay_ms: 130, interval_ms: 30 }, ..Default::default() },
      ]
    };
    
    let mut ops: VecDeque<TestOp> = VecDeque::new();
    ops.push_back(TestOp::RegisterPoll);
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(LEFTSHIFT)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(LEFTSHIFT)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(A)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(A), Released(A)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Released(A)) });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(B)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(B), Released(B)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: Some(Duration::from_millis(130)), result: PollResult::TimedOut });
    ops.push_back(TestOp::Send { evs: vec![Pressed(C), Released(C)] });
    
    ops.push_back(TestOp::Poll { timeout: Some(Duration::from_millis(160)), result: PollResult::TimedOut });
    ops.push_back(TestOp::Send { evs: vec![Pressed(C), Released(C)] });
    
    ops.push_back(TestOp::Poll { timeout: Some(Duration::from_millis(190)), result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Released(B)) });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Released(LEFTSHIFT)) });
    ops.push_back(TestOp::Send { evs: vec![Released(LEFTSHIFT)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::End });
    
    let mut driver = TestDriver { ops: ops };
    do_remapping_loop_one_device(&mut driver, layout).unwrap();
    driver.finish();
  }
  
  #[test]
  fn test_remapping_loop_repeat_2() {
    let layout = Layout {
      mappings: vec![
        Mapping { from: vec![A], to: vec![A], repeat: Repeat::Disabled, ..Default::default() },
        Mapping { from: vec![B], to: vec![B], repeat: Repeat::Special { key: C, delay_ms: 130, interval_ms: 30 }, ..Default::default() },
      ]
    };
    
    let mut ops: VecDeque<TestOp> = VecDeque::new();
    ops.push_back(TestOp::RegisterPoll);
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(LEFTSHIFT)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(LEFTSHIFT)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(A)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(A), Released(A)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Released(A)) });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(B)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(B), Released(B)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: Some(Duration::from_millis(130)), result: PollResult::TimedOut });
    ops.push_back(TestOp::Send { evs: vec![Pressed(C), Released(C)] });
    
    ops.push_back(TestOp::Poll { timeout: Some(Duration::from_millis(160)), result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(D)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(D)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::End });
    
    let mut driver = TestDriver { ops: ops };
    do_remapping_loop_one_device(&mut driver, layout).unwrap();
    driver.finish();
  }
}

