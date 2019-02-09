//! Peacock is a game engine for making beautiful games.
//!
//! [![Crates.io](https://img.shields.io/crates/v/peacock.svg)](https://crates.io/crates/peacock)
//! [![Docs.rs](https://docs.rs/peacock/badge.svg)](https://docs.rs/peacock/)
//! [![Crates.io](https://img.shields.io/crates/l/peacock.svg)](https://github.com/maxdeviant/peacock/blob/master/LICENSE)
//!
//! ## Installation
//! ```toml
//! [dependencies]
//! peacock = "0.0.1"
//! ```

pub mod error;
pub mod graphics;

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::window::{Event, Style};

use crate::graphics::Color;

use self::error::Result;

pub fn duration_to_f64(duration: Duration) -> f64 {
    let seconds = duration.as_secs() as f64;
    let nanos = f64::from(duration.subsec_nanos()) * 1e-9;
    seconds + nanos
}

pub fn f64_to_duration(duration: f64) -> Duration {
    debug_assert!(duration >= 0.0);
    let seconds = duration.trunc() as u64;
    let nanos = (duration.fract() * 1e9) as u32;
    Duration::new(seconds, nanos)
}

pub fn get_fps(ctx: &Context) -> f64 {
    1.0 / (ctx.fps_tracker.iter().sum::<f64>() / ctx.fps_tracker.len() as f64)
}

pub trait State {
    fn update(&mut self, ctx: &mut Context) -> Result<()>;
    fn draw(&mut self, ctx: &mut Context, dt: f64) -> Result<()>;
}

pub struct Context {
    // TODO: This shouldn't be public.
    pub window: RenderWindow,
    is_running: bool,
    tick_rate: Duration,
    // TODO: This should probably be in a dedicated struct. No primitive obsession!
    fps_tracker: VecDeque<f64>,
}

impl Context {
    pub fn run<S>(&mut self, state: &mut S) -> Result<()>
    where
        S: State,
    {
        self.window.set_active(true);

        let mut last_time = Instant::now();
        let mut lag = Duration::from_secs(0);

        self.is_running = true;

        while self.is_running {
            let current_time = Instant::now();
            let elapsed_time = current_time - last_time;
            last_time = current_time;
            lag += elapsed_time;

            self.fps_tracker.pop_front();
            self.fps_tracker.push_back(duration_to_f64(elapsed_time));

            while let Some(event) = self.window.poll_event() {
                if let Err(err) = self.handle_event(event) {
                    self.is_running = false;
                    return Err(err);
                }
            }

            while lag >= self.tick_rate {
                if let Err(err) = state.update(self) {
                    self.is_running = false;
                    return Err(err);
                }

                lag -= self.tick_rate;
            }

            let dt = duration_to_f64(lag) / duration_to_f64(self.tick_rate);

            self.window.clear(&Color::MAGENTA);

            if let Err(err) = state.draw(self, dt) {
                self.is_running = false;
                return Err(err);
            }

            self.window.display();

            std::thread::yield_now();
        }

        self.window.close();

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<Event> {
        match event {
            Event::Closed => self.is_running = false,
            _ => {}
        }

        Ok(event)
    }
}

#[derive(Debug, Clone)]
pub struct ContextBuilder<'a> {
    title: &'a str,
    width: i32,
    height: i32,
    vsync: bool,
    tick_rate: f64,
    quit_on_escape: bool,
}

impl<'a> ContextBuilder<'a> {
    pub fn new(title: &'a str, width: i32, height: i32) -> Self {
        Self {
            title,
            width,
            height,
            ..ContextBuilder::default()
        }
    }

    pub fn vsync(&mut self, vsync: bool) -> &mut Self {
        self.vsync = vsync;
        self
    }

    pub fn tick_rate(&mut self, tick_rate: f64) -> &mut Self {
        self.tick_rate = tick_rate;
        self
    }

    pub fn build(&self) -> Result<Context> {
        let mut window = RenderWindow::new(
            (self.width as u32, self.height as u32),
            self.title,
            Style::CLOSE,
            &Default::default(),
        );

        window.set_vertical_sync_enabled(self.vsync);

        let mut fps_tracker = VecDeque::with_capacity(200);
        fps_tracker.resize(200, 1.0 / 60.0);

        Ok(Context {
            window,
            is_running: false,
            tick_rate: f64_to_duration(self.tick_rate),
            fps_tracker,
        })
    }
}

impl<'a> Default for ContextBuilder<'a> {
    fn default() -> Self {
        Self {
            title: "Game",
            width: 800,
            height: 600,
            vsync: true,
            tick_rate: 1.0 / 60.0,
            quit_on_escape: true,
        }
    }
}
