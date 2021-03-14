use bitvec::prelude::*;

pub struct ButtonState {
  current: BitVec,
  last: BitVec,
}

impl ButtonState {
  pub fn new(capacity: usize) -> Self {
    let mut current = BitVec::with_capacity(capacity);
    current.resize(capacity, false);

    let mut last = BitVec::with_capacity(capacity);
    last.resize(capacity, false);

    Self { current, last }
  }

  pub fn get_current(&self) -> &BitVec {
    &self.current
  }

  pub fn newly_pressed(&self) -> Vec<usize> {
    let mut current = self.current.clone();
    current &= !self.last.clone();
    current.iter_ones().collect::<Vec<usize>>()
  }

  pub fn pressed(&self) -> Vec<usize> {
    self.current.iter_ones().collect::<Vec<usize>>()
  }

  pub fn save(&mut self, state: BitVec) {
    let current = self.current.clone();
    self.last.copy_from_bitslice(&current[..]);
    self.current.copy_from_bitslice(&state[..]);
  }
}
