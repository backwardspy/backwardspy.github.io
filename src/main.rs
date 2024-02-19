use std::{collections::HashMap, fmt::Write, fs, path::Path};

use color_eyre::Result;
use copy_dir::copy_dir;
use tera::{Context, Error, Tera, Value};

const SOURCES: &[&str] = &["ctp.j2", "index.j2", "rosetta.j2"];
const CRUFT: &[&str] = &["favicon.ico"];

fn inscription(args: &HashMap<String, Value>) -> Result<Value, Error> {
    let author = args
        .get("author")
        .ok_or(Error::msg("expected author parameter"))?
        .as_str()
        .ok_or(Error::msg("expected author to be a string"))?;
    let lang = args
        .get("lang")
        .ok_or(Error::msg("expected lang parameter"))?
        .as_str()
        .ok_or(Error::msg("expected lang to be a string"))?;
    let link = args
        .get("link")
        .ok_or(Error::msg("expected link parameter"))?
        .as_str()
        .ok_or(Error::msg("expected link to be a string"))?;

    let path = Path::new("inscriptions").join(lang);
    let content = fs::read_to_string(path)?
        .lines()
        .fold(String::new(), |mut acc, line| {
            let _ = write!(acc, "<p>{line}</p>");
            acc
        });

    let lang = lang.replace('-', " ");

    let element = format!(
        r#"<div class="inscription">
    {content}
    <p class="credit"><small>{lang} translation by <a href="{link}">{author}</a></small></p>
</div>"#
    );

    Ok(Value::String(element))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut tera = Tera::new("templates/**.j2")?;
    tera.register_function("inscription", Box::new(inscription));
    let context = Context::new();

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
    fs::remove_dir_all(res_dir)?;
    copy_dir("res", res_dir)?;

    println!("copying cruft");
    for cruft in CRUFT {
        let dest = out_dir.join(cruft);
        println!(" - {}", dest.display());
        fs::copy(cruft, dest)?;
    }

    println!("done!");

    Ok(())
}
