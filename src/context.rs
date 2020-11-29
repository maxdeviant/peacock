use std::time::{Duration, Instant};

use lazy_static::*;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::ecs::World;
use crate::error::{AnyhowContext, Result, Sdl2Error};
use crate::graphics::{self, Color, GraphicsContext};
use crate::input::{self, KeyboardContext, MouseContext};
use crate::time;
use crate::{FpsTracker, State};

lazy_static! {
    pub(crate) static ref SDL_TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

pub struct Context {
    pub(crate) sdl_context: Sdl,
    pub(crate) canvas: Canvas<Window>,
    is_running: bool,
    tick_rate: Duration,
    pub(crate) fps_tracker: FpsTracker,
    pub(crate) world: World,
    pub(crate) graphics: GraphicsContext,
    pub(crate) keyboard: KeyboardContext,
    pub(crate) mouse: MouseContext,
}

impl Context {
    /// Runs the context using the provided game state.
    pub fn run<S>(&mut self, state: &mut S) -> Result<()>
    where
        S: State,
    {
        let mut last_time = Instant::now();
        let mut lag = Duration::from_secs(0);

        self.is_running = true;

        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(Sdl2Error::ErrorMessage)
            .context("Failed to obtain the SDL2 event pump")?;

        while self.is_running {
            let current_time = Instant::now();
            let elapsed_time = current_time - last_time;
            last_time = current_time;
            lag += elapsed_time;

            self.fps_tracker.tick(elapsed_time);

            for event in event_pump.poll_iter() {
                if let Err(err) = self
                    .handle_event(event)
                    .and_then(|event| input::handle_event(self, event))
                {
                    self.is_running = false;
                    return Err(err);
                }
            }

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

            self.canvas.present();

            std::thread::yield_now();
        }

        Ok(())
    }

    /// Runs the context using the game state returned from the provided function.
    pub fn run_with<F, S>(&mut self, get_state: F) -> Result<()>
    where
        S: State,
        F: FnOnce(&mut Self) -> S,
    {
        let mut state = get_state(self);
        self.run(&mut state)
    }

    /// Runs the context using the game state returned from the provided (fallible) function.
    pub fn run_with_result<F, S>(&mut self, get_state: F) -> Result<()>
    where
        S: State,
        F: FnOnce(&mut Self) -> Result<S>,
    {
        let mut state = get_state(self)?;
        self.run(&mut state)
    }

    fn handle_event(&mut self, event: Event) -> Result<Event> {
        match event {
            Event::Quit { .. } => self.is_running = false,
            _ => {}
        }

        Ok(event)
    }
}

#[derive(Debug, Clone)]
pub struct ContextBuilder<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    vsync: bool,
    tick_rate: f64,
    fullscreen: bool,
    quit_on_escape: bool,
}

impl<'a> ContextBuilder<'a> {
    pub fn new(title: &'a str, width: u32, height: u32) -> Self {
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
        let sdl_context = sdl2::init()
            .map_err(Sdl2Error::ErrorMessage)
            .context("Failed to initialize SDL2 context")?;
        let video_subsystem = sdl_context
            .video()
            .map_err(Sdl2Error::ErrorMessage)
            .context("Failed to initialize SDL2 video subsystem")?;

        let window = video_subsystem
            .window(self.title, self.width, self.height)
            .position_centered()
            .build()
            .context("Failed to build SDL2 window")?;

        let canvas = window
            .into_canvas()
            .build()
            .context("Failed to build SDL2 canvas")?;

        Ok(Context {
            sdl_context,
            canvas,
            is_running: false,
            tick_rate: time::f64_to_duration(self.tick_rate),
            fps_tracker: FpsTracker::new(),
            world: World::new(),
            graphics: GraphicsContext::new(),
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
