#[macro_use]
extern crate serde_json;

#[macro_use] extern crate serde_derive;

extern crate clap;

use std::env;
use std::fs;
use std::str::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::os::unix;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use clap::{Arg, App, SubCommand};

mod api;

use api::project::create_project;

fn main() {
    let matches = App::new("Kodesmell")
        .version("0.1.0")
        .author("hanjaekwon <hanjaekwon@icloud.com>")
        .about("Collect code smells")
        .subcommand(SubCommand::with_name("init")
            .about("Init kodesmell in this repository")
        )
        .subcommand(SubCommand::with_name("run")
            .about("Run kodesmell in this repository")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let dir = get_dir();
        let piece: Vec<&str> = dir.split(MAIN_SEPARATOR).collect();
        let name = piece[piece.len() - 1];

        // let id = create_project(name);
        // println!("{:?}", id);

        let dir = create_kodesmell_dir(&dir).unwrap();
        println!("{:?}", dir);
        // write_json(&id);
    } else if let Some(matches) = matches.subcommand_matches("run") {
        println!("Running kodesmell init.")
    }
}

fn get_dir() -> std::string::String {
    let mut dir = env::current_dir().unwrap();
    dir.into_os_string().into_string().unwrap()
}

fn create_kodesmell_dir(root: &std::string::String) -> io::Result<String> {
    let path = Path::new(&root).join(".kodesmell/");

    match fs::create_dir(&path) {
        Ok(_) => {
            Ok(path.into_os_string().into_string().unwrap())
        },
        Err(e) => {
            Err(e)
        }
    }
}

fn write_json(id: &std::string::String) {
    let mut file = File::create("./foo.txt").expect("hehh");
    file.write_all(id.as_bytes()).expect("Unable to write data");
}