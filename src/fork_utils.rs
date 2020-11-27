
// vim: shiftwidth=2

use nix::unistd::ForkResult;

pub fn fork_if_needed<CC: FnOnce()>(fork: bool, cc: CC) -> nix::Result<()> {
  if fork {
    match unsafe { nix::unistd::fork() }? {
      ForkResult::Parent { .. } => Ok(()),
      ForkResult::Child => Ok(cc()),
    }
  }
  else {
    Ok(cc())
  }
}

