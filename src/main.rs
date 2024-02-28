use std::{fs, path::Path};

use color_eyre::Result;
use copy_dir::copy_dir;
use tera::{Context, Tera};

mod filters;
mod inscriptions;

const SOURCES: &[&str] = &["ctp.j2", "index.j2", "rosetta.j2"];
const CRUFT: &[&str] = &["favicon.ico"];

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut tera = Tera::new("templates/**.j2")?;
    tera.register_filter("lines", Box::new(filters::lines));
    let mut context = Context::new();
    context.insert("inscriptions", &inscriptions::all());

    let out_dir = Path::new("dist");

    println!("creating {}", out_dir.display());
    fs::create_dir_all(out_dir)?;

    println!("rendering sources");
    for source in SOURCES {
        let dest = out_dir.join(source).with_extension("html");
        println!(" - {} → {}", source, dest.display());
        fs::write(dest, tera.render(source, &context)?)?;
    }

    let res_dir = &out_dir.join("res");
    println!("replacing {}", res_dir.display());
    if res_dir.exists() {
        fs::remove_dir_all(res_dir)?;
    }
    copy_dir("res", res_dir)?;

    println!("copying cruft");
    for cruft in CRUFT {
        let dest = out_dir.join(cruft);
        println!(" - {} → {}", cruft, dest.display());
        fs::copy(cruft, dest)?;
    }

    println!("done!");

    Ok(())
}
