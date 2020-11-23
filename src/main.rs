 
// vim: shiftwidth=2
 
mod key_codes;
mod keys;
mod key_transforms;
mod dev_input_rw;
mod struct_ser;

fn main() {
  let mut r = dev_input_rw::DevInputReader::open("/dev/input/by-path/pci-0000:00:14.0-usb-0:1:1.0-event-kbd", true).unwrap();
  let mut w = dev_input_rw::DevInputWriter::open().unwrap();

  let file = std::fs::File::open("data/example-layout-1.json").unwrap();
  let reader = std::io::BufReader::new(file);
  let layout = serde_json::from_reader(reader).unwrap();
  let mut mapper = key_transforms::Mapper::for_layout(layout);
  
  loop {
    let ev_in = r.next().unwrap();
    let evs_out = mapper.step(ev_in);
    
    for ev in evs_out {
      w.send(ev).unwrap();
    }
  }
}

