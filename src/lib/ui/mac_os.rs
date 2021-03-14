use super::button_state::ButtonState;
use super::traits::MusicBox;
use anyhow::Result;
use bitvec::prelude::*;
use std::char::from_digit;
use std::convert::TryInto;
use std::io;
use termion::{
  event::Key, input::TermRead, raw::IntoRawMode, raw::RawTerminal, screen::AlternateScreen,
};
use tui::{
  backend::TermionBackend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Style},
  widgets::{Block, BorderType, Borders},
  Terminal,
};

const BUTTON_COUNT: usize = 8;

pub struct MacOS {
  terminal: Terminal<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
  buttons: ButtonState,
  leds: ButtonState,
}

impl MacOS {
  fn draw(&mut self) -> Result<()> {
    let current = self.leds.get_current();

    self.terminal.draw(|f| {
      let size = f.size();
      let pressed = Style::default().bg(Color::White);
      let default = Style::default().bg(Color::Black);

      let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

      let col_count = 4;
      let row_count = BUTTON_COUNT / col_count;

      let vertical_constraints = (0..row_count)
        .map(|_| Constraint::Percentage((100 / row_count).try_into().unwrap()))
        .collect::<Vec<Constraint>>();

      let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(size);

      let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
          [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
          ]
          .as_ref(),
        );

      for row in 0..row_count {
        for col in 0..col_count {
          let index = row * col;
          if let Some(is_pressed) = current.get(index) {
            if *is_pressed {
              f.render_widget(
                block.clone().style(pressed),
                horizontal.split(vertical[row])[col],
              );
            } else {
              f.render_widget(
                block.clone().style(default),
                horizontal.split(vertical[row])[col],
              );
            }
          }
        }
      }
    })?;

    Ok(())
  }
}

impl MusicBox for MacOS {
  fn new() -> MacOS {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    MacOS {
      buttons: ButtonState::new(BUTTON_COUNT),
      leds: ButtonState::new(BUTTON_COUNT),
      terminal,
    }
  }

  fn get_state(&self) -> &ButtonState {
    &self.buttons
  }

  fn save_state(&mut self, state: BitVec) -> Result<()> {
    self.buttons.save(state);
    Ok(())
  }

  fn read(&mut self) -> Result<BitVec> {
    let mut state = BitVec::with_capacity(BUTTON_COUNT);
    state.resize(BUTTON_COUNT, false);

    //let key_map = (0..(BUTTON_COUNT as u32))
    //  .map(|i| from_digit(i, 10).unwrap())
    //  .collect::<Vec<char>>();

    //for pressed in io::stdin().keys() {
    //  let keys = key_map.clone();
    //  let pressed_key = pressed.unwrap();
    //  for (i, key) in keys.iter().enumerate() {
    //    match pressed_key {
    //      Key::Char(pressed_key) if *key == pressed_key => state.set(i, true),
    //      _ => (),
    //    }
    //  }
    //}
    self.draw()?;

    Ok(state)
  }

  fn write(&mut self, pin: usize, value: bool) -> Result<()> {
    let mut current = self.leds.get_current().clone();
    current.set(pin, value);
    self.leds.save(current);
    self.draw()?;
    Ok(())
  }

  fn clear(&mut self) -> Result<()> {
    let mut current: BitVec = self.leds.get_current().clone();
    current.set_all(false);
    self.leds.save(current);
    self.draw()?;
    Ok(())
  }
}
