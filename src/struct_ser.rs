
// vim: shiftwidth=2

pub struct StructSerializer {
  pub sink: Vec<u8>
}

impl StructSerializer {
  pub fn add_string_in_buf(self: &mut StructSerializer, text: &str, buff_size: usize) {
    let text_bytes = text.as_bytes();
    
    for i in 0 .. text_bytes.len() {
      self.sink.push(text_bytes[i]);
    }
    
    for _i in 0 .. (buff_size - text_bytes.len()) {
      self.sink.push(0);
    }
  }
  
  #[allow(dead_code)]
  pub fn add_u8(self: &mut StructSerializer, x: u8) {
    for b in &x.to_ne_bytes() {
      self.sink.push(*b);
    }
  }
  
  pub fn add_u16(self: &mut StructSerializer, x: u16) {
    for b in &x.to_ne_bytes() {
      self.sink.push(*b);
    }
  }
  
  pub fn add_u32(self: &mut StructSerializer, x: u32) {
    for b in &x.to_ne_bytes() {
      self.sink.push(*b);
    }
  }
  
  pub fn add_i32(self: &mut StructSerializer, x: i32) {
    for b in &x.to_ne_bytes() {
      self.sink.push(*b);
    }
  }
  
  #[allow(dead_code)]
  pub fn add_u64(self: &mut StructSerializer, x: u64) {
    for b in &x.to_ne_bytes() {
      self.sink.push(*b);
    }
  }
  
  pub fn add_i64(self: &mut StructSerializer, x: i64) {
    for b in &x.to_ne_bytes() {
      self.sink.push(*b);
    }
  }
  
  pub fn add_i32_array(self: &mut StructSerializer, xs: &[i32]) {
    for x in xs {
      for b in &x.to_ne_bytes() {
        self.sink.push(*b);
      }
    }
  }
}

