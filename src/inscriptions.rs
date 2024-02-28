#[derive(serde::Serialize)]
pub(crate) struct Inscription {
    content: &'static str,
    language: &'static str,
    author: &'static str,
    link: &'static str,
}

pub(crate) const fn all() -> [Inscription; 8] {
    macro_rules! inscription {
        ($slug:literal, $lang:literal, $author:literal, $link:literal) => {
            Inscription {
                content: include_str!(concat!("../inscriptions/", $slug)),
                language: $lang,
                author: $author,
                link: $link,
            }
        };
    }
    [
        inscription!("austrian", "Austrian", "winston", "https://winston.sh"),
        inscription!("danish", "Danish", "Nyx", "https://github.com/nyxkrage"),
        inscription!("english", "English", "winston", "https://winston.sh"),
        inscription!("esperanto", "Esperanto", "pigeon", "/"),
        inscription!("glaswegian", "Glaswegian", "hammy", "https://goudham.com"),
        inscription!("saxon", "Saxon", "justTOBBI", "https://justtobbi.is-a.dev"),
        inscription!("toki-pona", "toki pona", "pigeon", "/"),
        inscription!("welsh", "Welsh", "Name", "https://github.com/NamesCode"),
    ]
}
