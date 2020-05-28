#[macro_use]
extern crate clap;

use ghq_rs::cmd;

fn main() {
    let matches = 
        clap_app!(ghq_rs =>
                    (version:   crate_version!())
                    (author:    crate_authors!())
                    (about:     crate_description!())
                    (@subcommand get =>
                        (about: "Clone/sync with a remote repository")
                        (@arg update: -u --update "Update local repository if cloned already")
                        (@arg repository: +required "repository URL")
                    )
                    (@subcommand list =>
                        (about: "List locally cloned repositories")
                    )
                ).get_matches();

    match matches.subcommand() {
        ("get", Some(args)) => {
            let url = args.value_of("repository").unwrap();
            if args.is_present("update") {
                cmd::pull(url);
            } else {
                cmd::get(url);
            }
        },
        ("list", Some(_args)) => {
            cmd::list();
        },
        _ => eprintln!("{}", matches.usage()),
    }
}
