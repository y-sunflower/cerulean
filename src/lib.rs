use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize)]
struct Palette {
    name: String,
    #[serde(deserialize_with = "deserialize_palette")]
    palette: Vec<String>,
    source: String,
    kind: Option<String>,
}

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
    let data = include_str!("palettes.json");
    let palettes_vec: Vec<Palette> = serde_json::from_str(data)?;

    let map: HashMap<String, Palette> = palettes_vec
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect();

    Ok(map)
}

pub fn load_palette(name: &str) -> Vec<String> {
    let palettes: HashMap<String, Palette> = load_palettes().expect("Error reading palettes");
    palettes
        .get(name)
        .expect("Palette not found")
        .palette
        .clone()
}

pub fn load_source(name: &str) -> String {
    let palettes: HashMap<String, Palette> = load_palettes().expect("Error reading palettes");
    palettes
        .get(name)
        .expect("Palette not found")
        .source
        .clone()
}

pub fn load_kind(name: &str) -> String {
    let palettes: HashMap<String, Palette> = load_palettes().expect("Error reading palettes");
    palettes
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
