# Peacock

[![Crates.io](https://img.shields.io/crates/v/peacock.svg)](https://crates.io/crates/peacock)
[![Docs.rs](https://docs.rs/peacock/badge.svg)](https://docs.rs/peacock/)
[![Crates.io](https://img.shields.io/crates/l/peacock.svg)](https://github.com/maxdeviant/peacock/blob/master/LICENSE)
[![Travis](https://img.shields.io/travis/maxdeviant/peacock.svg?style=flat)](https://travis-ci.org/maxdeviant/peacock)

Peacock is a game engine for making beautiful games.

#### 🚧 UNDER CONSTRUCTION 🚧

Peacock is still very much a work-in-progress. You're welcome to use it and report any issues you find, but I make no promises about stability or supporting your specific use cases until we hit `1.0.0`.

## Installation

To install Peacock, add the following to your `Cargo.toml`:

```toml
[dependencies]
peacock = "0.0.1"
```

If you're like me and enjoy living on the bleeding edge, you can use a `git` dependency:

```toml
[dependencies]
peacock = { git = "https://github.com/maxdeviant/peacock" }
```

> Since we're still in the early days, I would suggest opting for the `git` dependency.

### Dependencies

Peacock is built on top of [SFML](https://www.sfml-dev.org/), so you will need to install the SFML libraries for your platform.

If you're running an Ubuntu-based Linux distribution you can (most likely) do the following:

```sh
sudo apt install lib-sfmldev
sudo apt install lib-csfmldev
```

For other platforms check out the [`rust-sfml`](https://github.com/jeremyletang/rust-sfml) docs.
