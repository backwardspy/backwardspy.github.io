use std::{collections::HashMap, fs, path::Path};

use color_eyre::Result;
use copy_dir::copy_dir;
use tera::{Context, Error, Tera, Value};

const SOURCES: &[&str] = &["ctp.j2", "index.j2", "rosetta.j2"];
const CRUFT: &[&str] = &["favicon.ico"];

#[derive(serde::Serialize)]
struct Inscription {
    content: &'static str,
    language: &'static str,
    author: &'static str,
    link: &'static str,
}

const fn inscriptions() -> [Inscription; 8] {
    macro_rules! inscription {
        ($lang:literal, $author:literal, $link:literal) => {
            Inscription {
                content: include_str!(concat!("../inscriptions/", $lang)),
                language: $lang,
                author: $author,
                link: $link,
            }
        };
    }
    [
        inscription!("austrian", "winston", "https://winston.sh"),
        inscription!("danish", "Nyx", "https://github.com/nyxkrage"),
        inscription!("english", "winston", "https://winston.sh"),
        inscription!("esperanto", "pigeon", "/"),
        inscription!("glaswegian", "hammy", "https://goudham.com"),
        inscription!("saxon", "justTOBBI", "https://justtobbi.is-a.dev"),
        inscription!("toki-pona", "pigeon", "/"),
        inscription!("welsh", "Name", "https://github.com/NamesCode"),
    ]
}

fn lines(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let value = value
        .as_str()
        .ok_or_else(|| Error::msg("expected string value"))?;
    let lines = value.lines().collect::<Vec<_>>();
    Ok(lines.into())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut tera = Tera::new("templates/**.j2")?;
    tera.register_filter("lines", Box::new(lines));
    let mut context = Context::new();
    context.insert("inscriptions", &inscriptions());

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
