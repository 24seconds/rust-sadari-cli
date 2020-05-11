# 🍺 rust-sadari-cli

[![Crates.io](https://img.shields.io/crates/v/rust-sadari-cli.svg)](https://crates.io/crates/rust-sadari-cli)

`rust-sadari-cli` is sadari game (ghost leg in another words) based on terminal. Check more info [here!](https://en.wikipedia.org/wiki/Ghost_Leg)

> Generally, it is used to do lottery game to find random pairs of two sets.

Rust-sadari-cli draws most parts using [tui-rs](https://github.com/fdehau/tui-rs). Tui-rs is awesome library for making cli tools. (Rust-sadari-cli is also listed in [tui-rs' Apps using tui section!](https://github.com/fdehau/tui-rs#apps-using-tui))


### Demo
<img src="./assets/rust_sadari_demo.gif" width="800">

#### main page
<img src="./assets/rust_sadari_cli_demo_main.png" width="600">

#### result page
<img src="./assets/rust_sadari_cli_demo_result.png" width="600">

-------------

### 🎴 How to use?

#### Run with file path!
```
$ cargo run ./text.txt
```

In `text.txt` file... (two lines of texts separated by comma)
```
Iron man, Spider man, Thanos, Doctor strange, Captim america, Thor
    Soul,       Time, Space,            Mind,        Reality, Power
```

--------------

#### Just Run! rust-sadari will ask several questions to you!

```
$ cargo run
```

It will ask you some inputs like this!

```
Type list of names separated by comma! ex) name1, name2, name3 ...

	Q,q) Quit

$ type:

```
--------------

#### Installation

For rust users

```
$ cargo install rust-sadari-cli
```

