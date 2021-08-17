use anyhow::{anyhow, Result};
use futures::future::FutureExt;
use futures::pin_mut;
use futures::task;
use log::info;
use music_box::config::{get_config, Config};
use music_box::piano;
use music_box::ui::{MusicBox, UI};
use rand::{thread_rng, Rng};
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::oneshot;
use tokio::sync::RwLock;
use tokio::time::{sleep, timeout, Duration};

const CONFIG: Config = get_config();

fn create_sink(path: String) -> Result<Sink> {
  let device = rodio::default_output_device().unwrap();
  let sink = Sink::new(&device);
  sink.append(Decoder::new(BufReader::new(File::open(path)?))?);
  sink.set_volume(CONFIG.volume);
  Ok(sink)
}

fn play_note(pin: usize) -> Result<Sink> {
  create_sink(CONFIG.pitch_path(pin))
}

struct PerfectPitch {
  sequence: Vec<usize>,
  ui: Arc<RwLock<UI>>,
}

async fn timer() -> Result<()> {
  let (_tx, rx) = oneshot::channel::<()>();
  timeout(Duration::from_secs(2), rx).await??;
  Ok(())
}

impl PerfectPitch {
  pub fn new(ui: Arc<RwLock<UI>>) -> Self {
    let mut sequence = vec![];
    let pin: usize = thread_rng().gen_range(0..=11);
    sequence.push(pin);

    Self {
      sequence,
      ui: Arc::clone(&ui),
    }
  }

  async fn poll(&mut self, expected_pin: usize) -> Result<usize> {
    let t = timer();
    pin_mut!(t);
    let waker = task::noop_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
      let mut pressed = vec![];
      let mut ui = self.ui.write().await;
      ui.poll()?;
      pressed.append(&mut ui.pressed());
      drop(ui);

      if !pressed.is_empty() {
        if pressed.contains(&expected_pin) {
          return Ok(expected_pin);
        } else {
          return Err(anyhow!("wrong pin"));
        }
      }

      if let Poll::Ready(Err(e)) = t.poll_unpin(&mut cx) {
        return Err(e);
      }
      sleep(Duration::from_millis(10)).await;
    }
  }

  pub async fn wait_for_press(&mut self) -> Result<()> {
    for i in 0..self.sequence.len() {
      self.poll(self.sequence[i]).await?;
    }
    Ok(())
  }

  async fn play_sequence(&mut self) -> Result<()> {
    for i in 0..self.sequence.len() {
      let note = play_note(self.sequence[i]);
      let mut ui = self.ui.write().await;
      ui.on(self.sequence[i])?;
      drop(ui);
      sleep(Duration::from_millis(500)).await;
      drop(note);

      let mut ui = self.ui.write().await;
      ui.off(self.sequence[i])?;
      drop(ui);

      sleep(Duration::from_millis(500)).await;
    }

    Ok(())
  }

  fn next(&mut self) -> Result<()> {
    let pin: usize = thread_rng().gen_range(0..=11);
    self.sequence.push(pin);

    Ok(())
  }

  fn reset(&mut self) -> Result<()> {
    self.sequence.clear();
    let pin: usize = thread_rng().gen_range(0..=11);
    self.sequence.push(pin);
    Ok(())
  }
}

async fn run(ui: Arc<RwLock<UI>>) -> Result<()> {
  let mut game = PerfectPitch::new(Arc::clone(&ui));
  loop {
    game.play_sequence().await?;
    let res = game.wait_for_press().await;
    match res {
      Ok(_) => {
        info!("ok");
        let _good_job = create_sink(CONFIG.speech_path("good".to_owned()))?;
        sleep(Duration::from_secs(2)).await;
        game.next()?;
      }
      Err(_) => {
        info!("err");
        //let _try_again = create_sink(CONFIG.speech_path("try_again".to_owned()))?;
        sleep(Duration::from_secs(5)).await;
        game.reset()?;
      }
    }
  }
}

pub async fn start() -> Result<()> {
  let ui = Arc::new(RwLock::new(UI::new()));
  let game = run(Arc::clone(&ui));
  let piano = piano::start(Arc::clone(&ui));

  let intro = create_sink(CONFIG.speech_path("game".to_owned()))?;
  sleep(Duration::from_secs(4)).await;
  drop(intro);

  tokio::join!(tokio::spawn(game), tokio::spawn(piano));
  Ok(())
}
