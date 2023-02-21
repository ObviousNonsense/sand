# Sand

Port/new version of a falling sand sim [I first started working on in
Javascript](https://github.com/ObviousNonsense/FallingSand), now written in Rust.

[You can play with the web version here](ObviousNonsense.github.io/sand/)

Or clone the repo and build it yourself, assuming you have installed cargo (through rustup), just run:

```
cargo run --release
```

Rendering and interaction is performed using [macroquad](https://github.com/not-fl3/macroquad). The
UI is made with [egui](https://github.com/emilk/egui) via
[egui-macroquad](https://github.com/optozorax/egui-macroquad).

## Inspiration
Inspiration is drawn from various similar falling sand sims, including Powder Toy and Noita. The
[Exploring the Tech and Design of Noita](https://www.youtube.com/watch?v=prXuyMCgbTc) talk from GDC
2019 is very helpful, as well as [this video](https://www.youtube.com/watch?v=5Ka3tbbT-9E) which
explains how one might implement some of these techniques.
[Sandspiel](https://github.com/MaxBittker/sandspiel) also provided some insight into how to make
this work in Rust specifically (I've tried to avoid looking too hard at it so I could figure some
things out on my own, but some parts of the structure of the code are *heavily* inspired by it).
