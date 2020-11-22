
extern crate nix;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::close;

fn openclose() {
    println!("Hello, world!");
    
    let uinput_fd = open("/dev/null", OFlag::O_WRONLY | OFlag::O_NONBLOCK, Mode::empty()).unwrap();
    
    close(uinput_fd).unwrap();
}

