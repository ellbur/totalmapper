 
// vim: shiftwidth=2
 
#[macro_use]
extern crate enum_display_derive;

mod key_codes;
mod events;
mod keys;
mod fancy_keys;
mod fancy_layout_interpreting;
mod key_transforms;
mod dev_input_rw;
mod struct_ser;
mod default_fancy_layouts;
mod remapping_loop;
mod keyboard_listing;
mod udev_utils;
mod layout_loading;
mod version;
mod monitor;
mod monitor_raw;
mod struct_de;
mod tablet_mode_switch_reader;
mod monitor_tablet_mode;
mod example_hardware;
mod layout_parsing_formatting;
mod char_production_map;
mod physical_keyboard_layouts;
mod complete_tests;

use clap::{Arg, App};
use keys::Layout;

fn main() {
  let mut app =
    App::new("totalmapper")
      .version(version::VERSION)
      .author("Owen Healy <owen@owenehealy.com>")
      .about("Remaps keycodes in the Linux input event system based on a simple, JSON-defined list of mappings.\n\
        \n\
        To try mapping your keyboard, run:\n\
        \n\
            totalmapper remap --default-layout caps-for-movement --all-keyboards\n\
        \n\
        (making sure you have write access to /dev/uinput).\n\
        \n\
        To see an example of how to define mappings, run:\n\
        \n\
            totalmapper print_default_layout caps-for-movement\n\
        \n\
        More documentation is available at https://github.com/ellbur/totalmapper")
      .subcommand(App::new("remap")
        .about("Remap a keyboard")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .takes_value(true)
          .value_name("FILE")
          .multiple_occurrences(true)
          .number_of_values(1)
          .help_heading(Some("DEVICE SELECTION"))
          .help("A path under /dev/input representing a keyboard device. To find your keyboards, run `totalmapper list_keyboards`. Repeat this option multiple times to map multiple keyboards, e.g., `totalmapper remap --dev-file /dev/input/event0 --dev-file /dev/input/event1`. Use --all-keyboards to map all keyboards currently plugged in.")
        )
        .arg(Arg::new("all_keyboards")
          .long("all-keyboards")
          .help_heading(Some("DEVICE SELECTION"))
          .help("Remap all keyboards currently plugged in. Note that this will not affect keyboards you plug in after invoking this command. To automatically remap new keyboards, see --auto-all-keyboards or the command `totalmapper add_udev_rule`.")
        )
        .arg(Arg::new("auto_all_keyboards")
          .long("auto-all-keyboards")
          .help_heading(Some("DEVICE SELECTION"))
          .help("Automatically remap keyboards as they are plugged in. Useful on systems that don't use systemd.")
        )
        .arg(Arg::new("default_layout")
          .long("default-layout")
          .takes_value(true)
          .value_name("NAME")
          .help_heading(Some("LAYOUT SELECTION"))
          .help("Use the builtin layout named NAME. To list the builtin layouts, use `totalmapper list_default_layouts`. To get the JSON for a default layout, use `totalmapper print_default_layout <name>`.")
        )
        .arg(Arg::new("layout_file")
          .long("layout-file")
          .takes_value(true)
          .value_name("FILE")
          .help_heading(Some("LAYOUT SELECTION"))
          .help("Load a layout from json file FILE. To see an example of the form, print an example using `totalmapper print_default_layout caps-for-movement`.")
        )
        .arg(Arg::new("only_if_keyboard")
          .long("only-if-keyboard")
          .help_heading(Some("PROCESS"))
          .help("If the device selected with --dev-file is not a keyboard, exit successfully. Useful when running from udev, since there is no easy way to test in a udev rule whether an input device is a keyboard.")
        )
        .arg(Arg::new("exclude")
          .long("exclude")
          .takes_value(true)
          .value_name("PATTERN")
          .multiple_occurrences(true)
          .help_heading(Some("DEVICE SELECTION"))
          .help("Don't apply to keyboards with names matching glob-style pattern. To see the names of currently connected keyboards, run `totalmapper list_keyboards`; the part before the ':' is the name. Repeat this option to exclude multiple patterns. Useful when running from udev.")
        )
        .arg(Arg::new("tablet_mode_switch_device")
          .long("tablet-mode-switch-device")
          .takes_value(true)
          .value_name("FILE")
          .help_heading(Some("TABLET MODE"))
          .help("Do not emit key events when the selected device indicates the computer is in tablet mode.")
        )
        .arg(Arg::new("verbose")
          .long("verbose")
          .help_heading(Some("DEBUGGING"))
          .help("Print verbose info.")
        )
      )
      .subcommand(App::new("list_keyboards")
        .about("List keyboard devices under /dev/input")
        .arg(Arg::new("verbose")
          .long("verbose")
        )
      )
      .subcommand(App::new("list_default_layouts")
        .about("List the names of the default layouts")
      )
      .subcommand(App::new("print_default_layout")
        .about("Print the JSON for one of the builtin layouts")
        .arg(Arg::new("NAME")
          .required(true)
          .index(1)
          .help("The name of the builtin layout to print. Use `totalmapper list_default_layouts` to see the list of builtin layouts.")
        )
      )
      .subcommand(App::new("monitor")
        .about("Print events from a keyboard device (without consuming them)")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .takes_value(true)
          .value_name("FILE")
          .number_of_values(1)
          .help("A path under /dev/input representing a keyboard device. To find your keyboards, run `totalmapper list_keyboards`.")
        )
      )
      .subcommand(App::new("monitor_raw")
        .about("Print all events from any input device (without consuming them).")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .takes_value(true)
          .value_name("FILE")
          .number_of_values(1)
          .help("A path under /dev/input")
        )
      )
      .subcommand(App::new("monitor_tablet_mode")
        .about("Monitor a table mode switch device.")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .takes_value(true)
          .value_name("FILE")
          .number_of_values(1)
          .help("A path under /dev/input representing your tablet mode switch")
        )
      )
      .subcommand(App::new("add_systemd_service")
        .about("Add (or update, if one exists) a rule in /etc/udev/rules.d/ and service in /etc/systemd/system/ to start totalmapper when a new keyboard is plugged in. Add --and-start option to also start it for keyboards already plugged in. Must be run as root.")
        .arg(Arg::new("default_layout")
          .long("default-layout")
          .takes_value(true)
          .value_name("NAME")
          .help_heading(Some("LAYOUT SELECTION"))
          .help("Use the builtin layout named NAME. To list the builtin layouts, use `totalmapper list_default_layouts`. To get the JSON for a default layout, use `totalmapper print_default_layout <name>`.")
        )
        .arg(Arg::new("layout_file")
          .long("layout-file")
          .takes_value(true)
          .value_name("FILE")
          .help_heading(Some("LAYOUT SELECTION"))
          .help("Load a layout from json file FILE. To see an example of the form, print an example using `totalmapper print_default_layout caps-for-movement`.")
        )
        .arg(Arg::new("and_start")
          .long("and-start")
          .help_heading(Some("RUNNING"))
          .help("Also start the service for all existing keyboards")
        )
        .arg(Arg::new("exclude")
          .long("exclude")
          .takes_value(true)
          .value_name("PATTERN")
          .multiple_occurrences(true)
          .help_heading(Some("DEVICE SELECTION"))
          .help("Don't apply to keyboards with names matching glob-style pattern. To see the names of currently connected keyboards, run `totalmapper list_keyboards`; the part before the ':' is the name. Repeat this option to exclude multiple patterns.")
        )
      );
      
  let m = app.clone().get_matches();
  
  if let Some(m) = m.subcommand_matches("remap") {
    let layout = load_layout(&m.value_of("default_layout"), &m.value_of("layout_file"));
    match layout {
      Err(msg) => {
        println!("{}", msg);
        std::process::exit(1);
      },
      Ok(layout) => {
        match (m.occurrences_of("all_keyboards") > 0, m.values_of("dev_file"), m.occurrences_of("auto_all_keyboards") > 0) {
          (false, None, false) => {
            println!("Error: Must specify a least one --dev-file or --all-keyboards");
          },
          (true, Some(_), _) => {
            println!("Error: Must specify either --dev-file, --all-keyboards, or --auto-all-keyboards, not both");
          },
          (true, _, true) => {
            println!("Error: Must specify either --dev-file, --all-keyboards, or --auto-all-keyboards, not both");
          },
          (_, Some(_), true) => {
            println!("Error: Must specify either --dev-file, --all-keyboards, or --auto-all-keyboards, not both");
          },
          (true, _, _) => {
            let excludes: Vec<&str> = match m.values_of("exclude") {
              None => vec![],
              Some(excludes) => excludes.collect()
            };
            match remapping_loop::do_remapping_loop_all_devices(&layout, &excludes, m.occurrences_of("verbose") > 0) {
              Ok(_) => (),
              Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
              }
            }
          },
          (_, Some(devs), _) => {
            let devs2 = devs.collect();
            let excludes: Vec<&str> = match m.values_of("exclude") {
              None => vec![],
              Some(excludes) => excludes.collect()
            };
            match remapping_loop::do_remapping_loop_multiple_devices(
                &devs2,
                m.occurrences_of("only_if_keyboard") > 0,
                &excludes,
                &layout,
                &m.value_of("tablet_mode_switch_device"),
                m.occurrences_of("verbose") > 0)
            {
              Ok(_) => (),
              Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
              }
            }
          },
          (_, _, true) => {
            let excludes: Vec<&str> = match m.values_of("exclude") {
              None => vec![],
              Some(excludes) => excludes.collect()
            };
            match remapping_loop::do_remapping_loop_auto_all_devices(&layout, &excludes, m.occurrences_of("verbose") > 0) {
              Ok(_) => (),
              Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
              }
            }
          }
        }
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("list_keyboards") {
    keyboard_listing::list_keyboards_to_stdout(m.occurrences_of("verbose") > 0).unwrap();
  }
  else if let Some(_) = m.subcommand_matches("list_default_layouts") {
    for name in (*default_fancy_layouts::DEFAULT_LAYOUTS).keys() {
      println!("{}", name);
    }
  }
  else if let Some(m) = m.subcommand_matches("print_default_layout") {
    let name = m.value_of("NAME").unwrap();
    match (*default_fancy_layouts::DEFAULT_LAYOUTS).get(name) {
      None => {
        println!("Error: no builtin layout named {}", name);
        std::process::exit(1);
      },
      Some(layout) => {
        println!("{}", layout)
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("monitor") {
    match m.value_of("dev_file") {
      None => {
        println!("Must specify --dev-file");
      },
      Some(dev_file) => {
        monitor::run_monitor(dev_file);
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("monitor_raw") {
    match m.value_of("dev_file") {
      None => {
        println!("Must specify --dev-file");
      },
      Some(dev_file) => {
        monitor_raw::run_monitor_raw(dev_file);
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("monitor_tablet_mode") {
    match m.value_of("dev_file") {
      None => {
        println!("Must specify --dev-file");
      },
      Some(dev_file) => {
        monitor_tablet_mode::run_monitor(dev_file);
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("add_systemd_service") {
    match load_layout(&m.value_of("default_layout"), &m.value_of("layout_file")) {
      Err(s) => {
        println!("{}", s);
        std::process::exit(1);
      },
      Ok(layout) => {
        let excludes: Vec<&str> = match m.values_of("exclude") {
          None => vec![],
          Some(excludes) => excludes.collect()
        };
        
        match udev_utils::add_systemd_service(&layout, excludes.into_iter()) {
          Err(msg) => {
            println!("{}", msg);
            std::process::exit(1);
          },
          Ok(_) => {
            if m.occurrences_of("and_start") > 0 {
              match udev_utils::start_systemd_service() {
                Err(msg) => {
                  println!("{}", msg);
                  std::process::exit(1);
                },
                Ok(_) => ()
              };
            }
          }
        }
      }
    }
  }
  else {
    app.print_long_help().unwrap();
  }
}

fn load_layout(default_layout: &Option<&str>, layout_file: &Option<&str>) -> Result<Layout, String> {
  match (default_layout, layout_file) {
    (None, None) => {
      Err("Error: no layout specified. Use --default-layout or --layout-file.".to_string())
    },
    (Some(_), Some(_)) => {
      Err("Error: use either --default-layout or --layout-file, not both.".to_string())
    },
    (Some(name), None) => {
      match (*default_fancy_layouts::DEFAULT_LAYOUTS).get(&name.to_string()) {
        None => Err(format!("Error: no builtin layout named {}", name)),
        Some(layout) => Ok(
          fancy_layout_interpreting::convert(
            &layout_parsing_formatting::parse_layout_from_json(
              &serde_json::from_str(layout).unwrap()
            ).unwrap()
          ).unwrap()
        )
      }
    },
    (None, Some(path)) => {
      match layout_loading::load_layout_from_file(path) {
        Err(err) => Err(err),
        Ok(layout) => Ok(layout)
      }
    }
  }
}

