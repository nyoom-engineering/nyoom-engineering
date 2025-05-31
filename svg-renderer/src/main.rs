use std::{env, error::Error, fs};
use resvg::{
    tiny_skia::{Color, Pixmap},
    usvg::{fontdb::Database, Options, Tree, Transform},
};

// default widths
const DEF_W: &[u32] = &[3840, 2560, 1920, 1280, 800];

fn widths() -> Vec<u32> {
    env::var("WIDTHS")
        .ok()
        .and_then(|s| {
            let v: Vec<u32> = s.split_whitespace().filter_map(|t| t.parse().ok()).collect();
            (!v.is_empty()).then_some(v)
        })
        .unwrap_or_else(|| DEF_W.to_vec())
}

fn main() -> Result<(), Box<dyn Error>> {
    // cli
    let mut a = env::args().skip(1);
    let input = a.next().ok_or("usage: svg <in> [out]")?;
    let out = a.next().unwrap_or_else(|| "output".into());

    // read + parse
    let data = fs::read(input)?;
    let mut db = Database::new();
    db.load_system_fonts();
    let tree = Tree::from_data(&data, &Options::default(), &db)?;

    // base size
    let (w0, h0) = {
        let s = tree.size();
        (s.width() as f32, s.height() as f32)
    };
    if w0 == 0.0 || h0 == 0.0 {
        return Err("zero size".into());
    }
    let asp = h0 / w0; // h / w

    // render loop
    for w in widths() {
        let h = ((w as f32) * asp).round() as u32;
        let mut pix = Pixmap::new(w, h).ok_or("pixmap")?;
        pix.fill(Color::from_rgba8(0, 0, 0, 255));
        let mut canvas = pix.as_mut();
        resvg::render(
            &tree,
            Transform::from_scale(w as f32 / w0, h as f32 / h0),
            &mut canvas,
        );
        pix.save_png(format!("{}-{}x{}.png", &out, w, h))?;
    }
    Ok(())
}