use super::button_state::ButtonState;
use anyhow::Result;
use bitvec::prelude::*;

pub trait MusicBox {
  fn new() -> Self;

  fn get_state(&self) -> &ButtonState;
  fn save_state(&mut self, state: BitVec) -> Result<()>;
  fn read(&mut self) -> Result<BitVec>;
  fn write(&mut self, button: usize, state: bool) -> Result<()>;
  fn clear(&mut self) -> Result<()>;

  fn pressed(&self) -> Vec<usize> {
    self.get_state().pressed()
  }

  fn newly_pressed(&self) -> Vec<usize> {
    self.get_state().newly_pressed()
  }

  fn newly_released(&self) -> Vec<usize> {
    self.get_state().newly_released()
  }

  fn poll(&mut self) -> Result<()> {
    let state = self.read()?;
    self.save_state(state)
  }

  fn on(&mut self, button: usize) -> Result<()> {
    self.write(button, true)?;
    Ok(())
  }

  fn off(&mut self, button: usize) -> Result<()> {
    self.write(button, false)?;
    Ok(())
  }
}
