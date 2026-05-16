#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    has_alpha: bool,
}

impl Color {
    fn from_hex(hex: &str) -> Result<Self, String> {
        let hex_digits = hex.strip_prefix('#').unwrap_or(hex);

        if hex_digits.len() != 6 && hex_digits.len() != 8 {
            return Err(format!("expected a 6- or 8-digit hex color, got `{}`", hex));
        }

        let channel = |range: std::ops::Range<usize>| {
            u8::from_str_radix(&hex_digits[range], 16)
                .map_err(|error| format!("invalid hex color `{}`: {}", hex, error))
        };

        Ok(Self {
            r: channel(0..2)?,
            g: channel(2..4)?,
            b: channel(4..6)?,
            a: if hex_digits.len() == 8 {
                channel(6..8)?
            } else {
                255
            },
            has_alpha: hex_digits.len() == 8,
        })
    }

    fn interpolate(start: Self, end: Self, t: f64) -> Self {
        Self {
            r: interpolate_channel(start.r, end.r, t),
            g: interpolate_channel(start.g, end.g, t),
            b: interpolate_channel(start.b, end.b, t),
            a: interpolate_channel(start.a, end.a, t),
            has_alpha: start.has_alpha || end.has_alpha,
        }
    }

    fn to_hex(self, include_alpha: bool) -> String {
        if include_alpha {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        } else {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        }
    }
}

fn interpolate_channel(start: u8, end: u8, t: f64) -> u8 {
    (start as f64 + (end as f64 - start as f64) * t)
        .round()
        .clamp(0.0, 255.0) as u8
}

pub(crate) fn interpolate_palette(colors: &[String], n: usize) -> Result<Vec<String>, String> {
    if n == 0 || colors.is_empty() {
        return Ok(Vec::new());
    }

    if n == 1 {
        return Ok(vec![colors[0].clone()]);
    }

    if n == colors.len() {
        return Ok(colors.to_vec());
    }

    if colors.len() == 1 {
        return Ok(vec![colors[0].clone(); n]);
    }

    let parsed_colors = colors
        .iter()
        .map(|color| Color::from_hex(color))
        .collect::<Result<Vec<_>, _>>()?;
    let include_alpha = parsed_colors.iter().any(|color| color.has_alpha);
    let palette_span = parsed_colors.len() - 1;
    let colormap_span = n - 1;

    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        if i == colormap_span {
            result.push(parsed_colors[palette_span].to_hex(include_alpha));
            continue;
        }

        let position = i as f64 * palette_span as f64 / colormap_span as f64;
        let start_index = position.floor() as usize;
        let end_index = start_index + 1;
        let segment_t = position - start_index as f64;
        let color = Color::interpolate(
            parsed_colors[start_index],
            parsed_colors[end_index],
            segment_t,
        );

        result.push(color.to_hex(include_alpha));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::interpolation::interpolate_palette;

    #[test]
    fn interpolates_between_two_rgb_colors() {
        let colors = vec!["#000000".to_string(), "#FFFFFF".to_string()];

        assert_eq!(
            interpolate_palette(&colors, 3).unwrap(),
            vec!["#000000", "#808080", "#FFFFFF"]
        );
    }

    #[test]
    fn interpolates_alpha_when_present() {
        let colors = vec!["#00000080".to_string(), "#FFFFFF00".to_string()];

        assert_eq!(
            interpolate_palette(&colors, 3).unwrap(),
            vec!["#00000080", "#80808040", "#FFFFFF00"]
        );
    }

    #[test]
    fn preserves_palette_when_requested_size_matches() {
        let colors = vec!["#10345c".to_string(), "#ffacac".to_string()];

        assert_eq!(interpolate_palette(&colors, 2).unwrap(), colors);
    }

    #[test]
    fn one_color_request_returns_first_palette_color() {
        let colors = vec!["#000000".to_string(), "#FFFFFF".to_string()];

        assert_eq!(interpolate_palette(&colors, 1).unwrap(), vec!["#000000"]);
    }

    #[test]
    fn returns_requested_number_of_colors_across_full_palette() {
        let colors = vec![
            "#000000".to_string(),
            "#FF0000".to_string(),
            "#FFFFFF".to_string(),
        ];

        assert_eq!(
            interpolate_palette(&colors, 5).unwrap(),
            vec!["#000000", "#800000", "#FF0000", "#FF8080", "#FFFFFF"]
        );
    }
}
