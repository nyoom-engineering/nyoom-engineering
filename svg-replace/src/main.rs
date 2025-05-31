use clap::Parser;
use font_kit::{family_name::FamilyName as Fam, properties::Properties, source::SystemSource};
use fontdue::Font;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};
use xmltree::{Element, XMLNode};

#[derive(Parser)]
struct A {
    #[arg(num_args = 2)]
    w: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // load SF Pro Display font
    let f = Font::from_bytes(
        SystemSource::new()
            .select_best_match(&[Fam::Title("SF Pro Display".into())], &Properties::new())?
            .load()?
            .copy_font_data()
            .ok_or("font")?
            .as_ref()
            .to_vec(),
        Default::default(),
    )?;

    // kerning and overhang aware measurements
    const PX: f32 = 17.438;
    let glyph_width = |s: &str| -> f64 {
        let (mut pen, mut min, mut max, mut prev) = (0f32, 0f32, 0f32, None);
        for ch in s.chars() {
            if let Some(p) = prev {
                pen += f.horizontal_kern(p, ch, PX).unwrap_or(0.0);
            }
            let m = f.metrics(ch, PX);
            let l = pen + m.bounds.xmin as f32;
            let r = l + m.bounds.width as f32;
            min = min.min(l);
            max = max.max(r);
            pen += m.advance_width;
            prev = Some(ch);
        }
        (max - min) as f64
    };

    // load logo ... hardcoaded but whatever
    let mut svg = Element::parse(
        fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/logo.svg"))?.as_slice(),
    )?;
    let orig_w: f64 = svg.attributes["width"].trim_end_matches("px").parse()?;

    // calculate left margin
    fn first_x(e: &Element) -> Option<f64> {
        e.attributes
            .get("x")
            .and_then(|v| v.parse().ok())
            .or_else(|| {
                e.children.iter().find_map(|c| {
                    if let XMLNode::Element(el) = c {
                        first_x(el)
                    } else {
                        None
                    }
                })
            })
    }
    let left = first_x(&svg).unwrap_or(0.0);

    // calculate new canvas width
    let args = A::parse();
    let right_pad = orig_w - left - glyph_width("Engineering");
    let new_w = left + args.w.iter().map(|s| glyph_width(s)).fold(0.0, f64::max) + right_pad;

    // swap in text
    fn swap(e: &mut Element, v: &[String], n: &mut usize) {
        for c in e.children.iter_mut() {
            match c {
                XMLNode::Text(t) if *n < 2 && !t.trim().is_empty() => {
                    *t = v[*n].clone();
                    *n += 1;
                }
                XMLNode::Element(el) => swap(el, v, n),
                _ => {}
            }
            if *n == 2 {
                break;
            }
        }
    }
    swap(&mut svg, &args.w, &mut 0);

    // update width + viewBox
    svg.attributes.insert("width".into(), new_w.to_string());
    if let Some(vb) = svg.attributes.get_mut("viewBox") {
        let mut p: Vec<f64> = vb
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if p.len() == 4 {
            p[2] = new_w;
            *vb = p.iter().map(f64::to_string).collect::<Vec<_>>().join(" ");
        }
    }

    // write file
    let mut out = File::create("logo-custom.svg")?;
    svg.write(&mut out)?;
    out.flush()?;
    Ok(())
}
