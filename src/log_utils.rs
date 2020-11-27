
// vim: shiftwidth=2

use std::path::{Path, PathBuf};
use std::fs::{metadata, remove_file, File};
use std::io;
use std::time::{SystemTime, Duration};
use std::process;
use std::fs::OpenOptions;
use gag::{Redirect, Gag};

pub enum Logging {
  Redirected(Redirect<File>, Redirect<File>),
  Silent(Gag, Gag)
}

pub fn open_log_file_and_delete_stale() -> io::Result<Logging> {
  let log_root = &Path::new("/var/log/totalmapper");
  
  if !log_root.exists() {
    println!("WARNING: /var/log/totalmapper does not exist. Not loging.");
    Ok(Logging::Silent(Gag::stdout()?, Gag::stderr()?))
  }
  else {
    delete_stale(log_root)?;
    
    let pid = process::id();
    
    let out_log_filename = format!("pid-{}-out.log", pid);
    let mut out_log_path = PathBuf::from(log_root);
    out_log_path.push(out_log_filename);
    let out_log_file = OpenOptions::new()
      .truncate(true).read(false).create(true).write(true)
      .open(out_log_path)?;
    let out_redirect = Redirect::stdout(out_log_file)?;
    
    let err_log_filename = format!("pid-{}-err.log", pid);
    let mut err_log_path = PathBuf::from(log_root);
    err_log_path.push(err_log_filename);
    let err_log_file = OpenOptions::new()
      .truncate(true).read(false).create(true).write(true)
      .open(err_log_path)?;
    let err_redirect = Redirect::stderr(err_log_file)?;
    
    println!("Writing redirected log output.");
    
    Ok(Logging::Redirected(out_redirect, err_redirect))
  }
}

fn delete_stale<P: AsRef<Path>>(root: P) -> io::Result<()> {
  let num_files = root.as_ref().read_dir()?.count();
  if num_files > 10 {
    let num_to_delete = num_files - 10;
    
    struct PathAge {
      path: PathBuf,
      age: Duration
    }
    
    let now = SystemTime::now();
    let mut path_ages = Vec::new();
    
    for _entry in root.as_ref().read_dir()? {
      let entry = _entry?;
      let mtime = metadata(entry.path())?.modified()?;
      path_ages.push(PathAge {
        path: entry.path(),
        age: match now.duration_since(mtime) {
          Err(_) => Duration::from_secs(0),
          Ok(d) => d  
        }
      });
    }
    
    path_ages.sort_by_key(|p| p.age);
    
    for pa in path_ages.iter().rev().take(num_to_delete) {
      let path = &pa.path;
      // Swallow these errors since it could be due to the file being
      // open, or other things that don't matter.
      match remove_file(path) {
        Err(_) => (),
        Ok(_) => ()
      }
    }
  }
  
  Ok(())
}

