
// vim: shiftwidth=2

use crate::keys::Layout;
use nix::Error;
use nix::errno::Errno::ENODEV;
use wildmatch::WildMatch;
use crate::key_transforms;
use crate::keyboard_listing::{filter_keyboards_verbose, list_keyboards, ExtractedKeyboard};
use crate::dev_input_rw::{DevInputReader, DevInputWriter, Exclusion};
use std::thread::{spawn, JoinHandle};
use std::sync::Mutex;
use std::sync::Arc;
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
use inotify::{
  Inotify,
  WatchMask
};

pub fn do_remapping_loop_all_devices(layout: &Layout, excludes: &[&str], verbose: bool) -> Result<(), String> {
  match list_keyboards() {
    Err(e) => Err(format!("Failed to get the list of keyboards: {}", e)),
    Ok(devs) => {
      let devs_with_exclusions = flag_excluded(devs, excludes);
      
      if verbose {
        eprintln!("Got the list of keyboards:");
        for dev in &devs_with_exclusions {
          let excluded_flag_text = if dev.excluded { " (excluded)" } else { "" };
          eprintln!(" * {:?}{}", dev.extracted_keyboard.dev_path, excluded_flag_text);
        }
      }
      
      let devs: Vec<ExtractedKeyboard> = devs_with_exclusions.into_iter()
        .filter(|e| !e.excluded)
        .map(|e| e.extracted_keyboard).collect();
      
      do_remapping_loop_these_devices(&devs.iter().map(|d| d.dev_path.clone()).collect(), layout, &None, verbose)
    }
  }
}

struct WorkingChild {
  dev_path: PathBuf,
  thread: JoinHandle<Result<(), String>>,
  done: Arc<Mutex<bool>>
}

pub fn do_remapping_loop_auto_all_devices(layout: &Layout, excludes: &[&str], verbose: bool) -> Result<(), String> {
  let mut inotify = Inotify::init().expect("Error initializing");
  inotify.add_watch("/dev/input", WatchMask::CREATE | WatchMask::ATTRIB)
    .expect("Failed to add watch");

  let mut children: Vec<WorkingChild> = Vec::new();
  
  loop {
    if verbose {
      eprintln!("Reaping finished devices");
    }
    for i in (0..children.len()).rev() {
      let done = *children[i].done.lock().unwrap();
      if verbose {
        eprintln!(" * {:?}: done={}", children[i].dev_path, done);
      }
      if done {
        match children.remove(i).thread.join().expect("Failed to join child") {
          Ok(_) => {
            if verbose {
              eprintln!("    Joined.");
            }
          },
          Err(msg) => {
            eprintln!("Error from child: {}", msg);
          }
        };
      }
    }
    
    if verbose {
      eprintln!("Getting the current list of keyboards");
    }
    
    match list_keyboards() {
      Err(e) => break Err(format!("Failed to get the list of keyboards: {}", e)),
      Ok(devs) => {
        let devs_with_exclusions = flag_excluded(devs, excludes);
        
        if verbose {
          eprintln!("Got the current list of keyboards:");
          for dev in &devs_with_exclusions {
            let excluded_flag_text = if dev.excluded { " (excluded)" } else { "" };
            eprintln!(" * {:?}{}", dev.extracted_keyboard.dev_path, excluded_flag_text);
          }
        }
        
        let devs: Vec<ExtractedKeyboard> = devs_with_exclusions.into_iter()
          .filter(|e| !e.excluded)
          .map(|e| e.extracted_keyboard).collect();
        
        if verbose {
          eprintln!("Checking which devices are already running:")
        }
        for dev in devs {
          let already_have_it = children.iter().any(|c| c.dev_path == dev.dev_path);
          if verbose { eprintln!(" * {:?}: {}", dev.dev_path, already_have_it); }
          if !already_have_it {
            match open_device(dev.dev_path.as_path(), &None) {
              Err(msg) => {
                eprintln!("Failed to open keyboard device: {}", msg)
              },
              Ok(mut driver) => {
                let done = Arc::new(Mutex::new(false));

                children.push(WorkingChild {
                  dev_path: dev.dev_path,
                  thread: {
                    let done = Arc::clone(&done);
                    let layout = layout.clone();
                    spawn(move || {
                      let res = do_remapping_loop_one_device(&mut driver, layout, verbose);
                      *done.lock().unwrap() = true;
                      res
                    })
                  },
                  done
                })
              }
            }
          }
        }
      }
    }
    
    let mut buffer = [0; 1024];
    inotify.read_events_blocking(&mut buffer).expect("Error reading events");
  }
}

