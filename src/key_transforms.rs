
// vim: shiftwidth=2

use crate::keys::{Layout, Mapping, KeyCode, Pressed, Released, Event};

use std::collections::HashMap;
use std::cmp::Ordering;

fn final_key(trigger: &Vec<KeyCode>) -> KeyCode {
  return trigger[trigger.len() - 1];
}

fn is_supported(trigger: &Vec<KeyCode>, pressed_keys: &Vec<KeyCode>, new_key: &KeyCode) -> bool {
  for k in trigger {
    if !(pressed_keys.contains(&k) || k == new_key) {
      return false;
    }
  }
  return true;
}

fn fails_when_released(trigger: &Vec<KeyCode>, key: &KeyCode) -> bool {
  for k in trigger {
    if k == key {
      return true;
    }
  }
  return false;
}

#[derive(Debug)]
struct State {
  input_pressed_keys: Vec<KeyCode>,
  active_mappings: Vec<Mapping>,
  pass_through_keys: Vec<KeyCode>,
  mapped_output_keys: Vec<KeyCode>,
}

fn init_state() -> State {
  return State {
    input_pressed_keys: Vec::new(),
    active_mappings: Vec::new(),
    pass_through_keys: Vec::new(),
    mapped_output_keys: Vec::new(),
  };
}

struct HashedLayout {
  mappings: HashMap<KeyCode, Vec<Mapping>>
}

fn trigger_priority(t1: &Vec<KeyCode>, t2: &Vec<KeyCode>) -> Ordering {
  if t1.len() > t2.len() {
    return Ordering::Less;
  }
  else if t1.len() < t2.len() {
    return Ordering::Greater;
  }
  else {
    for i in (0 .. t1.len()).rev() {
      if t1[i] < t2[i] {
        return Ordering::Less;
      }
      else if t1[i] > t1[i] {
        return Ordering::Greater;
      }
    }
    return Ordering::Equal;
  }
}

fn mapping_priority(m1: &Mapping, m2: &Mapping) -> Ordering {
  return trigger_priority(&m1.from, &m2.from);
}

fn make_hashed_layout(layout: &Layout) -> HashedLayout {
  let mut mappings: HashMap<KeyCode, Vec<Mapping>> = HashMap::new();

  for mapping in &layout.0 {
    for i in 0 .. mapping.from.len() {
      for j in i+1 .. mapping.from.len() {
        if mapping.from[i] == mapping.from[j] {
          panic!("Duplicate key in from");
        }
      }
    }
    
    for i in 0 .. mapping.to.len() {
      for j in i+1 .. mapping.to.len() {
        if mapping.to[i] == mapping.to[j] {
          panic!("Duplicate key in to");
        }
      }
    }
  }
  
  for mapping in &layout.0 {
    let last = final_key(&mapping.from);
    match mappings.get_mut(&last) {
      None => {
        mappings.insert(last, vec![mapping.clone()]);
      },
      Some(existing) => {
        existing.push(mapping.clone());
        existing.sort_by(mapping_priority);
      }
    }
  }
  
  return HashedLayout { mappings: mappings };
}

pub struct Mapper {
  layout: HashedLayout,
  state: State
}

impl Mapper {
  pub fn for_layout(layout: &Layout) -> Mapper {
    Mapper {
      layout: make_hashed_layout(layout),
      state: init_state()
    }
  }
  
  pub fn step(self: &mut Mapper, input: Event) -> Vec<Event> {
    let state = &mut self.state;

    match input {
      Pressed(k) => {
        if !state.input_pressed_keys.contains(&k) {
          return newly_press(self, k);
        }
        else {
          return vec![];
        }
      },
      Released(k) => {
        if state.input_pressed_keys.contains(&k) {
          return newly_release(self, k);
        }
        else {
          return vec![];
        }
      },
    }
  }
  
  pub fn release_all(self: &mut Mapper) -> Vec<Event> {
    let to_release = self.state.input_pressed_keys.clone();
    
    let mut res: Vec<Event> = Vec::new();
    
    for k in to_release {
      let mut chunk = self.step(Released(k));
      res.append(&mut chunk);
    }
    
    res
  }
}

fn is_action_key(k: &KeyCode) -> bool {
  use KeyCode::{LEFTSHIFT, RIGHTSHIFT, LEFTMETA, RIGHTMETA, LEFTCTRL, RIGHTCTRL};
  
  match k {
    LEFTSHIFT => false,
    RIGHTSHIFT => false,
    LEFTMETA => false,
    RIGHTMETA => false,
    LEFTCTRL => false,
    RIGHTCTRL => false,
    _ => true
  }
}

fn is_action_mapping(m: &Mapping) -> bool {
  if m.to.len() == 0 {
    false
  }
  else {
    let last_key = &m.to[m.to.len() - 1];
    is_action_key(last_key)
  }
}

