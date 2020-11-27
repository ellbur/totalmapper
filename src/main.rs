 
// vim: shiftwidth=2
 
mod key_codes;
mod keys;
mod key_transforms;
mod dev_input_rw;
mod struct_ser;
mod default_layouts;
mod remapping_loop;
mod layout_generation;
mod keyboard_listing;
mod fork_utils;

use clap::{Arg, App, SubCommand};

fn main() {
  let mut app =
    App::new("The TotalMapper keyboard remapping tool")
      .version("1.0")
      .author("Owen Healy <owen@owenehealy.com>")
      .about("Remaps keycodes in the Linux input event system based on a JSON-defined list of mappings")
      .subcommand(SubCommand::with_name("remap")
        .about("Remap a keyboard")
        .arg(Arg::with_name("dev_file")
          .long("dev-file")
          .takes_value(true)
          .value_name("FILE")
          .multiple(true)
          .number_of_values(1)
          .help("A path under /dev/input representing a keyboard device. To find your keyboards, run `totalmapper list_keyboards`. Repeat this option multiple times to map multiple keyboards, e.g., `totalmapper remap --dev-file /dev/input/event0 --dev-file /dev/input/event1`. Use --all-keyboards to map all keyboards currently plugged in.")
        )
        .arg(Arg::with_name("all_keyboards")
          .long("all-keyboards")
          .help("Remap all keyboards currently plugged in. Note that this will not affect keyboards you plug in after invoking this command. To automatically remap new keyboards, see the help for `totalmapper add_udev_rule`.")
        )
        .arg(Arg::with_name("fork")
          .long("fork")
          .help("Fork a detached child process and immediately return control. This is needed when invoked from a udev rule since commands invoked from a udev rule must exit quickly.")
        )
      )
      .subcommand(SubCommand::with_name("list_keyboards")
        .about("List keyboard devices under /dev/input")
      );
  
  let m = app.clone().get_matches();
  
  if let Some(m) = m.subcommand_matches("remap") {
    let fork = m.occurrences_of("fork") > 0;
    fork_utils::fork_if_needed(fork, || {
      unsafe {
        libc::signal(libc::SIGHUP, libc::SIG_IGN);
      }
      for i in 0 .. 100 {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("From child {}", i);
      }
    }).unwrap();
  }
  else if let Some(_) = m.subcommand_matches("list_keyboards") {
    keyboard_listing::list_keyboards_to_stdout().unwrap();
  }
  else {
    app.print_long_help().unwrap();
  }
}

