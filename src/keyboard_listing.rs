
// vim: shiftwidth=2

use std::fs::{File, canonicalize};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

pub fn list_keyboards_to_stdout() -> io::Result<()> {
  for p in list_keyboards()? {
    println!("{}", p.to_string_lossy());
  }
  
  Ok(())
}

pub fn list_keyboards() -> io::Result<Vec<PathBuf>> {
  let mut res = Vec::new();
  
  let file = File::open("/proc/bus/input/devices")?;
  let lines = io::BufReader::new(file).lines();
  
  let mut working_sysfs_path = Box::new(Some("".to_string()));
  
  for _line in lines {
    let line = _line?;
    if line.starts_with("S: Sysfs=") {
      let new_sysfs_path = line[9..].to_string();
      *working_sysfs_path = Some(new_sysfs_path);
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
      // Treat any input device with at this many keys as a keyboard.
      // This excludes devices like the power switch or the lid switch that have a few keys, but
      // would not be considered keyboards by most people.
      if num_keys >= 10 {
        match &*working_sysfs_path {
          None => (),
          Some(p) => {
            if !p.starts_with("/devices/virtual") {
              match dev_path_for_sysfs_name(p)? {
                None => (),
                Some(dev_path) => {
                  res.push(dev_path);
                }
              }
            }
          }
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

pub fn filter_keyboards<'a>(devices: &Vec<&'a str>) -> io::Result<Vec<&'a str>> {
  let mut res = Vec::new();
  
  let all_keyboards = list_keyboards()?;
  let mut canonical_set: HashSet<String> = HashSet::new();
  for p in all_keyboards {
    for q in canonicalize(p) {
      for s in q.to_str() {
        canonical_set.insert(s.to_string());
      }
    }
  }
  
  for s in devices {
    match canonicalize(Path::new(s)) {
      Err(_) => (),
      Ok(c) => {
        match c.to_str() {
          None => (),
          Some(l) => {
            if canonical_set.contains(&l.to_string()) {
              res.push(*s)
            }
          }
        }
      }
    }
  }
  
  Ok(res)
}

