// Copyright 2017 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// you can obtain one at http://mozilla.org/MPL/2.0/.


extern crate docopt;
extern crate minifier;
extern crate rustc_serialize;


use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use docopt::Docopt;
use minifier::html;


mod parser;


const VERSION_STR: &'static str = "htmli 0.1.2

Copyright (c) 2017 Nathan Sizemore <nathanrsizemore@gmail.com>
License: MPL-2.0 https://www.mozilla.org/en-US/MPL/2.0
This is free software: you are free to change and redistribute it.";


const USAGE: &'static str = "
htmli - Utility to statically resolve html-include directives.

Usage:
    htmli <file> [--minify] [--output=<f>]
    htmli (-h | --help)
    htmli --version

Options:
    -m --minify        Minifies output.
    -o --output=<f>    Direct output to file.

    -h --help          Show this screen.
    --version          Show version.
";


#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_file: String,
    pub flag_minify: bool,
    pub flag_output: String,
    pub flag_version: bool
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    match handle_input(&args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };
}

fn handle_input(args: &Args) -> Result<(), String> {
    if args.flag_version {
        println!("{}", VERSION_STR);
        return Ok(());
    }

    let (input_path, mut input_file) = try!(open_as_file(&args.arg_file));
    let mut html_str = try!(parser::parse(input_path, &mut input_file));

    if args.flag_minify {
        html_str = html::minify(&html_str);
    }

    if args.flag_output.len() == 0 {
        println!("{}", html_str);
        return Ok(());
    }

    let output_path = Path::new(&args.flag_output);
    let mut output_file = try!(create_file(output_path));
    let result = try!(write_to_file(&mut output_file, html_str.as_bytes()));

    Ok(result)
}

fn open_as_file(s: &String) -> Result<(&Path, File), String> {
    let path = Path::new(s);
    let result = File::open(path);
    if result.is_err() { return Err(desc_from_err(result)); }
    let file = result.unwrap();

    Ok((path, file))
}

fn create_file(path: &Path) -> Result<File, String> {
    let result = File::create(path);
    if result.is_err() { return Err(desc_from_err(result)); }
    let file = result.unwrap();

    Ok(file)
}

fn write_to_file(file: &mut File, bytes: &[u8]) -> Result<(), String> {
    let result = file.write_all(bytes);
    if result.is_err() { Err(desc_from_err(result)) } else { Ok(()) }
}

fn desc_from_err<T: Debug>(result: Result<T, io::Error>) -> String {
    let err = result.unwrap_err();
    format!("{}", err.description())
}
