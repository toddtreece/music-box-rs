use crate::ui::button_state::ButtonState;
use crate::ui::traits::MusicBox;
use anyhow::Result;
use bitvec::prelude::*;
use mcp23017::{PinMode, MCP23017};
use rppal::i2c::I2c;
use shared_bus::{BusManagerStd, I2cProxy};
use std::sync::Mutex;

// datasheet: https://cdn-shop.adafruit.com/datasheets/mcp23017.pdf
const BUTTON_ADDRESS: u8 = 0x24;
const LED_ADDRESS: u8 = 0x20;

const BUTTON_COUNT: usize = 12;
const BUTTON_MAP: [u8; BUTTON_COUNT] = [8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 4];

pub struct RaspberryPI {
  bus: &'static BusManagerStd<I2c>,
  buttons: MCP23017<I2cProxy<'static, Mutex<I2c>>>,
  leds: MCP23017<I2cProxy<'static, Mutex<I2c>>>,
  state: ButtonState,
}

impl MusicBox for RaspberryPI {
  fn new() -> Self {
    let i2c = I2c::new().unwrap();
    let bus: &'static _ = shared_bus::new_std!(I2c = i2c).unwrap();

    let mut buttons = MCP23017::new(bus.acquire_i2c(), BUTTON_ADDRESS).unwrap();
    for pin in BUTTON_MAP.iter() {
      buttons.pull_up(*pin, true).unwrap();
    }
    buttons.all_pin_mode(mcp23017::PinMode::INPUT).unwrap();

    let mut leds = MCP23017::new(bus.acquire_i2c(), LED_ADDRESS).unwrap();
    leds.all_pin_mode(PinMode::OUTPUT).unwrap();
    leds.write_gpioab(0).unwrap();

    Self {
      bus,
      buttons,
      leds,
      state: ButtonState::new(BUTTON_COUNT),
    }
  }

  fn get_state(&self) -> &ButtonState {
    &self.state
  }

  fn save_state(&mut self, state: BitVec) -> Result<()> {
    self.state.save(state);
    Ok(())
  }

  fn read(&mut self) -> Result<BitVec> {
    let bytes = self.buttons.read_gpioab()?.to_le_bytes();
    let source = BitVec::<Lsb0, _>::from_slice(&bytes)?;
    let mut formatted = BitVec::with_capacity(BUTTON_COUNT);
    formatted.resize(BUTTON_COUNT, false);

    for i in 0..BUTTON_COUNT {
      // button values are reversed when they are read,
      // so we need to use true as the default
      let state: bool = *source.get(i).unwrap();

      // flip the state so that pressed == true
      formatted.set(i, !state);
    }

    Ok(formatted)
  }

  fn write(&mut self, pin: usize, value: bool) -> Result<()> {
    self.leds.digital_write(BUTTON_MAP[pin], value)?;
    Ok(())
  }

  fn clear(&mut self) -> Result<()> {
    self.leds.write_gpioab(0)?;
    Ok(())
  }
}
