//! Access 2,500+ curated color palettes from Rust.
//!
//! `cerulean` bundles palette names, colors, sources, and palette kinds in the
//! crate, so palettes can be loaded without any runtime file or network access.
//!
//! # Examples
//!
//! ```
//! use cerulean::{load_kind, load_palette, load_source};
//!
//! let palette = load_palette("Acadia");
//! let source = load_source("Acadia");
//! let kind = load_kind("Acadia");
//!
//! assert_eq!(palette.len(), 6);
//! assert_eq!(source, "The R package: {nationalparkcolors}");
//! assert_eq!(kind, "qualitative");
//! ```

use flate2::read::GzDecoder;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct Palette {
    name: String,
    #[serde(deserialize_with = "deserialize_palette")]
    palette: Vec<String>,
    source: String,
    kind: Option<String>,
}

// avoid decompress-parse-build every time we need palette
static PALETTES: OnceLock<HashMap<String, Palette>> = OnceLock::new();

fn deserialize_palette<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum PaletteValue {
        Colors(Vec<String>),
        String(String),
    }

    match PaletteValue::deserialize(deserializer)? {
        PaletteValue::Colors(colors) => Ok(colors),
        PaletteValue::String(colors) => {
            let json = colors.replace('\'', "\"");
            serde_json::from_str(&json).map_err(serde::de::Error::custom)
        }
    }
}

fn load_palettes() -> Result<HashMap<String, Palette>, Box<dyn Error>> {
    let compressed = include_bytes!("palettes.json.gz");
    let mut decoder = GzDecoder::new(&compressed[..]);
    let mut data = String::new();
    decoder.read_to_string(&mut data)?;
    let palettes_vec: Vec<Palette> = serde_json::from_str(&data)?;

    let map: HashMap<String, Palette> = palettes_vec
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect();

    Ok(map)
}

fn palette_map() -> &'static HashMap<String, Palette> {
    PALETTES.get_or_init(|| load_palettes().expect("Error reading palettes"))
}

/// Loads the colors for a palette by name.
///
/// Returns the colors as hexadecimal strings in their original order.
///
/// # Panics
///
/// Panics if the embedded palette data cannot be read or if `name` does not
/// match a bundled palette.
///
/// # Examples
///
/// ```
/// let palette = cerulean::load_palette("Acadia");
///
/// assert_eq!(palette[0], "#FED789FF");
/// ```
pub fn load_palette(name: &str) -> Vec<String> {
    palette_map()
        .get(name)
        .expect("Palette not found")
        .palette
        .clone()
}

/// Loads the original source for a palette by name.
///
/// Sources usually identify the package, project, or author that provided the
/// palette.
///
/// # Panics
///
/// Panics if the embedded palette data cannot be read or if `name` does not
/// match a bundled palette.
///
/// # Examples
///
/// ```
/// let source = cerulean::load_source("Acadia");
///
/// assert_eq!(source, "The R package: {nationalparkcolors}");
/// ```
pub fn load_source(name: &str) -> String {
    palette_map()
        .get(name)
        .expect("Palette not found")
        .source
        .clone()
}

/// Loads the kind of a palette by name.
///
/// Palette kinds describe how the colors are intended to be used, such as
/// `qualitative`, `quantitative`, or `sequential`. Returns `"unknown"` when a
/// bundled palette does not define a kind.
///
/// # Panics
///
/// Panics if the embedded palette data cannot be read or if `name` does not
/// match a bundled palette.
///
/// # Examples
///
/// ```
/// let kind = cerulean::load_kind("Acadia");
///
/// assert_eq!(kind, "qualitative");
/// ```
pub fn load_kind(name: &str) -> String {
    palette_map()
        .get(name)
        .expect("Palette not found")
        .kind
        .clone()
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use crate::{load_kind, load_palette};

    #[test]
    fn load_a_specific_palette() {
        let palette = load_palette("Acadia");
        let expected = vec![
            "#FED789FF".to_string(),
            "#023743FF".to_string(),
            "#72874EFF".to_string(),
            "#476F84FF".to_string(),
            "#A4BED5FF".to_string(),
            "#453947FF".to_string(),
        ];
        assert_eq!(palette, expected)
    }

    #[test]
    fn load_a_string_encoded_palette() {
        let palette = load_palette("Wanteeed");
        let expected = vec!["#10345c".to_string(), "#ffacac".to_string()];
        assert_eq!(palette, expected)
    }

    #[test]
    fn load_unknown_kind_when_kind_is_null() {
        assert_eq!(load_kind("Accent"), "unknown")
    }
}
