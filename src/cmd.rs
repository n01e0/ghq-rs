use crate::util;
use git2::{Repository};

pub fn get(url: &str) {
    match util::url2path(url) {
        Ok(path) => {
            match Repository::clone_recurse(url, path) {
                Ok(repo) => println!("cloned {:?}", repo.path()),
                Err(e) => eprintln!("{}", e.message()),
            }
        },
        Err(e) => eprintln!("{}", e),
    }
}

pub fn pull(url: &str) {
    match util::url2path(url) {
        Ok(path) => println!("{}", path),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn list() {
    println!("listed");
}
