
// vim: shiftwidth=2

use std::convert::TryInto;

pub struct StructDeserializer<'a> {
  src: &'a Vec<u8>,
  ptr: usize
}

impl<'a> StructDeserializer<'a> {
  pub fn new(src: &'a Vec<u8>) -> StructDeserializer<'a> {
    StructDeserializer {
      src: src,
      ptr: 0
    }
  }
  
  pub fn read_u16(self: &mut StructDeserializer<'a>) -> Option<u16> {
    if self.ptr + 2 > self.src.len() {
      self.ptr = self.src.len();
      None
    }
    else {
      let res = u16::from_ne_bytes(self.src[self.ptr .. self.ptr+2].try_into().unwrap());
      self.ptr += 2;
      Some(res)
    }
  }
  
  pub fn read_i32(self: &mut StructDeserializer<'a>) -> Option<i32> {
    if self.ptr + 4 > self.src.len() {
      self.ptr = self.src.len();
      None
    }
    else {
      let res = i32::from_ne_bytes(self.src[self.ptr .. self.ptr+4].try_into().unwrap());
      self.ptr += 4;
      Some(res)
    }
  }
  
  pub fn read_i64(self: &mut StructDeserializer<'a>) -> Option<i64> {
    if self.ptr + 8 > self.src.len() {
      self.ptr = self.src.len();
      None
    }
    else {
      let res = i64::from_ne_bytes(self.src[self.ptr .. self.ptr+8].try_into().unwrap());
      self.ptr += 8;
      Some(res)
    }
  }
}

