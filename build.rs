use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/palettes.json");

    let input = fs::read("src/palettes.json").expect("failed to read palettes.json");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let output = out_dir.join("palettes.json.gz");

    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(&input)
        .expect("failed to compress palettes.json");
    let compressed = encoder.finish().expect("failed to finish compression");

    fs::write(output, compressed).expect("failed to write compressed palettes");
}
