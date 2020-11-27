 
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
mod log_utils;
mod udev_utils;
mod layout_loading;

use clap::{Arg, App};
use std::borrow::Cow;

fn main() {
  let mut app =
    App::new("The TotalMapper keyboard remapping tool")
      .version("1.0")
      .author("Owen Healy <owen@owenehealy.com>")
      .about("Remaps keycodes in the Linux input event system based on a JSON-defined list of mappings")
      .subcommand(App::new("remap")
        .about("Remap a keyboard")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .takes_value(true)
          .value_name("FILE")
          .multiple(true)
          .number_of_values(1)
          .help_heading(Some("DEVICE SELECTION"))
          .about("A path under /dev/input representing a keyboard device. To find your keyboards, run `totalmapper list_keyboards`. Repeat this option multiple times to map multiple keyboards, e.g., `totalmapper remap --dev-file /dev/input/event0 --dev-file /dev/input/event1`. Use --all-keyboards to map all keyboards currently plugged in.")
        )
        .arg(Arg::new("all_keyboards")
          .long("all-keyboards")
          .help_heading(Some("DEVICE SELECTION"))
          .about("Remap all keyboards currently plugged in. Note that this will not affect keyboards you plug in after invoking this command. To automatically remap new keyboards, see the help for `totalmapper add_udev_rule`.")
        )
        .arg(Arg::new("default_layout")
          .long("default-layout")
          .takes_value(true)
          .value_name("NAME")
          .help_heading(Some("LAYOUT SELECTION"))
          .about("Use the builtin layout named NAME. To list the builtin layouts, use `totalmapper list_default_layouts`. To get the JSON for a default layout, use `totalmapper print_default_layout <name>`.")
        )
        .arg(Arg::new("layout_file")
          .long("layout-file")
          .takes_value(true)
          .value_name("FILE")
          .help_heading(Some("LAYOUT SELECTION"))
          .about("Load a layout from json file FILE. To see an example of the form, print an example using `totalmapper print_default_layout caps-for-movement`.")
        )
        .arg(Arg::new("fork")
          .long("fork")
          .help_heading(Some("PROCESS"))
          .about("Fork a detached child process and immediately return control. This is needed when invoked from a udev rule since commands invoked from a udev rule must exit quickly. Implies --log.")
        )
        .arg(Arg::new("only_if_keyboard")
          .long("only-if-keyboard")
          .help_heading(Some("PROCESS"))
          .about("If the device selected with --dev-file is not a keyboard, exit successfully. Useful when running from udev, since there is no easy way to test in a udev rule whether an input device is a keyboard.")
        )
        .arg(Arg::new("log_to_file")
          .long("log")
          .help_heading(Some("PROCESS"))
          .about("Send messages to /var/log/totalmapper/ instead of stdout and stderr. The directory must exist and be writable. If logging to the directory fails, there will be no logging.")
        )
      )
      .subcommand(App::new("list_keyboards")
        .about("List keyboard devices under /dev/input")
      )
      .subcommand(App::new("list_default_layouts")
        .about("List the names of the default layouts")
      )
      .subcommand(App::new("print_default_layout")
        .about("Print the JSON for one of the builtin layouts")
        .arg(Arg::new("NAME")
          .required(true)
          .index(1)
          .about("The name of the builtin layout to print. Use `totalmapper list_default_layouts` to see the list of builtin layouts.")
        )
      )
      .subcommand(App::new("add_udev_rule")
        .about("Add (or update, if one exists) a rule in /etc/udev/rules.d/ to start totalmapper when a new keyboard is plugged in. Does not affect keyboards already plugged in. Must be run as root.")
        .arg(Arg::new("default_layout")
          .long("default-layout")
          .takes_value(true)
          .value_name("NAME")
          .help_heading(Some("LAYOUT SELECTION"))
          .about("Use the builtin layout named NAME. To list the builtin layouts, use `totalmapper list_default_layouts`. To get the JSON for a default layout, use `totalmapper print_default_layout <name>`.")
        )
        .arg(Arg::new("layout_file")
          .long("layout-file")
          .takes_value(true)
          .value_name("FILE")
          .help_heading(Some("LAYOUT SELECTION"))
          .about("Load a layout from json file FILE. To see an example of the form, print an example using `totalmapper print_default_layout caps-for-movement`.")
        )
      )
      .subcommand(App::new("print_udev_rule")
        .about("Print the udev rule that would be added to /etc/udev/rules.d to start totalmapper when a new keyboard is plugged in.")
        .arg(Arg::new("default_layout")
          .long("default-layout")
          .takes_value(true)
          .value_name("NAME")
          .help_heading(Some("LAYOUT SELECTION"))
          .about("Use the builtin layout named NAME. To list the builtin layouts, use `totalmapper list_default_layouts`. To get the JSON for a default layout, use `totalmapper print_default_layout <name>`.")
        )
        .arg(Arg::new("layout_file")
          .long("layout-file")
          .takes_value(true)
          .value_name("FILE")
          .help_heading(Some("LAYOUT SELECTION"))
          .about("Load a layout from json file FILE. To see an example of the form, print an example using `totalmapper print_default_layout caps-for-movement`.")
        )
      );
      
  let m = app.clone().get_matches();
  
  if let Some(m) = m.subcommand_matches("remap") {
    let fork = m.occurrences_of("fork") > 0;
    let ignore_sig_hup = fork;
    let log = fork || m.occurrences_of("log_to_file") > 0;
    
    fork_utils::fork_if_needed(fork, || {
      if ignore_sig_hup {
        unsafe {
          libc::signal(libc::SIGHUP, libc::SIG_IGN);
        }
      }
      let _log_redirection = {
        if log {
          Some(log_utils::open_log_file_and_delete_stale().unwrap())
        }
        else {
          None
        }
      };
      
      println!("Starting remapping.");
      
      let layout =
        match (m.value_of("default_layout"), m.value_of("layout_file")) {
          (None, None) => {
            Err("Error: no layout specified. Use --default-layout or --layout-file.".to_string())
          },
          (Some(_), Some(_)) => {
            Err("Error: use either --default-layout or --layout-file, not both.".to_string())
          },
          (Some(name), None) => {
            match (*default_layouts::DEFAULT_LAYOUTS).get(name) {
              None => Err(format!("Error: no builtin layout named {}", name)),
              Some(layout) => Ok(Cow::Borrowed(*layout))
            }
          },
          (None, Some(path)) => {
            match layout_loading::load_layout_from_file(path) {
              Err(err) => Err(err),
              Ok(layout) => Ok(Cow::Owned(layout))
            }
          }
        };
      
      match layout {
        Err(msg) => println!("{}", msg),
        Ok(layout) => {
          match (m.occurrences_of("all_keyboards") > 0, m.values_of("dev_file")) {
            (false, None) => {
              println!("Error: Must specify a least one --dev-file or --all-keyboards");
            },
            (true, Some(_)) => {
              println!("Error: Must specify either --dev-file or --all-keyboards, not both");
            },
            (true, None) => {
              match remapping_loop::do_remapping_loop_all_devices(&layout) {
                Ok(_) => (),
                Err(err) => {
                  println!("Error: {}", err);
                }
              }
            },
            (false, Some(devs)) => {
              let devs2 = devs.collect();
              match remapping_loop::do_remapping_loop_multiple_devices(&devs2, m.occurrences_of("only_if_keyboard") > 0, &layout) {
                Ok(_) => (),
                Err(err) => {
                  println!("Error: {}", err);
                }
              }
            }
          }
        }
      }
    }).unwrap();
  }
  else if let Some(_) = m.subcommand_matches("list_keyboards") {
    keyboard_listing::list_keyboards_to_stdout().unwrap();
  }
  else if let Some(_) = m.subcommand_matches("list_default_layouts") {
    for name in (*default_layouts::DEFAULT_LAYOUTS).keys() {
      println!("{}", name);
    }
  }
  else if let Some(m) = m.subcommand_matches("print_default_layout") {
    let name = m.value_of("NAME").unwrap();
    match (*default_layouts::DEFAULT_LAYOUTS).get(name) {
      None => {
        println!("Error: no builtin layout named {}", name);
      },
      Some(layout) => {
        println!("{}", serde_json::to_string_pretty(layout).unwrap())
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("add_udev_rule") {
    match udev_utils::add_udev_rule(m.value_of("default_layout"), m.value_of("layout_file")) {
      Err(msg) => {
        println!("{}", msg);
      },
      Ok(_) => ()
    }
  }
  else if let Some(m) = m.subcommand_matches("print_udev_rule") {
    match udev_utils::udev_rule(m.value_of("default_layout"), m.value_of("layout_file")) {
      Err(msg) => {
        println!("{}", msg);
      },
      Ok(rule) => {
        println!("{}", rule);
      }
    }
  }
  else {
    app.print_long_help().unwrap();
  }
}

