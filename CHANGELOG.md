# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added animation support with `Animation`
- Added all of the [X11 web colors](https://en.wikipedia.org/wiki/Web_colors#X11_color_names) as constants on `Color`
- Added `graphics::clear` for clearing the screen
- Added `Drawable` trait for objects that can be drawn to the screen
- Added `graphics::draw` for drawing `Drawable`s to the screen
- Added `graphics::draw_sprite` for drawing a sprite to the screen
- Added `graphics::draw_text` for drawing text to the screen
- Added `window::set_title` for setting the window title
- Added `window::set_view` for setting the view on the window

### Changed

- `Context.window` and `Context.fps_tracker` are no longer visible outside of the crate
- Replaced SFML `Color` with custom `Color`

### Fixed

- Fixed sprite batching by removing some hard-coded values

## 0.0.1 - 2019-02-09

### Added

- Added base game constructs
  - `State`
  - `Context`
  - `SpriteBatch`

[unreleased]: https://github.com/maxdeviant/peacock/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/maxdeviant/peacock/compare/2e44af3...v0.0.1
