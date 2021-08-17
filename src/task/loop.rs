use anyhow::Result;
use music_box::{
  config::{get_config, Config},
  sequencer::Sequencer,
  ui::{MusicBox, UI},
};
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;

const STEPS: usize = 8;
const QUANTIZE: usize = 4;
const CONFIG: Config = get_config();

pub fn start() -> Result<()> {
  let mut ui = UI::new();
  let device = rodio::default_output_device().unwrap();
  let sink = Sink::new(&device);
  let source = Decoder::new(BufReader::new(File::open(
    CONFIG.loop_path("loop".to_owned()),
  )?))?;

  let sequencer = Sequencer::new(source, STEPS * QUANTIZE, move |current, next| {
    ui.poll().unwrap();
    let mut new_next = next;
    let pressed = ui.pressed();

    if let Some(pin) = pressed.get(0) {
      if *pin < STEPS {
        new_next = *pin * QUANTIZE;
      } else {
        new_next = 0;
      }
    }

    if new_next == current {
      new_next = next;
    }

    if current % QUANTIZE == 0 {
      ui.off(current / QUANTIZE).unwrap();
    }
    if new_next % QUANTIZE == 0 {
      ui.on(new_next / QUANTIZE).unwrap();
    }
    new_next
  });

  sink.append(sequencer);
  sink.set_volume(0.005);
  sink.sleep_until_end();

  Ok(())
}
