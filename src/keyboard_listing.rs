
// vim: shiftwidth=2

use std::fs::{File, canonicalize, read_to_string};
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use crate::key_codes::KeyCode;

pub fn list_keyboards_to_stdout() -> io::Result<()> {
  for p in list_keyboards()? {
    println!("{}: {}", p.name, p.dev_path.to_string_lossy());
  }
  
  Ok(())
}

struct ExtractedProcBusKeyboard {
  sysfs_path: String,
  name: String
}

pub struct ExtractedKeyboard {
  pub dev_path: PathBuf,
  pub name: String
}

fn parse_mask_hex(hex: &str) -> Result<HashSet<i32>, ParseIntError> {
  let tokens = hex.rsplit(' ');
  
  let mut res = HashSet::new();
  
  let mut token_index = 0;
  for token in tokens {
    let num = u64::from_str_radix(token, 16)?;
    for i in 0u8 .. 63u8 {
      let mask = 1u64 << i;
      if (num & mask) != 0 {
        res.insert((i as i32) + token_index * 64);
      }
    }
    token_index += 1;
  }
  
  Ok(res)
}

fn extract_keyboards_from_proc_bus_input_devices(proc_bus_input_devices: &str) -> Vec<ExtractedProcBusKeyboard> {
  let mut res = Vec::new();
  let lines = proc_bus_input_devices.split('\n');
  
  let mut working_sysfs_path = Box::new(None);
  let mut working_name = Box::new(None);
  let mut working_ev_mask = Box::new(None);
  
  for line in lines {
    if line.starts_with("I:") {
      *working_sysfs_path = None;
      *working_name = None;
      *working_ev_mask = None;
    }
    else if line.starts_with("S: Sysfs=") {
      let new_sysfs_path = line[9..].to_string();
      *working_sysfs_path = Some(new_sysfs_path);
    }
    else if line.starts_with("N: Name=\"") {
      let mut name = line[9..].to_string();
      name = name.trim_end().to_string();
      if name.ends_with('"') {
        name = name[..name.len()-1].to_string();
      }
      *working_name = Some(name);
    }
    else if line.starts_with("B: EV=") {
      *working_ev_mask = Some(line[6..].to_string());
    }
    else if line.starts_with("B: KEY=") {
      let mut num_keys = 0;
      for c in line[7..].chars() {
        num_keys += match c {
          '0' => 0, '1' => 1, '2' => 1, '3' => 2,
          '4' => 1, '5' => 2, '6' => 2, '7' => 3,
          '8' => 1, '9' => 2, 'a' => 2, 'b' => 3,
          'c' => 2, 'd' => 3, 'e' => 3, 'f' => 4,
          _ => 0
        }
      }
      
      let key_set = parse_mask_hex(&line[7..]).unwrap_or(HashSet::new());
      
      let ev_set = match &*working_ev_mask {
        None => HashSet::new(),
        Some(mask_hex) => {
          parse_mask_hex(mask_hex.as_str()).unwrap_or(HashSet::new())
        }
      };
      
      let num_normal_keys = 
          (key_set.contains(&(KeyCode::A as i32)) as i32)
        + (key_set.contains(&(KeyCode::B as i32)) as i32)
        + (key_set.contains(&(KeyCode::C as i32)) as i32)
        + (key_set.contains(&(KeyCode::SPACE as i32)) as i32)
        + (key_set.contains(&(KeyCode::LEFTSHIFT as i32)) as i32)
        + (key_set.contains(&(KeyCode::RIGHTSHIFT as i32)) as i32)
        + (key_set.contains(&(KeyCode::BACKSPACE as i32)) as i32)
        + (key_set.contains(&(KeyCode::ENTER as i32)) as i32)
        + (key_set.contains(&(KeyCode::ESC as i32)) as i32)
        + (key_set.contains(&(KeyCode::PAUSE as i32)) as i32)
        ;
      
      let name = match &*working_name {
        None => "".to_string(),
        Some(name) => name.clone()
      };
      
      let has_scroll_down = key_set.contains(&(KeyCode::SCROLLDOWN as i32));
      let lacks_leds = !ev_set.contains(&0x11);
      let has_mouse_in_name = name.contains("Mouse");
      
      let mousey = (has_scroll_down as i32) + (lacks_leds as i32) + (has_mouse_in_name as i32) >= 2;
      
      let has_rel_motion = ev_set.contains(&0x2);
      
      // Heuristic for what is a keyboard
      if num_keys >= 20 && num_normal_keys >= 3 && !has_rel_motion && !mousey {
        match &*working_sysfs_path {
          None => (),
          Some(p) => {
            res.push(ExtractedProcBusKeyboard {
              sysfs_path: p.to_string(),
              name
            });
          }
        }
      }
    }
  }
  
  res
}

