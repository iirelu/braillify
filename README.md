```
⠀⠀⣠⠴⠒⠒⠲⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣄⠀⠀⣀⣤⠀⢀⣀⡄⠀⢀⣤⡀⠀⢀⣴⣶⢶⡄⠀⠀⠀⠀⠀⠀
⢠⡿⠁⣸⣿⠇⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⠟⠀⢸⣿⡇⠀⢸⣿⡇⠀⠘⠿⠃⠀⣼⣿⠃⠀⠁⠀⠀⠀⠀⠀⠀
⠸⠇⠀⣿⣿⣤⣾⣏⠀⠀⣴⣶⡦⢶⣶⠀⣴⣶⠟⢶⣶⡆⠀⣰⣶⠆⠀⣾⣿⠁⠀⣿⣿⠀⠀⣶⣶⠀⢠⣿⡿⠒⢰⣶⡖⠀⣴⣶⠂⠀
⠀⠀⢸⣿⡇⠀⠘⣿⣧⢀⣿⡿⠀⠈⠁⣾⣿⠁⠀⣾⣿⠁⠀⣿⣿⠀⢠⣿⡟⠀⢰⣿⡏⠀⢸⣿⡇⠀⣸⣿⡇⠀⣼⣿⠃⢀⣿⡿⠀⠀
⠀⠀⣾⣿⠁⠀⢠⣿⡏⢸⣿⡇⠀⠀⢰⣿⣟⠀⢠⣿⡟⠀⣸⣿⡇⠀⣼⣿⡇⠀⣼⣿⠃⢀⣿⣿⠁⢀⣿⣿⠀⢀⣿⣿⠀⣸⣿⡇⢀⠄
⠀⠠⠿⠟⠸⣶⡿⠟⠀⠿⠿⠀⠀⠀⠀⠻⠿⠶⠋⠿⠿⠞⠙⠿⠷⠞⠹⠿⠿⠋⠹⠿⠿⠋⠻⠿⠷⣻⣿⡏⠀⠈⠿⠿⢞⣿⣿⠴⠋⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⠉⣿⣿⠁⠀⠀⢠⠞⢹⣿⡇⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⠟⠃⠀⠀⠀⠘⠳⠿⠋⠀⠀⠀⠀
```
[![Build Status](https://travis-ci.org/iirelu/braillify.svg?branch=master)](https://travis-ci.org/iirelu/braillify)

A tool for converting images into braille!

## Setup

* [Install Rust](http://www.rust-lang.org/install.html)
* `cargo run [program options]` or `cargo build; target/braille [program options]`

Isn't Rust nice?

## Usage

```
    braillify [OPTIONS] <file>

FLAGS:
        --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -h, --height <height>
            Specifies the height of the rendered braille image. If --width is not also
            specified, it is guessed from aspect ratio.
    -w, --width <width>
            Specifies the width of the rendered braille image. If --height is not also
            specified, it is guessed from aspect ratio.

ARGS:
    <file>
            Image file to convert
```

## Examples

`braillify tests/rust-logo.png -w40`:

```
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣤⡀⢀⣴⣆⠀⢀⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠀⠀⣿⣷⣤⣿⣿⣷⣾⣿⣿⣷⣿⣿⣧⣴⣾⣇⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣾⣿⣿⣿⣿⣿⣿⣏⠀⢈⣿⣿⣿⣿⣿⣿⣿⣶⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣿⣷⣾⣿⣿⣿⡿⠛⠉⠁⠀⠈⠻⣿⡿⠋⠀⠀⠉⠙⠻⢿⣿⣿⣿⣶⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢠⣤⣤⣾⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣧⣤⣤⠀⠀⠀⠀
⠀⠀⠀⠀⣻⣿⣿⣿⣿⣷⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣤⣀⠀⠀⠘⢿⣿⣿⣿⡇⠀⠀⠀⠀
⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⢀⣿⣿⣿⣿⣿⠇⠀⠀
⠀⢀⣠⣿⣿⠋⠙⣿⡏⠉⢻⣿⣿⣿⣿⣿⡟⠛⠛⠛⠛⠛⠻⣿⣿⣿⣿⣿⣿⡇⠀⣼⡟⠉⢻⣿⣯⣀⡀⠀
⠀⠻⣿⣿⣿⣶⣶⠿⠋⠀⢸⣿⣿⣿⣿⣿⣇⣀⣀⣀⣀⣀⣤⣿⣿⣿⣿⣿⡿⠁⠈⠻⢷⣴⣾⣿⣿⡿⠋⠀
⢀⣤⣾⣿⣿⡏⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡋⠀⠀⠀⠀⠀⠈⣿⣿⣿⣧⣄⠀
⠈⠛⢿⣿⣿⣇⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡿⠿⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⢠⣤⣿⣿⣿⡟⠋⠀
⠀⣴⣿⣿⣿⣿⡀⠀⠀⠀⣸⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⣀⠀⣀⣾⣿⣿⣿⣿⣷⣄⠀
⠀⠈⠙⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣟⠉⠁⠀
⠀⠀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀
⠀⠀⠀⠀⣽⣿⣿⣿⣯⣉⣉⣉⣉⠉⠉⠉⠉⠉⠉⠉⠀⠀⠀⠀⠀⠉⢉⣉⣉⡉⣩⣿⣿⣿⣿⡇⠀⠀⠀⠀
⠀⠀⠀⠘⠛⠛⢿⣿⣿⣿⠟⠛⢿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠟⠛⢿⣿⣿⣿⡟⠛⠛⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣿⡿⢿⣿⣦⣤⣾⣷⣤⣀⡀⠀⠀⠀⠀⠀⠀⢀⣀⣴⣿⣦⣤⣾⣿⠿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⣿⡿⠛⣿⣿⡿⢿⣿⣿⡿⣿⣿⡟⠻⢿⡏⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠛⠁⠈⠻⠏⠀⠈⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
```

Note: If whatever fonts your browser use don't support Braille patterns, you're not going to see much there. If that's the case, [here's a screenshot](http://i.cubeupload.com/tpE1DF.png).

## Why?

Why not? But really, this project has taught me a *lot* about Rust, so I'm probably going to go on to do more useful stuff in the future. The code is a mess, as I didn't know any Rust before I started, but it's good enough for my standards.

## How?

Now that's a more interesting question! One day I noticed [the way Unicode handles Braille patterns](https://en.wikipedia.org/wiki/Braille_Patterns) is actually (almost) logical, so I decided "Hey, why not do something with that?" So I did! And I ended up making [this neat Braille animation](http://gfycat.com/EarnestColorlessLacewing). Oh, and then I made this, of course.

All 256 possible combinations of the 8 dots in a Braille character are mapped out in a way that makes it very easy to generate the character for any combination of dots you want. You can see the code that does it [here](https://github.com/iirelu/braillify/blob/master/src/braille.rs).
