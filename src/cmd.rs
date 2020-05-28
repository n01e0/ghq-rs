use crate::util;

pub fn get(url: &str) {
    match util::url2path(url) {
        Ok(path) => {
            println!("cloning {} to {}", url, path);
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