fn add_new_mapping(state: &mut State, m: &Mapping) -> Vec<Event> {
  let mut res: Vec<Event> = Vec::new();
  
  let pass_through_keys = &mut state.pass_through_keys;
  let mapped_output_keys = &mut state.mapped_output_keys;
  
  pass_through_keys.retain(|&old_key| {
    if m.from.contains(&old_key) || m.to.contains(&old_key) {
      if !m.to.contains(&old_key) {
        res.push(Released(old_key));
        return false;
      }
      else {
        mapped_output_keys.push(old_key);
        return false;
      }
    }
    else {
      return true;
    }
  });
  
  // An "action mapping" is one that has an immediate effect, like
  // typing a key. The modifiers should be stripped immediately after
  // they are used to avoid having them pressed when the next key is pressed.
  if is_action_mapping(&m) {
    let last_key = &m.to[m.to.len()-1];
    
    let last_key_currently_pressed = state.mapped_output_keys.contains(last_key) || state.pass_through_keys.contains(last_key);
    
    if last_key_currently_pressed {
      // Make sure every action mapping does something. If the key
      // is already pressed, release it, and press it again!
      res.push(Released(*last_key));
    }
    
    let mut m_modifiers_to_press: Vec<KeyCode> = Vec::new();
    for i in 0..m.to.len()-1 {
      let potential_new_key = &m.to[i];
      if !state.mapped_output_keys.contains(potential_new_key) && !state.pass_through_keys.contains(potential_new_key) {
        m_modifiers_to_press.push(*potential_new_key);
      }
    }
    
    for k in &m_modifiers_to_press {
      res.push(Pressed(*k));
    }
    res.push(Pressed(*last_key));
    for k in (&m_modifiers_to_press).iter().rev() {
      res.push(Released(*k));
    }
    
    let modifier_stripped_mapping = Mapping {
      from: m.from.clone(),
      to: vec![*last_key]
    };
    
    state.active_mappings.push(modifier_stripped_mapping);
    state.mapped_output_keys.push(*last_key);
  }
  else {
    for new_key in &m.to {
      if !state.mapped_output_keys.contains(new_key) {
        if state.pass_through_keys.contains(new_key) {
          state.pass_through_keys.retain(|k2| k2 != new_key);
          state.mapped_output_keys.push(*new_key);
        }
        else {
          res.push(Pressed(*new_key));
          state.mapped_output_keys.push(*new_key);
        }
      }
    }
    
    state.active_mappings.push(m.clone());
  }
  
  return res;
}

fn newly_press(mapper: &mut Mapper, k: KeyCode) -> Vec<Event> {
  let mappings = &mapper.layout.mappings;
  let state = &mut mapper.state;
  
  let mut res: Vec<Event> = Vec::new();
  
  for mappings in mappings.get(&k) {
    for mapping in mappings {
      if is_supported(&mapping.from, &state.input_pressed_keys, &k) {
        res.append(&mut add_new_mapping(state, mapping));
        break;
      }
    }
  }
  
  let mut any_hit: bool = false;
  for m in &state.active_mappings {
    if m.from.contains(&k) {
      any_hit = true;
      break;
    }
    else if m.to.contains(&k) {
      any_hit = true;
      break;
    }
  }
  
  if !any_hit {
    if !state.pass_through_keys.contains(&k){
      res.push(Pressed(k));
      state.pass_through_keys.push(k);
    }
  }
  
  state.input_pressed_keys.push(k);
  
  return res;
}

fn remove_mapping(state: &mut State, i: usize, removed_key: KeyCode) -> Vec<Event> {
  let mut res: Vec<Event> = Vec::new();
  
  let active_mappings = &mut state.active_mappings;
  let input_pressed_keys = &state.input_pressed_keys;
  let pass_through_keys = &mut state.pass_through_keys;

  for mapped_output_i in (0 .. state.mapped_output_keys.len()).rev() {
    let k = state.mapped_output_keys[mapped_output_i];
    
    let mut still_used: bool = false;
    for j in 0 .. active_mappings.len() {
      if j != i {
        if active_mappings[j].to.contains(&k) {
          still_used = true;
          break;
        }
      }
    }

    if !still_used {
      if input_pressed_keys.contains(&k) && k != removed_key {
        let mut still_shadowed = false;
        for j in 0 .. active_mappings.len() {
          if j != i {
            if active_mappings[j].from.contains(&k) {
              still_shadowed = true;
              break;
            }
          }
        }
        if !still_shadowed {
          pass_through_keys.push(k);
        }
        else {
          res.push(Released(k));
        }
      }
      else {
        res.push(Released(k));
      }
    }
    
    if !still_used {
      state.mapped_output_keys.remove(mapped_output_i);
    }
  }
    
  active_mappings.remove(i);
  
  return res;
}

