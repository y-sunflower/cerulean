# **cerulean**: Collection of 2500+ beautiful color palettes for Rust 🎨

<br>

## Quick start

```rs
use cerulean::{load_kind, load_palette, load_source};

fn main() {
    let palette = load_palette("Acadia");
    let source = load_source("Acadia");
    let kind = load_kind("Acadia");

    println!("palette: {:?}", palette);
    println!("source: '{source}'");
    println!("kind: '{kind}'");
}
```

```py
#> palette: ["#FED789FF", "#023743FF", "#72874EFF", "#476F84FF", "#A4BED5FF", "#453947FF"]
#> source: 'The R package: {nationalparkcolors}'
#> kind: 'qualitative'
```

- `load_palette()` returns a `Vec<String>` with all the colors
- `load_source()` returns the original source of the palette (here it's an R package for instance)
- `load_kind()` returns the kind palette. Could be one of qualitative, quantitative, sequential

<br>

## Browse all the palettes

Since `cerulean` bundles 2500+ different color palettes, it's recommended to use the [Color Palette Finder](https://python-graph-gallery.com/color-palette-finder/) to easily see which one fit your needs best.

[![](https://github.com/y-sunflower/pypalettes/blob/main/pypalettes.gif?raw=true)](https://python-graph-gallery.com/color-palette-finder/)

<br>

## Installation

```bash
cargo add --git https://github.com/y-sunflower/cerulean
```

<br>

## Acknowledgements

`PyPalettes` is **highly** inspired by—and relies on—the R package [paletteer](https://github.com/EmilHvitfeldt/paletteer), and all the original authors who created the palettes. See [LICENSE.note](LICENSE.note).

A big thanks to [Yan Holtz](https://www.yan-holtz.com/) for creating the Color Palette Finder, a [web app for browsing palettes](https://python-graph-gallery.com/color-palette-finder/).