pub fn do_remapping_loop_multiple_devices(devices: &Vec<&str>, skip_non_keyboard: bool, excludes: &[&str], layout: &Layout, tablet_mode_switch_device: &Option<&str>, verbose: bool) -> Result<(), String> {
  let devices = filter_devices_verbose(devices, skip_non_keyboard, excludes, verbose)?;

  do_remapping_loop_these_devices(
    &devices.into_iter().map(|p| Path::new(p).to_path_buf()).collect(),
    layout,
    &tablet_mode_switch_device.map(|p| Path::new(p).to_path_buf()),
    verbose
  )
}

let filter_devices_verbose<'s>(devices: &Vec<&'s str>, skip_non_keyboard: bool, excludes: &[&str]) -> Result<Vec<&'s str>, String> {
  
}

struct PossiblyExcludedDevice {
  extracted_keyboard: ExtractedKeyboard,
  excluded: bool
}

fn flag_excluded(devices: Vec<ExtractedKeyboard>, excludes: &[&str]) -> Vec<PossiblyExcludedDevice> {
  let wilds: Vec<WildMatch> = excludes.iter().map(|e| WildMatch::new(e)).collect();
  devices.into_iter().map(|d| {
    let excluded = wilds.iter().any(|w| w.matches(&d.name));
    PossiblyExcludedDevice {
      extracted_keyboard: d,
      excluded
    }
  }).collect()
}

