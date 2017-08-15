#[macro_use]
extern crate serde_json;

#[macro_use] extern crate serde_derive;

extern crate clap;

use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;
use clap::{Arg, App, SubCommand};

mod api;

use api::project::create_project;
// use std::io::prelude::*;
// use std::fs::File;
// use std::net::TcpListener;
// use std::net::TcpStream;

fn main() {
    let matches = App::new("Kodesmell")
        .version("0.1.0")
        .author("hanjaekwon <hanjaekwon@icloud.com>")
        .about("Collect code smells")
        .subcommand(SubCommand::with_name("init")
            .about("Init kodesmell in this repository")
            .arg(Arg::with_name("input")
                .help("Select specific folder or file to run kodesmell in this project")
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let root = env::current_dir().unwrap();
        let kodesmell = root.join(".kodesmell");
        println!("{}", kodesmell.display());
        
        create_project("kode")
        // match fs::create_dir(&kodesmell) {
        //     Err(why) => {
        //         println!("Aborting kodesmell init. '.kodesmell' {:?}", why.kind());
        //     },
        //     Ok(_) => {
        //     },
        // }
    }
}

// fn main() {
//     // Do some initialization work...
//     //

//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         handle_connect(stream);
//     }

// }

// fn handle_connect(mut stream: TcpStream) {
//     let mut buffer = [0; 512];
//     stream.read(&mut buffer).unwrap();

//     let mut file = File::open("./index.html").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();

//     // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }