#[macro_use] extern crate clap;
#[macro_use] extern crate emojicons;

use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::{self, Command, Output};
use std::str;

use clap::{App, Arg};

fn main() {
    let home_dir = std::env::home_dir()
        .and_then(|p| p.to_str().map(|s| s.to_owned()))
        .expect("missing home directory");
    let matches_result = App::new("hacknow")
        .about("A utility for managing workspaces and project directories")
        .author("Marcus Weiner <melodie124@gmail.com>, Moritz Gunz <moritz.gunz@gmail.com>")
        .version(crate_version!())
        .arg(Arg::with_name("REPO")
            .index(1)
            .value_name("REPO")
            .help("The repo ID in the form of <Github User>/<Repo Name>")
            .required(true))
        .arg(Arg::with_name("PROJECT-DIR")
            .short("d")
            .long("dir")
            .value_name("DIR")
            .help("The directory to clone the project to")
            .default_value(&home_dir)
            .takes_value(true))
        .arg(Arg::with_name("SSH")
            .long("ssh")
            .help("Use SSH for cloning (HTTPS is the default)"))
        .get_matches_safe();

    if let Err(err) = matches_result {
        eprintln!("{}", &err.message);
        exit();
    }

    let matches = matches_result.unwrap();
    let repo = matches.value_of("REPO").unwrap();
    let dir = matches.value_of("PROJECT-DIR").unwrap();
    let ssh = matches.is_present("SSH");

    hacknow(repo, dir, ssh);
}

fn hacknow(repo: &str, dir: &str, ssh: bool) -> ! {
    let full_dir = PathBuf::from(dir).join(repo);
    let stat = fs::metadata(&full_dir)
        .map(|md| md.is_dir())
        .ok();

    match stat {
        Some(true) => {
            eprintln!("{} Fetching from origin...", emoji!("arrow_down"));
            let o = Command::new("git")
                .arg("fetch")
                .arg("--all")
                .current_dir(&full_dir)
                .output();
            handle_git_result(o);

            eprintln!("{} Repository overview:", emoji!("white_check_mark"));
            let o = Command::new("git")
                .arg("status")
                .current_dir(&full_dir)
                .output();
            let o = handle_git_result(o);
            eprintln!("{}", String::from_utf8_lossy(&o.stdout));
        },
        Some(false) => {
            eprintln!("On the desired path is a file, therefore we can not create a directory there");
            exit();
        },
        None => {
            let remote = if ssh {
                format!("git@github.com:{}", repo)
            } else {
                format!("https://github.com/{}", repo)
            };

            eprintln!("{} Cloning into new directory...", emoji!("arrow_down"));
            let o = Command::new("git")
                .arg("clone")
                .arg(&remote)
                .arg(&full_dir)
                .output();
            handle_git_result(o);
            eprintln!("{} Repository ready", emoji!("white_check_mark"));
        }
    }

    println!("{}", full_dir.to_str().expect("final directory path is not UTF-8 compatible"));
    process::exit(0);
}

fn exit() -> ! {
    println!(".");
    process::exit(1)
}

fn handle_git_result(res: Result<Output>) -> Output {
    match res {
        Ok(output) => {
            if !output.status.success() {
                let output_string = String::from_utf8_lossy(&output.stderr);
                eprintln!("{} Git command failed:\n{}", emoji!("warning"), output_string);
                exit();
            }

            output
        },
        Err(_) => {
            eprintln!("{} Launching git failed!", emoji!("warning"));
            exit();
        }
    }
}