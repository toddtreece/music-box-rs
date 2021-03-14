use super::take_duration::{take_duration, TakeDuration};
use rodio::source::Buffered;
use rodio::{Sample, Source};
use std::convert::TryInto;
use std::time::Duration;

const NS_PER_SECOND: u128 = 1_000_000_000;

fn skip_duration<I>(input: &mut I, duration: Duration)
where
  I: Source,
  I::Item: Sample,
{
  let samples_per_channel: u128 = duration.as_nanos() * input.sample_rate() as u128 / NS_PER_SECOND;
  let samples_to_skip: u128 = samples_per_channel * input.channels() as u128;

  for _ in 0..samples_to_skip {
    if input.next().is_none() {
      break;
    }
  }
}

pub struct Sequencer<I, F>
where
  I: Source,
  I::Item: Sample,
  F: FnMut(usize, usize) -> usize,
{
  inner: TakeDuration<Buffered<I>>,
  steps: usize,
  step_duration: Duration,
  queue: Vec<TakeDuration<Buffered<I>>>,
  current_cursor: usize,
  last_cursor: usize,
  tick_fn: F,
}

impl<I, F> Sequencer<I, F>
where
  I: Source,
  I::Item: Sample,
  F: FnMut(usize, usize) -> usize,
{
  pub fn new(input: I, steps: usize, tick_fn: F) -> Self {
    let input = input.buffered();
    let total_nanos = input.total_duration().unwrap().as_nanos();
    let step_nanos = total_nanos.wrapping_div(steps as u128);
    let step_duration = Duration::from_nanos(step_nanos.try_into().unwrap());

    let mut queue: Vec<TakeDuration<Buffered<I>>> = Vec::new();

    for step in 0..steps {
      let chunk_nanos: u64 = (step_nanos * step as u128).try_into().unwrap();
      let chunk_skip = Duration::from_nanos(chunk_nanos);
      let mut chunk = input.clone();
      skip_duration(&mut chunk, chunk_skip);
      queue.push(take_duration(chunk, step_duration));
    }

    Self {
      inner: queue.get(0).unwrap().clone(),
      steps,
      step_duration,
      queue,
      current_cursor: 0,
      last_cursor: 0,
      tick_fn,
    }
  }

  pub fn get_current_cursor(&self) -> usize {
    self.current_cursor
  }

  pub fn get_previous_cursor(&self) -> usize {
    self.last_cursor
  }

  pub fn get_step(&self, step: usize) -> usize {
    if step < self.steps {
      step
    } else {
      0
    }
  }

  pub fn get_next_step(&self) -> usize {
    self.get_step(self.current_cursor + 1)
  }

  pub fn get_previous_step(&self) -> usize {
    self.get_step(self.current_cursor - 1)
  }

  pub fn get_chunk(&self, step: usize) -> TakeDuration<Buffered<I>> {
    self.queue.get(step).unwrap().clone()
  }

  pub fn get_next_chunk(&self) -> TakeDuration<Buffered<I>> {
    self.get_chunk(self.get_next_step())
  }

  pub fn set_step(&mut self, step: usize) {
    self.last_cursor = self.current_cursor;
    self.current_cursor = self.get_step(step);
    self.inner = self.get_chunk(self.current_cursor);
  }

  pub fn next_step(&mut self) {
    self.set_step(self.get_next_step());
  }

  pub fn previous_step(&mut self) {
    self.set_step(self.get_previous_step());
  }
}

impl<I, F> Iterator for Sequencer<I, F>
where
  I: Source,
  I::Item: Sample,
  F: FnMut(usize, usize) -> usize,
{
  type Item = <I as Iterator>::Item;

  #[inline]
  fn next(&mut self) -> Option<<I as Iterator>::Item> {
    if let Some(value) = self.inner.next() {
      return Some(value);
    }

    let current_cursor = self.get_current_cursor();
    let next_cursor = self.get_next_step();
    let next_step = (self.tick_fn)(current_cursor, next_cursor);
    self.set_step(next_step);
    self.inner.next()
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    // infinite
    (0, None)
  }
}

impl<I, F> Source for Sequencer<I, F>
where
  I: Iterator + Source,
  I::Item: Sample,
  F: FnMut(usize, usize) -> usize,
{
  #[inline]
  fn current_frame_len(&self) -> Option<usize> {
    match self.inner.current_frame_len() {
      Some(0) => {
        let next = self.get_next_chunk();
        next.current_frame_len()
      }
      a => a,
    }
  }

  #[inline]
  fn channels(&self) -> u16 {
    match self.inner.current_frame_len() {
      Some(0) => {
        let next = self.get_next_chunk();
        next.channels()
      }
      _ => self.inner.channels(),
    }
  }

  #[inline]
  fn sample_rate(&self) -> u32 {
    match self.inner.current_frame_len() {
      Some(0) => {
        let next = self.get_next_chunk();
        next.sample_rate()
      }
      _ => self.inner.sample_rate(),
    }
  }

  #[inline]
  fn total_duration(&self) -> Option<Duration> {
    None
  }
}

impl<I, F: Clone> Clone for Sequencer<I, F>
where
  I: Source,
  I::Item: Sample,
  F: FnMut(usize, usize) -> usize,
{
  #[inline]
  fn clone(&self) -> Sequencer<I, F> {
    Sequencer {
      inner: self.inner.clone(),
      steps: self.steps,
      step_duration: self.step_duration.clone(),
      queue: self.queue.clone(),
      current_cursor: self.current_cursor,
      last_cursor: self.last_cursor,
      tick_fn: self.tick_fn.clone(),
    }
  }
}
