use std::{env, fs, path::Path};

use resvg::{
    usvg::{fontdb::Database, Options, Tree},
    tiny_skia::{Color, Pixmap},
};

// resolutions
const DEFAULT_WIDTHS: &[u32] = &[3840, 2560, 1920, 1280, 800];

fn parse_widths_from_env() -> Vec<u32> {
    env::var("WIDTHS")
        .ok()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|tok| tok.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cli
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <input.svg> [output_basename]",
            args.get(0)
                .map(Path::new)
                .and_then(Path::file_name)
                .and_then(|s| s.to_str())
                .unwrap_or("svg_renderer")
        );
        std::process::exit(1);
    }
    let input_svg = &args[1];
    let output_base = args.get(2).map(String::as_str).unwrap_or("output");

    // svg -> usvg tree
    let svg_data = fs::read(input_svg)?;
    let opt = Options::default();

    // use system fonts
    let mut fontdb = Database::new();
    fontdb.load_system_fonts();

    let tree = Tree::from_data(&svg_data, &opt, &fontdb)?;

    // determine aspect ratio
    let size = tree.size();
    let (orig_w, orig_h) = (size.width() as f64, size.height() as f64);
    if orig_w == 0.0 || orig_h == 0.0 {
        return Err("SVG reports zero width/height; cannot compute aspect‑ratio".into());
    }
    let aspect = orig_h / orig_w;

    // render
    let widths = {
        let custom = parse_widths_from_env();
        if custom.is_empty() {
            DEFAULT_WIDTHS.to_vec()
        } else {
            custom
        }
    };
    for width in widths {
        let height = ((width as f64) * aspect).round() as u32;
        let mut pixmap = Pixmap::new(width, height).ok_or("Failed to create pixmap")?;
        pixmap.fill(Color::from_rgba8(0, 0, 0, 255)); // black background

        let mut canvas = pixmap.as_mut();
        let transform = resvg::usvg::Transform::from_scale(
            width as f32 / orig_w as f32,
            height as f32 / orig_h as f32,
        );
        resvg::render(&tree, transform, &mut canvas);

        pixmap.save_png(format!("{}-{}x{}.png", output_base, width, height))?;
    }

    println!("rendered SVG at {} resolution(s).", DEFAULT_WIDTHS.len());
    Ok(())
}