use std::collections::VecDeque;
use std::time::{Duration, Instant};

use sdl2::event::Event;

use crate::error::Result;
use crate::graphics::{self, Color};
use crate::input::{self, KeyboardContext, MouseContext};
use crate::time;
use crate::State;

pub struct Context {
    is_running: bool,
    tick_rate: Duration,
    // TODO: This should probably be in a dedicated struct. No primitive obsession!
    pub(crate) fps_tracker: VecDeque<f64>,
    pub(crate) keyboard: KeyboardContext,
    pub(crate) mouse: MouseContext,
}

impl Context {
    pub fn run<S>(&mut self, state: &mut S) -> Result<()>
    where
        S: State,
    {
        let mut last_time = Instant::now();
        let mut lag = Duration::from_secs(0);

        self.is_running = true;

        while self.is_running {
            let current_time = Instant::now();
            let elapsed_time = current_time - last_time;
            last_time = current_time;
            lag += elapsed_time;

            self.fps_tracker.pop_front();
            self.fps_tracker
                .push_back(time::duration_to_f64(elapsed_time));

            while lag >= self.tick_rate {
                if let Err(err) = state.update(self) {
                    self.is_running = false;
                    return Err(err);
                }

                input::cleanup_after_state_update(self);
                lag -= self.tick_rate;
            }

            let dt = time::duration_to_f64(lag) / time::duration_to_f64(self.tick_rate);

            graphics::clear(self, Color::CADET_BLUE);

            if let Err(err) = state.draw(self, dt) {
                self.is_running = false;
                return Err(err);
            }

            std::thread::yield_now();
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<Event> {
        match event {
            Event::Quit {..} => self.is_running = false,
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
    fullscreen: bool,
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

    pub fn fullscreen(&mut self, fullscreen: bool) -> &mut Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn build(&self) -> Result<Context> {
        let mut fps_tracker = VecDeque::with_capacity(200);
        fps_tracker.resize(200, 1.0 / 60.0);

        Ok(Context {
            is_running: false,
            tick_rate: time::f64_to_duration(self.tick_rate),
            fps_tracker,
            keyboard: KeyboardContext::new(),
            mouse: MouseContext::new(),
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
            fullscreen: false,
            quit_on_escape: true,
        }
    }
}