pub fn list_keyboards() -> io::Result<Vec<ExtractedKeyboard>> {
  let mut res = Vec::new();
  
  let proc_bus_input_devices = read_to_string("/proc/bus/input/devices")?;
  let extracted = extract_keyboards_from_proc_bus_input_devices(&proc_bus_input_devices);
  
  for dev in extracted {
    let p = dev.sysfs_path;
    if !p.starts_with("/devices/virtual") {
      match dev_path_for_sysfs_name(&p)? {
        None => (),
        Some(dev_path) => {
          res.push(ExtractedKeyboard {
            dev_path,
            name: dev.name
          });
        }
      }
    }
  }
  
  Ok(res)
}

fn dev_path_for_sysfs_name(sysfs_name: &String) -> io::Result<Option<PathBuf>> {
  let mut sysfs_path = "/sys".to_string();
  sysfs_path.push_str(sysfs_name);

  for _entry in Path::new(&sysfs_path).read_dir()? {
    let entry = _entry?;
    let path = entry.path();
    match path.file_name() {
      None => (),
      Some(_name) => {
        let name = _name.to_string_lossy();
        if name.starts_with("event") {
          let mut uevent_path = path.clone();
          uevent_path.push("uevent");
          for _line in io::BufReader::new(File::open(uevent_path)?).lines() {
            let line = _line?;
            if line.starts_with("DEVNAME=") {
              let dev_name = line[8..].to_string();
              let mut dev_path = PathBuf::new();
              dev_path.push("/dev");
              dev_path.push(dev_name);
              return Ok(Some(dev_path));
            }
          }
        }
      }
    }
  }
  
  Ok(None)
}

pub fn filter_keyboards_verbose<'a>(devices: &Vec<&'a str>) -> io::Result<Vec<&'a str>> {
  let mut res = Vec::new();
  
  let all_keyboards = list_keyboards()?;
  let mut canonical_set: HashSet<String> = HashSet::new();
  for p in all_keyboards {
    for q in canonicalize(p.dev_path) {
      for s in q.to_str() {
        canonical_set.insert(s.to_string());
      }
    }
  }
  
  for s in devices {
    match canonicalize(Path::new(s)) {
      Err(_) => {
        eprintln!("Skipping {} because could not canonicalize path", s);
      },
      Ok(c) => {
        match c.to_str() {
          None => {
            eprintln!("Skipping {} because could not c-strify path", s);
          },
          Some(l) => {
            if canonical_set.contains(&l.to_string()) {
              res.push(*s)
            }
            else {
              eprintln!("Skipping {} because {} is not in the list of keyboards", s, l);
            }
          }
        }
      }
    }
  }
  
  Ok(res)
}

#[cfg(test)]
mod tests {
  use crate::example_hardware;
  use super::*;

  #[test]
  fn test_parse_mask() {
    let res = parse_mask_hex("120013").unwrap();
    
    for item in &res {
      println!(" * {:x}", item)
    }
    
    assert_eq!(res.len(), 5);
    assert!(res.contains(&0x00));
    assert!(res.contains(&0x01));
    assert!(res.contains(&0x04));
    assert!(res.contains(&0x11));
    assert!(res.contains(&0x14));
  }
  
  #[test]
  fn test_parse_mask_2() {
    let res = parse_mask_hex("1 1").unwrap();
    
    for item in &res {
      println!(" * {:x}", item)
    }
    
    assert_eq!(res.len(), 2);
    assert!(res.contains(&0));
    assert!(res.contains(&64));
  }
  
  #[test]
  fn test_gaming_mouse_exclusion() {
    let text = example_hardware::GAMING_MOUSE_SETUP_1;
    
    let keyoards = extract_keyboards_from_proc_bus_input_devices(text);
    
    println!("Found:");
    for keyboard in &keyoards {
      println!(" * {}", keyboard.name);
    }
    println!("");
    
    let mut desired_name_set: HashSet<String> = HashSet::new();
    desired_name_set.insert("AT Translated Set 2 keyboard".to_string());
    
    let mut actual_name_set: HashSet<String> = HashSet::new();
    for keyboard in &keyoards {
      actual_name_set.insert(keyboard.name.clone());
    }
    
    for actual_name in &actual_name_set {
      if !desired_name_set.contains(actual_name) {
        panic!("Found spurious {}", actual_name);
      }
    }
    
    for desired_name in &desired_name_set {
      if !actual_name_set.contains(desired_name) {
        panic!("Failed to detect {}", desired_name);
      }
    }
  }
}

