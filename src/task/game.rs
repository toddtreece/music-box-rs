use anyhow::Result;
use log::info;
use music_box::config::{get_config, Config};
use music_box::piano;
use music_box::ui::{MusicBox, UI};
use music_box::util::play_sound;
use rand::{thread_rng, Rng};
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

const CONFIG: Config = get_config();
const NOTE_DURATION: u64 = 800;
const NOTE_SEPERATION: u64 = 150;

struct PerfectPitch {
  sequence: Vec<usize>,
}

impl PerfectPitch {
  pub fn new() -> Self {
    let mut sequence = vec![];
    let pin: usize = thread_rng().gen_range(0..=11);
    sequence.push(pin);

    Self { sequence }
  }

  fn next(&mut self) -> Result<()> {
    let pin: usize = thread_rng().gen_range(0..=11);
    self.sequence.push(pin);

    Ok(())
  }

  fn reset(&mut self) -> Result<()> {
    self.sequence.clear();
    Ok(())
  }
}

pub async fn start() -> Result<()> {
  let mut ui = UI::new();
  let mut game = PerfectPitch::new();
  let mut piano = piano::Piano::new();

  let intro = play_sound(CONFIG.speech_path("game".to_owned()))?;
  sleep(Duration::from_secs(4)).await;
  drop(intro);

  loop {
    let sequence = game.sequence.clone();
    for button in sequence.iter() {
      ui.on(*button)?;
      piano.note_on(*button)?;
      sleep(Duration::from_millis(NOTE_DURATION)).await;
      ui.off(*button)?;
      piano.note_off(*button)?;
      sleep(Duration::from_millis(NOTE_SEPERATION)).await;
    }

    let mut reset = false;
    for button in sequence.iter() {
      let (tx, mut rx) = oneshot::channel::<bool>();

      tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;
        if !tx.is_closed() {
          tx.send(true).unwrap();
        }
      });

      loop {
        ui.poll()?;
        if let Ok(true) = rx.try_recv() {
          reset = true;
          break;
        }

        for button in ui.newly_pressed() {
          ui.on(button)?;
          piano.note_on(button)?;
        }

        if !ui.newly_pressed().is_empty() {
          if ui.newly_pressed().contains(button) {
            info!("ok");
            drop(rx);
            break;
          } else {
            info!("err");
            ui.clear()?;
            for button in 0..=11 {
              piano.note_off(button)?;
            }
            drop(rx);
            reset = true;
            let _try_again = play_sound(CONFIG.speech_path("try_again".to_owned()))?;
            sleep(Duration::from_secs(3)).await;
            break;
          }
        }
        sleep(Duration::from_millis(10)).await;
      }

      sleep(Duration::from_millis(NOTE_DURATION / 2)).await;
      ui.clear()?;
      for button in 0..=11 {
        piano.note_off(button)?;
      }
      if reset {
        game.reset()?;
        sleep(Duration::from_secs(2)).await;
        break;
      }
    }

    if !reset {
      let _good_job = play_sound(CONFIG.speech_path("good".to_owned()))?;
      sleep(Duration::from_secs(2)).await;
    }

    game.next()?;
    sleep(Duration::from_secs(1)).await;
  }
}
