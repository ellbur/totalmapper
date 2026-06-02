
// vim: shiftwidth=2

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

use clap::{Arg, ArgAction, Command};
use keys::Layout;

fn main() {
  let mut app =
    Command::new("totalmapper")
      .version(env!("CARGO_PKG_VERSION"))
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
      .subcommand(Command::new("remap")
        .about("Remap a keyboard")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .value_name("FILE")
          .action(ArgAction::Append)
          .help_heading("DEVICE SELECTION")
          .help("A path under /dev/input representing a keyboard device. To find your keyboards, run `totalmapper list_keyboards`. Repeat this option multiple times to map multiple keyboards, e.g., `totalmapper remap --dev-file /dev/input/event0 --dev-file /dev/input/event1`. Use --all-keyboards to map all keyboards currently plugged in.")
        )
        .arg(Arg::new("all_keyboards")
          .long("all-keyboards")
          .action(ArgAction::SetTrue)
          .help_heading("DEVICE SELECTION")
          .help("Remap all keyboards currently plugged in. Note that this will not affect keyboards you plug in after invoking this command. To automatically remap new keyboards, see --auto-all-keyboards or the command `totalmapper add_udev_rule`.")
        )
        .arg(Arg::new("auto_all_keyboards")
          .long("auto-all-keyboards")
          .action(ArgAction::SetTrue)
          .help_heading("DEVICE SELECTION")
          .help("Automatically remap keyboards as they are plugged in. Useful on systems that don't use systemd.")
        )
        .arg(Arg::new("default_layout")
          .long("default-layout")
          .value_name("NAME")
          .help_heading("LAYOUT SELECTION")
          .help("Use the builtin layout named NAME. To list the builtin layouts, use `totalmapper list_default_layouts`. To get the JSON for a default layout, use `totalmapper print_default_layout <name>`.")
        )
        .arg(Arg::new("layout_file")
          .long("layout-file")
          .value_name("FILE")
          .help_heading("LAYOUT SELECTION")
          .help("Load a layout from json file FILE. To see an example of the form, print an example using `totalmapper print_default_layout caps-for-movement`.")
        )
        .arg(Arg::new("only_if_keyboard")
          .long("only-if-keyboard")
          .action(ArgAction::SetTrue)
          .help_heading("PROCESS")
          .help("If the device selected with --dev-file is not a keyboard, exit successfully. Useful when running from udev, since there is no easy way to test in a udev rule whether an input device is a keyboard.")
        )
        .arg(Arg::new("exclude")
          .long("exclude")
          .value_name("PATTERN")
          .action(ArgAction::Append)
          .help_heading("DEVICE SELECTION")
          .help("Don't apply to keyboards with names matching glob-style pattern. To see the names of currently connected keyboards, run `totalmapper list_keyboards`; the part before the ':' is the name. Repeat this option to exclude multiple patterns. Useful when running from udev.")
        )
        .arg(Arg::new("tablet_mode_switch_device")
          .long("tablet-mode-switch-device")
          .value_name("FILE")
          .help_heading("TABLET MODE")
          .help("Do not emit key events when the selected device indicates the computer is in tablet mode.")
        )
        .arg(Arg::new("verbose")
          .long("verbose")
          .action(ArgAction::SetTrue)
          .help_heading("DEBUGGING")
          .help("Print verbose info.")
        )
      )
      .subcommand(Command::new("list_keyboards")
        .about("List keyboard devices under /dev/input")
        .arg(Arg::new("verbose")
          .long("verbose")
          .action(ArgAction::SetTrue)
        )
      )
      .subcommand(Command::new("list_default_layouts")
        .about("List the names of the default layouts")
      )
      .subcommand(Command::new("print_default_layout")
        .about("Print the JSON for one of the builtin layouts")
        .arg(Arg::new("NAME")
          .required(true)
          .index(1)
          .help("The name of the builtin layout to print. Use `totalmapper list_default_layouts` to see the list of builtin layouts.")
        )
        .arg(Arg::new("plain")
          .long("plain")
          .action(ArgAction::SetTrue)
          .help("Convert the layout to the plain format before printing. The plain format expands aliases and row mappings into individual key mappings.")
        )
      )
      .subcommand(Command::new("monitor")
        .about("Print events from a keyboard device (without consuming them)")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .value_name("FILE")
          .help("A path under /dev/input representing a keyboard device. To find your keyboards, run `totalmapper list_keyboards`.")
        )
      )
      .subcommand(Command::new("monitor_raw")
        .about("Print all events from any input device (without consuming them).")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .value_name("FILE")
          .help("A path under /dev/input")
        )
      )
      .subcommand(Command::new("monitor_tablet_mode")
        .about("Monitor a table mode switch device.")
        .arg(Arg::new("dev_file")
          .long("dev-file")
          .value_name("FILE")
          .help("A path under /dev/input representing your tablet mode switch")
        )
      )
      .subcommand(Command::new("add_systemd_service")
        .about("Add (or update, if one exists) a rule in /etc/udev/rules.d/ and service in /etc/systemd/system/ to start totalmapper when a new keyboard is plugged in. Add --and-start option to also start it for keyboards already plugged in. Must be run as root.")
        .arg(Arg::new("default_layout")
          .long("default-layout")
          .value_name("NAME")
          .help_heading("LAYOUT SELECTION")
          .help("Use the builtin layout named NAME. To list the builtin layouts, use `totalmapper list_default_layouts`. To get the JSON for a default layout, use `totalmapper print_default_layout <name>`.")
        )
        .arg(Arg::new("layout_file")
          .long("layout-file")
          .value_name("FILE")
          .help_heading("LAYOUT SELECTION")
          .help("Load a layout from json file FILE. To see an example of the form, print an example using `totalmapper print_default_layout caps-for-movement`.")
        )
        .arg(Arg::new("and_start")
          .long("and-start")
          .action(ArgAction::SetTrue)
          .help_heading("RUNNING")
          .help("Also start the service for all existing keyboards")
        )
        .arg(Arg::new("exclude")
          .long("exclude")
          .value_name("PATTERN")
          .action(ArgAction::Append)
          .help_heading("DEVICE SELECTION")
          .help("Don't apply to keyboards with names matching glob-style pattern. To see the names of currently connected keyboards, run `totalmapper list_keyboards`; the part before the ':' is the name. Repeat this option to exclude multiple patterns.")
        )
      );

  let m = app.clone().get_matches();

  if let Some(m) = m.subcommand_matches("remap") {
    let default_layout = m.get_one::<String>("default_layout").map(|s| s.as_str());
    let layout_file = m.get_one::<String>("layout_file").map(|s| s.as_str());
    let layout = load_layout(&default_layout, &layout_file);
    match layout {
      Err(msg) => {
        println!("{}", msg);
        std::process::exit(1);
      },
      Ok(layout) => {
        let dev_files: Option<Vec<&str>> = m.get_many::<String>("dev_file")
          .map(|vals| vals.map(|s| s.as_str()).collect());
        let all_keyboards = m.get_flag("all_keyboards");
        let auto_all_keyboards = m.get_flag("auto_all_keyboards");
        let verbose = m.get_flag("verbose");
        let only_if_keyboard = m.get_flag("only_if_keyboard");
        let excludes: Vec<&str> = m.get_many::<String>("exclude")
          .map(|vals| vals.map(|s| s.as_str()).collect())
          .unwrap_or_default();
        let tablet_mode_switch_device = m.get_one::<String>("tablet_mode_switch_device").map(|s| s.as_str());

        match (all_keyboards, dev_files.as_ref(), auto_all_keyboards) {
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
            match remapping_loop::do_remapping_loop_all_devices(&layout, &excludes, verbose) {
              Ok(_) => (),
              Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
              }
            }
          },
          (_, Some(devs), _) => {
            match remapping_loop::do_remapping_loop_multiple_devices(
                devs,
                only_if_keyboard,
                &excludes,
                &layout,
                &tablet_mode_switch_device,
                verbose)
            {
              Ok(_) => (),
              Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
              }
            }
          },
          (_, _, true) => {
            match remapping_loop::do_remapping_loop_auto_all_devices(&layout, &excludes, verbose) {
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
    keyboard_listing::list_keyboards_to_stdout(m.get_flag("verbose")).unwrap();
  }
  else if let Some(_) = m.subcommand_matches("list_default_layouts") {
    for name in (*default_fancy_layouts::DEFAULT_LAYOUTS).keys() {
      println!("{}", name);
    }
  }
  else if let Some(m) = m.subcommand_matches("print_default_layout") {
    let name = m.get_one::<String>("NAME").unwrap();
    match (*default_fancy_layouts::DEFAULT_LAYOUTS).get(name) {
      None => {
        println!("Error: no builtin layout named {}", name);
        std::process::exit(1);
      },
      Some(layout) => {
        if m.get_flag("plain") {
          let json_value: serde_json::Value = serde_json::from_str(layout).unwrap();
          let fancy_layout = layout_parsing_formatting::parse_layout_from_json(&json_value).unwrap();
          let plain_layout = fancy_layout_interpreting::convert(&fancy_layout).unwrap();
          let plain_json = serde_json::to_string_pretty(&plain_layout).unwrap();
          println!("{}", plain_json);
        } else {
          println!("{}", layout)
        }
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("monitor") {
    match m.get_one::<String>("dev_file") {
      None => {
        println!("Must specify --dev-file");
      },
      Some(dev_file) => {
        monitor::run_monitor(dev_file);
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("monitor_raw") {
    match m.get_one::<String>("dev_file") {
      None => {
        println!("Must specify --dev-file");
      },
      Some(dev_file) => {
        monitor_raw::run_monitor_raw(dev_file);
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("monitor_tablet_mode") {
    match m.get_one::<String>("dev_file") {
      None => {
        println!("Must specify --dev-file");
      },
      Some(dev_file) => {
        monitor_tablet_mode::run_monitor(dev_file);
      }
    }
  }
  else if let Some(m) = m.subcommand_matches("add_systemd_service") {
    let default_layout = m.get_one::<String>("default_layout").map(|s| s.as_str());
    let layout_file = m.get_one::<String>("layout_file").map(|s| s.as_str());
    match load_layout(&default_layout, &layout_file) {
      Err(s) => {
        println!("{}", s);
        std::process::exit(1);
      },
      Ok(layout) => {
        let excludes: Vec<&str> = m.get_many::<String>("exclude")
          .map(|vals| vals.map(|s| s.as_str()).collect())
          .unwrap_or_default();

        match udev_utils::add_systemd_service(&layout, excludes.into_iter()) {
          Err(msg) => {
            println!("{}", msg);
            std::process::exit(1);
          },
          Ok(_) => {
            if m.get_flag("and_start") {
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

