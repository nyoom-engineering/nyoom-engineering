use serde_json::Value;
use std::{collections::BTreeMap, env, fs, io};

const CELL: i32 = 100;
const PAD: i32  = 24;
const COLS: i32 = 4;
const ROWS: i32 = 4;
const EXTRA_BOTTOM: i32 = 96;

/// embed logo at compile‑time. shouldn't be doing this but oh well!
const RAW_LOGO: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/logo.svg"));

fn main() -> io::Result<()> {
    // load palette
    let palette_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "oxocarbon-dark.json".into());

    let palette_json = fs::read_to_string(&palette_path)?;
    let palette: BTreeMap<String, Value> =
        serde_json::from_str(&palette_json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if palette.len() != 16 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("palette must contain exactly 16 colours, found {}", palette.len()),
        ));
    }

    // remove <?xml …?> and <!DOCTYPE …> so it can be nested
    let logo_clean: String = RAW_LOGO
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !(t.starts_with("<?xml") || t.starts_with("<!DOCTYPE"))
        })
        .collect::<Vec<_>>()
        .join("\n");

    // geometry
    let width  = CELL * COLS + PAD * 2;
    let height = CELL * ROWS + PAD * 2 + EXTRA_BOTTOM;
    let grid_bottom = PAD + ROWS * CELL;            // y after last square
    let gutter      = (height - grid_bottom) as f32;
    let native_h    = viewbox_height(&logo_clean);  // detected from viewBox
    let scale       = (gutter * 0.4) / native_h;   // fill 85 % of gutter
    let logo_h      = native_h * scale;
    let logo_y      = grid_bottom as f32 - 15.0 + (gutter - logo_h) / 2.0;

    // build our svg
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" \
              width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">\n",
        w = width,
        h = height
    );
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#000000\" />\n");

    // colour squares
    for (idx, (_name, value)) in palette.iter().enumerate() {
        let x = PAD + (idx as i32 % COLS) * CELL;
        let y = PAD + (idx as i32 / COLS) * CELL;
        let hex = value.as_str().unwrap_or("#000000");
        svg.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{c}\" height=\"{c}\" fill=\"{hex}\" />\n",
            x = x,
            y = y,
            c = CELL,
            hex = hex
        ));
    }

    // centred, scaled logo
    svg.push_str(&format!(
        "<g transform=\"translate({lx:.2},{ly:.2}) scale({s:.5})\">\n{logo}\n</g>\n",
        lx   = PAD as f32,   // left‑aligned to grid
        ly   = logo_y,       // centred vertically in gutter
        s    = scale,
        logo = logo_clean
    ));
    svg.push_str("</svg>");

    // write
    fs::write("palette.svg", svg)?;
    println!("palette.svg generated from '{}'", palette_path);
    Ok(())
}

/// extracts the *height* value from an SVG `viewBox="minX minY width height"`
fn viewbox_height(svg: &str) -> f32 {
    svg.find("viewBox")
        .and_then(|i| svg[i..].find('"').map(|off| i + off + 1))
        .and_then(|start| {
            let rest = &svg[start..];
            rest.find('"')
                .map(|end| &rest[..end])
        })
        .and_then(|vb| vb.split_whitespace().nth(3))
        .and_then(|h| h.parse::<f32>().ok())
        .unwrap_or(32.0)
}