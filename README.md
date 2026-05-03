# **cerulean**: A collection of 2,500+ beautiful color palettes for Rust 🎨

`cerulean` is a lightweight **collection of palettes** (e.g., lists of colors) for Rust.

Designing good color palettes is often harder than it looks: colors need to work well together, be accessible, match company branding, and more. Many people have developed palettes and made them freely available, especially in the R community.

`cerulean` makes those palettes accessible from Rust by providing a minimalist API for using them.

> [!NOTE]
> Similar projects exist for Python with [pypalettes](https://github.com/y-sunflower/pypalettes) and R with [paletteer](https://github.com/EmilHvitfeldt/paletteer).

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

- `load_palette()` returns a `Vec<String>` with all the colors in the given palette.
- `load_source()` returns the original source of the palette (for instance, an R package).
- `load_kind()` returns the palette kind. It can be `qualitative`, `quantitative`, or `sequential`.

<br>

## Browse all palettes

Since `cerulean` bundles 2,500+ different color palettes, we recommend using the [Color Palette Finder](https://python-graph-gallery.com/color-palette-finder/) to see which ones fit your needs best.

[![](https://github.com/y-sunflower/pypalettes/blob/main/pypalettes.gif?raw=true)](https://python-graph-gallery.com/color-palette-finder/)

<br>

## Installation

```bash
cargo add --git https://github.com/y-sunflower/cerulean
```

<br>

## Acknowledgements

`cerulean` is **heavily** inspired by, and relies on, the R package [paletteer](https://github.com/EmilHvitfeldt/paletteer) and all the original authors who created the palettes. See [LICENSE.note](LICENSE.note).

A big thank-you to [Yan Holtz](https://www.yan-holtz.com/) for creating the Color Palette Finder, a [web app for browsing palettes](https://python-graph-gallery.com/color-palette-finder/).
