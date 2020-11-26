 
// vim: shiftwidth=2
 
mod key_codes;
mod keys;
mod key_transforms;
mod dev_input_rw;
mod struct_ser;
mod default_layouts;
mod remapping_loop;
mod layout_generation;

fn main() {
  let path = "/dev/input/by-path/pci-0000:00:14.0-usb-0:1:1.0-event-kbd";
  let layout = &*default_layouts::SUPER_DVORAK;
  remapping_loop::do_remapping_loop(path.to_string(), layout).unwrap();
}

