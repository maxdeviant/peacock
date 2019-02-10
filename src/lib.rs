//! Peacock is a game engine for making beautiful games.
//!
//! [![Crates.io](https://img.shields.io/crates/v/peacock.svg)](https://crates.io/crates/peacock)
//! [![Docs.rs](https://docs.rs/peacock/badge.svg)](https://docs.rs/peacock/)
//! [![Crates.io](https://img.shields.io/crates/l/peacock.svg)](https://github.com/maxdeviant/peacock/blob/master/LICENSE)
//! [![Travis](https://img.shields.io/travis/maxdeviant/peacock.svg?style=flat)](https://travis-ci.org/maxdeviant/peacock)
//!
//! ## Installation
//! ```toml
//! [dependencies]
//! peacock = "0.0.1"
//! ```

pub mod error;
pub mod graphics;
pub mod time;
pub mod window;

mod context;

pub use sfml::system::{Vector2f, Vector2i, Vector2u, Vector3f, Vector3i};

pub use crate::context::*;
pub use crate::error::Result;

pub trait State {
    fn update(&mut self, ctx: &mut Context) -> Result<()>;
    fn draw(&mut self, ctx: &mut Context, dt: f64) -> Result<()>;
}
