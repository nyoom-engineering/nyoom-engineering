use serde_json::Value;
use std::{collections::BTreeMap, env, fs, io};

const CELL: i32 = 100;
const PAD: i32 = 24;
const COLS: i32 = 4;
const ROWS: i32 = 4;
const EXTRA: i32 = 96;
const RAW_LOGO: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/logo.svg"));

fn main() -> io::Result<()> {
    // palette
    let path = env::args().nth(1).unwrap_or_else(|| "oxocarbon-dark.json".into());
    let pal: BTreeMap<String, Value> =
        serde_json::from_str(&fs::read_to_string(&path)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    if pal.len() != 16 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "need 16 colours"));
    }

    // logo without xml / doctype
    let logo = RAW_LOGO
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !(t.starts_with("<?xml") || t.starts_with("<!DOCTYPE"))
        })
        .collect::<Vec<_>>()
        .join("\n");

    // geometry
    let (w, h) = (CELL * COLS + PAD * 2, CELL * ROWS + PAD * 2 + EXTRA);
    let grid_bot = PAD + ROWS * CELL;
    let gutter = (h - grid_bot) as f32;
    let scale = (gutter * 0.4) / 32.0;
    let logo_y = grid_bot as f32 - 15.0 + (gutter - 32.0 * scale) / 2.0;

    // svg header + bg
    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
<rect width="100%" height="100%" fill="#000"/>
"##,
        w = w,
        h = h
    );

    // colour squares
    for (i, v) in pal.values().enumerate() {
        let (x, y) = (PAD + (i as i32 % COLS) * CELL, PAD + (i as i32 / COLS) * CELL);
        svg.push_str(&format!(
            r#"<rect x="{x}" y="{y}" width="{c}" height="{c}" fill="{}"/>
"#,
            v.as_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "bad hex"))?,
            x = x,
            y = y,
            c = CELL
        ));
    }

    // scaled logo + footer
    svg.push_str(&format!(
        r#"<g transform="translate({lx},{ly}) scale({s})">
{logo}
</g>
</svg>"#,
        lx = PAD,
        ly = logo_y,
        s = scale,
        logo = logo
    ));

    fs::write("palette.svg", svg)?;
    println!("palette.svg generated from '{path}'");
    Ok(())
}