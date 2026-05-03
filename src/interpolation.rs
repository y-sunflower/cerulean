use std::num::ParseIntError;

use ndarray::Array;

fn hex_to_rgb(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    let hex: &str = hex.strip_prefix("#").unwrap_or(hex);

    let r: u8 = u8::from_str_radix(&hex[0..2], 16)?;
    let g: u8 = u8::from_str_radix(&hex[2..4], 16)?;
    let b: u8 = u8::from_str_radix(&hex[4..6], 16)?;

    return Ok(vec![r, g, b]);
}

fn interpolation_2_colors(
    color_1: &String,
    color_2: &String,
    n: usize,
) -> Result<Vec<String>, String> {
    let color_1_rgb = hex_to_rgb(color_1)
        .map_err(|e| format!("Error when converting Hex ({}) to RGB: {}", color_1, e))?;

    let color_2_rgb = hex_to_rgb(color_2)
        .map_err(|e| format!("Error when converting Hex ({}) to RGB: {}", color_2, e))?;

    let c1: Vec<f64> = color_1_rgb.iter().map(|&x| x as f64).collect();
    let c2: Vec<f64> = color_2_rgb.iter().map(|&x| x as f64).collect();

    let ts = Array::linspace(0.0, 1.0, n);
    let colors: Vec<[u8; 3]> = ts
        .iter()
        .map(|&t| {
            let r = c1[0] + (c2[0] - c1[0]) * t;
            let g = c1[1] + (c2[1] - c1[1]) * t;
            let b = c1[2] + (c2[2] - c1[2]) * t;
            [r.round() as u8, g.round() as u8, b.round() as u8]
        })
        .collect();

    return Ok(Vec::new());
}
