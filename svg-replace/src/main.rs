use clap::Parser;
use std::{error::Error, fs::{self, File}, io::Write, path::Path};
use xmltree::{Element, XMLNode};

#[derive(Parser)]
struct Args { #[arg(num_args=2)] words: Vec<String> }

fn main() -> Result<(), Box<dyn Error>> {
    let w = Args::parse().words;
    let mut svg = Element::parse(
        fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/logo.svg"))?.as_slice()
    )?;

    // replace first two non‑blank text nodes
    fn walk(e:&mut Element,w:&[String],i:&mut usize){
        for c in e.children.iter_mut() {
            match c {
                XMLNode::Text(t) if *i<2 && !t.trim().is_empty() => {*t=w[*i].clone();*i+=1;}
                XMLNode::Element(el) => walk(el,w,i),
                _=>{}
            }
            if *i==2 {break}
        }
    }
    walk(&mut svg,&w,&mut 0);

    // scale width & viewBox
    if let Some(w_attr)=svg.attributes.get("width").cloned(){
        let k=w.iter().map(|s|s.chars().count()).max().unwrap_or(11) as f64/11.0;
        let new_w=(w_attr.trim_end_matches("px").parse::<f64>()?*k).ceil();
        svg.attributes.insert("width".into(),new_w.to_string());
        if let Some(vb)=svg.attributes.get_mut("viewBox"){
            let mut p:Vec<f64>=vb.split_whitespace().filter_map(|s|s.parse().ok()).collect();
            if p.len()==4{p[2]=new_w;*vb=p.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ")}
        }
    }

    // output
    let mut f=File::create("logo-custom.svg")?;
    svg.write(&mut f)?; f.flush()?;
    Ok(())
}