fn newly_release(mapper: &mut Mapper, k: KeyCode) -> Vec<Event> {
  let state = &mut mapper.state;
  
  let mut res: Vec<Event> = Vec::new();
  
  let mut i: isize = state.active_mappings.len() as isize - 1;
  while i >= 0 {
    if fails_when_released(&state.active_mappings[i as usize].from, &k) {
      res.append(&mut remove_mapping(state, i as usize, k));
    }
    i -= 1;
  }
  
  for i in (0 .. state.pass_through_keys.len()).rev() {
    if state.pass_through_keys[i] == k {
      res.push(Released(k));
      state.pass_through_keys.remove(i);
      break;
    }
  }
  
  state.input_pressed_keys.retain(|&old_key| {
    old_key != k
  });
  
  return res;
}

#[cfg(test)]
mod tests {
  use super::*;
  use KeyCode::*;
  
  #[test]
  fn test_most_basic() {
    let layout = Layout(vec![
      Mapping { from: vec![A], to: vec![B] },
    ]);
    let mut mapper = Mapper::for_layout(&layout);
    assert_eq!(vec![Pressed(B)], mapper.step(Pressed(A)));
  }
  
  #[test]
  fn test_single_key_remap() {
    let layout = Layout(vec![
      Mapping { from: vec![A], to: vec![B] },
    ]);
    let mut mapper = Mapper::for_layout(&layout);
    assert_eq!(vec![Pressed(B)], mapper.step(Pressed(A)));
    assert_eq!(vec![Released(B)], mapper.step(Released(A)));
    assert_eq!(vec![Pressed(C)], mapper.step(Pressed(C)));
    assert_eq!(vec![Released(C)], mapper.step(Released(C)));
    assert_eq!(vec![Pressed(LEFTSHIFT)], mapper.step(Pressed(LEFTSHIFT)));
    assert_eq!(vec![Pressed(B)], mapper.step(Pressed(A)));
  }
  
  #[test]
  fn test_multi_key_overlap() {
    let layout = Layout(vec![
      Mapping { from: vec![CAPSLOCK], to: vec![] },
      Mapping { from: vec![CAPSLOCK, M], to: vec![LEFTSHIFT, EQUAL] },
      Mapping { from: vec![CAPSLOCK, U], to: vec![EQUAL] },
    ]);
    let mut mapper = Mapper::for_layout(&layout);
    let empty: Vec<Event> = Vec::new();
    
    assert_eq!(empty, mapper.step(Pressed(CAPSLOCK)));
    assert_eq!(vec![Pressed(LEFTSHIFT), Pressed(EQUAL), Released(LEFTSHIFT)], mapper.step(Pressed(M)));
    assert_eq!(vec![Released(EQUAL), Pressed(EQUAL)], mapper.step(Pressed(U)));
  }
  
  #[test]
  fn test_super_multi() {
    let layout = Layout(vec![
      Mapping { from: vec![CAPSLOCK], to: vec![] },
      Mapping { from: vec![TAB], to: vec![] },
      Mapping { from: vec![F], to: vec![U] },
      Mapping { from: vec![N], to: vec![B] },
      Mapping { from: vec![CAPSLOCK, M], to: vec![LEFTSHIFT, EQUAL] },
      Mapping { from: vec![CAPSLOCK, F], to: vec![EQUAL] },
      Mapping { from: vec![CAPSLOCK, N], to: vec![LEFTSHIFT, K1] },
      Mapping { from: vec![TAB, M], to: vec![PAGEDOWN] },
      Mapping { from: vec![TAB, N], to: vec![LEFTCTRL, LEFT] },
    ]);
    let mut mapper = Mapper::for_layout(&layout);
    
    let empty: Vec<Event> = Vec::new();
    
    assert_eq!(vec![Pressed(LEFTSHIFT)], mapper.step(Pressed(LEFTSHIFT)));
    assert_eq!(empty, mapper.step(Pressed(TAB)));
    assert_eq!(vec![Pressed(LEFTCTRL), Pressed(LEFT), Released(LEFTCTRL)], mapper.step(Pressed(N)));
    assert_eq!(vec![Released(LEFT)], mapper.step(Released(N)));
    assert_eq!(empty, mapper.step(Released(TAB)));
    assert_eq!(vec![Pressed(M)], mapper.step(Pressed(M)));
    assert_eq!(vec![Released(M)], mapper.step(Released(M)));
    assert_eq!(vec![Released(LEFTSHIFT)], mapper.step(Released(LEFTSHIFT)));
    assert_eq!(empty, mapper.step(Pressed(CAPSLOCK)));
    assert_eq!(vec![Pressed(LEFTSHIFT), Pressed(EQUAL), Released(LEFTSHIFT)], mapper.step(Pressed(M)));
  }
}

