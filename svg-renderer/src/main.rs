use resvg::{
    tiny_skia::{Color, Pixmap},
    usvg::{fontdb::Database, Options, Transform, Tree},
};
use std::{env, error::Error, fs};

// default widths, including lower resolutions
const DEF_W: &[u32] = &[3840, 2560, 1920, 1280, 800, 400, 200, 100];

fn widths() -> Vec<u32> {
    env::var("WIDTHS")
        .ok()
        .and_then(|s| {
            let v: Vec<u32> = s
                .split_whitespace()
                .filter_map(|t| t.parse().ok())
                .collect();
            (!v.is_empty()).then_some(v)
        })
        .unwrap_or_else(|| DEF_W.to_vec())
}

fn main() -> Result<(), Box<dyn Error>> {
    // CLI args: input SVG path, optional output prefix
    let mut args = env::args().skip(1);
    let input_path = args.next().ok_or("usage: svg <in> [out]")?;
    let out_prefix = args.next().unwrap_or_else(|| "output".into());

    // read SVG data and parse
    let data = fs::read(&input_path)?;
    let mut fontdb = Database::new();
    fontdb.load_system_fonts();
    let tree = Tree::from_data(&data, &Options::default(), &fontdb)?;

    // original SVG dimensions
    let (w0, h0) = {
        let size = tree.size();
        (size.width(), size.height())
    };
    if w0 == 0.0 || h0 == 0.0 {
        return Err("SVG has zero size".into());
    }
    let aspect = h0 / w0;

    for w in widths() {
        // calculate height for this width
        let h = ((w as f32) * aspect).round() as u32;

        // 1) Render opaque version (black background)
        let mut pixmap_opaque = Pixmap::new(w, h).ok_or("failed to create pixmap")?;
        pixmap_opaque.fill(Color::from_rgba8(0, 0, 0, 255));
        let mut canvas_opaque = pixmap_opaque.as_mut(); // made mutable
        resvg::render(
            &tree,
            Transform::from_scale(w as f32 / w0, h as f32 / h0),
            &mut canvas_opaque,
        );
        pixmap_opaque.save_png(format!("{}-{}x{}.png", out_prefix, w, h))?;

        // 2) Render transparent version (no background)
        let mut pixmap_trans = Pixmap::new(w, h).ok_or("failed to create pixmap")?;
        pixmap_trans.fill(Color::from_rgba8(0, 0, 0, 0));
        let mut canvas_trans = pixmap_trans.as_mut(); // made mutable
        resvg::render(
            &tree,
            Transform::from_scale(w as f32 / w0, h as f32 / h0),
            &mut canvas_trans,
        );
        pixmap_trans.save_png(format!("{}-{}x{}-transparent.png", out_prefix, w, h))?;
    }

    Ok(())
}