fn open_device(path: &Path, tablet_mode_switch_device: &Option<PathBuf>) -> Result<RealDriver, String> {
  let r = match DevInputReader::open(path, Exclusion::WaitReleaseAndExclude, true) {
    Err(e) => Err(format!("Failed to open {:?} for reading: {}", path, e)),
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
  
  let rw = RW { r, w, t };
  
  Ok(RealDriver { rw })
}

pub fn do_remapping_loop_these_devices(devices: &Vec<PathBuf>, layout: &Layout, tablet_mode_switch_device: &Option<PathBuf>, verbose: bool) -> Result<(), String> {
  if verbose { eprintln!("Remapping {} devices.", devices.len()); }
  
  let mut drivers: Vec<RealDriver> = Vec::new();
  
  for p in devices {
    if verbose { eprintln!(" * {}", p.to_string_lossy()); }
    drivers.push(open_device(p.as_path(), tablet_mode_switch_device)?);
  }
  
  let mut threads: Vec<JoinHandle<Result<(), String>>> = Vec::new();
  for mut driver in drivers.drain(..) {
    let local_layout = layout.clone();
    threads.push(spawn(move || {
      do_remapping_loop_one_device(&mut driver, local_layout, verbose)
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
    keys: Vec<KeyCode>,
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
    
    Ok(RealPollRegistry { poll, events })
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

fn do_remapping_loop_one_device(driver: &mut impl Driver, layout: Layout, verbose: bool) -> Result<(), String> {
  let mut mapper = key_transforms::Mapper::for_layout(&layout);
  let mut working_repeat: WorkingRepeat = WorkingRepeat::Idle;
  
  let mut poll = driver.register_poll()?;
  
  let mut in_tablet_mode: bool = false;
  let mut restart_count: i32 = 0;
  
  if verbose { eprintln!("Starting remapping loop."); }
  
  loop {
    loop {
      let timeout = match working_repeat {
        WorkingRepeat::Idle => None,
        WorkingRepeat::Repeating { keys: _, next_wakeup, interval_ms: _ } => {
          let now = Instant::now();
          if now >= next_wakeup {
            Some(Duration::from_millis(1))
          }
          else {
            Some(next_wakeup - now)
          }
        }
      };
      
      match driver.poll(&mut poll, timeout)? {
        PollResult::TimedOut => {
          match working_repeat {
            WorkingRepeat::Idle => {
              // Well that's weird. I guess just keep going?
            },
            WorkingRepeat::Repeating { keys, next_wakeup, interval_ms } => {
              if !in_tablet_mode {
                let mut repeat_send = Vec::new();
                for key in &keys {
                  repeat_send.push(Pressed(*key));
                }
                for key in (&keys).iter().rev() {
                  repeat_send.push(Released(*key));
                }
                driver.send(&repeat_send)?;
                working_repeat = WorkingRepeat::Repeating {
                  keys,
                  next_wakeup: next_wakeup + Duration::from_millis(interval_ms as u64),
                  interval_ms
                };
              }
              else {
                working_repeat = WorkingRepeat::Idle;
              }
            }
          };
        },
        PollResult::Interrupted => {
          if verbose { eprintln!("poll() interrupted"); }
          restart_count += 1;
          if restart_count > 1 {
            // Avoid burning the CPU if we keep getting interrupted for some reason
            thread::sleep(Duration::from_millis(1000 * (1 << restart_count)));
          }
        },
        PollResult::DeviceEvent(dev_evs) => {
          restart_count = 0;
          for dev_ev in dev_evs {
            match dev_ev {
              Device::Keyboard => {
                loop {
                  match driver.next_keyboard()? {
                    Next::Busy => {
                      break;
                    }
                    Next::End => {
                      if verbose { eprintln!("Ending remapping loop because no more keyboard events."); }
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
                          ResultingRepeat::Repeating { keys, delay_ms, interval_ms } => WorkingRepeat::Repeating {
                            keys,
                            next_wakeup: Instant::now() + Duration::from_millis(delay_ms as u64),
                            interval_ms
                          },
                          ResultingRepeat::Disabled => WorkingRepeat::Idle,
                          ResultingRepeat::NoChange => working_repeat
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
    
    let mut driver = TestDriver { ops };
    do_remapping_loop_one_device(&mut driver, layout, true).unwrap();
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
    
    let mut driver = TestDriver { ops };
    do_remapping_loop_one_device(&mut driver, layout, true).unwrap();
    driver.finish();
  }
  
  #[test]
  fn test_remapping_loop_repeat_1() {
    let layout = Layout {
      mappings: vec![
        Mapping { from: vec![A], to: vec![A], repeat: Repeat::Disabled, ..Default::default() },
        Mapping { from: vec![B], to: vec![B], repeat: Repeat::Special { keys: vec![C], delay_ms: 130, interval_ms: 30 }, ..Default::default() },
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
    
    let mut driver = TestDriver { ops };
    do_remapping_loop_one_device(&mut driver, layout, true).unwrap();
    driver.finish();
  }
  
  #[test]
  fn test_remapping_loop_repeat_2() {
    let layout = Layout {
      mappings: vec![
        Mapping { from: vec![A], to: vec![A], repeat: Repeat::Disabled, ..Default::default() },
        Mapping { from: vec![B], to: vec![B], repeat: Repeat::Special { keys: vec![C], delay_ms: 130, interval_ms: 30 }, ..Default::default() },
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
    
    let mut driver = TestDriver { ops };
    do_remapping_loop_one_device(&mut driver, layout, true).unwrap();
    driver.finish();
  }
  
  #[test]
  fn test_remapping_loop_repeat_3() {
    let layout = Layout {
      mappings: vec![
        Mapping { from: vec![A], to: vec![A], repeat: Repeat::Disabled, ..Default::default() },
        Mapping { from: vec![B], to: vec![B], repeat: Repeat::Special { keys: vec![LEFTCTRL, C], delay_ms: 130, interval_ms: 30 }, ..Default::default() },
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
    ops.push_back(TestOp::Send { evs: vec![Pressed(LEFTCTRL), Pressed(C), Released(C), Released(LEFTCTRL)] });
    
    ops.push_back(TestOp::Poll { timeout: Some(Duration::from_millis(160)), result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::One(Pressed(D)) });
    ops.push_back(TestOp::Send { evs: vec![Pressed(D)] });
    ops.push_back(TestOp::NextKeyboard { result: Next::Busy });
    
    ops.push_back(TestOp::Poll { timeout: None, result: PollResult::DeviceEvent(vec![Device::Keyboard]) });
    ops.push_back(TestOp::NextKeyboard { result: Next::End });
    
    let mut driver = TestDriver { ops };
    do_remapping_loop_one_device(&mut driver, layout, true).unwrap();
    driver.finish();
  }
}

