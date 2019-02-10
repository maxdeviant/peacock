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

## Motivation

![xkcd: Standards](https://imgs.xkcd.com/comics/standards.png)

> Fig 1: The state of Rust game engines.

With the abundance of 2D game engines for Rust it does seem a little silly that I would add yet another one into the mix, so I'll do my best to explain my motivations for writing Peacock.

Up until recently I was using [`ggez`](http://ggez.rs) for my games, but it got to the point where it was pretty frustrating to work with. The `0.4.0` branch had a lot of shortcomings, bugs, and APIs that were on the chopping block. This led me to switch to the `devel` branch, which cleaned up some stuff, but in turn had issues of its own. Now, in light of [The State of GGEZ 2019](https://wiki.alopex.li/TheStateOfGGEZ2019), I figured it would be a good time to search for greener pastures.

[Tetra](https://github.com/17cupsofcoffee/tetra) was the next engine to catch my eye, and at a glance it makes a lot of the same decisions I would for an engine. But having just come off `ggez` I found it hard to sell myself on an even more immature engine. Even the thought of getting in at the ground-floor and contributing to Tetra didn't quite appeal to me either.

This got me thinking about what it is that I really want in a Rust game engine. After some thought, I think what it really comes down to is control. I want to be able to build games the way I want to without worrying about hitching my cart to an engine that goes down a direction that doesn't align with my goals.
