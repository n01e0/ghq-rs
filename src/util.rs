extern crate url;

use url::{Url, ParseError};
use git2::{Config};
use std::path::PathBuf;

pub fn url2path(url: &str) -> Result<String, ParseError> {
    match Url::parse(url) {
        Ok(url) => {
            let mut path = ghqroot();
            path.push(url.host_str().unwrap());
            path.push(
                url.path_segments().unwrap()
                .map(move |c| format!("{}/", c)).collect::<String>());
            Ok(path.to_str().unwrap().to_string())
        },
        Err(e) => Err(e)
    }
}

fn ghqroot() -> PathBuf {
    let config = Config::open_default().expect("You must write .gitconfig first");
    config.get_path("ghq.root").expect("You must set ghqroot (git config ghq.root)")
}
