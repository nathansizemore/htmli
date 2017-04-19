// Copyright 2017 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// you can obtain one at http://mozilla.org/MPL/2.0/.


use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;


pub fn parse(path: &Path, file: &mut File) -> Result<String, String> {
    let mut html = String::with_capacity(1024);
    try!(parse_to_string(path, file, &mut html, 0, 0));

    Ok(html)
}

fn parse_to_string(path: &Path,
                   file: &mut File,
                   s: &mut String,
                   start: usize,
                   end: usize)
                   -> Result<(), String>
{
    let file_contents = try!(file_to_string(file));
    remove_offsets_insert_str(s, start, end, file_contents.as_str());

    while let Some((start_offset, end_offset)) = find_include_offsets(s) {
        let include_contents = {
            // Parse for the path of the include directive
            let include_slice = &s[start_offset..(end_offset + 1)];
            let result = find_include_src(include_slice);
            if result.is_none() {
                let err_str = "No src for element".to_string();
                return Err(err_str);
            }

            let include_src = result.unwrap();

            // Resolve the full path to the next included file
            let parent_dir = Path::new(path.parent().unwrap().as_os_str());
            let result = parent_dir.join(include_src).canonicalize();
            if result.is_err() {
                let err = result.unwrap_err();
                let err_str = format!("{}", err.description());
                return Err(err_str);
            }

            let include_buf = result.unwrap();
            let include_path = include_buf.as_path();

            // Open the file, and push its contents into our string buffer
            let result = File::open(include_path);
            if result.is_err() {
                let err = result.unwrap_err();
                let err_str = format!("{}", err.description());
                return Err(err_str);
            }

            let mut include_file = result.unwrap();
            try!(file_to_string(&mut include_file))
        };

        remove_offsets_insert_str(s,
                                  start_offset,
                                  (end_offset + 1),
                                  include_contents.as_str());
    }

    Ok(())
}

fn find_include_offsets(s: &String) -> Option<(usize, usize)> {
    let maybe_offset = s.find("<include");
    if maybe_offset.is_none() { return None; }
    let start = maybe_offset.unwrap();

    let maybe_offset = &s[start..].find('>');
    if maybe_offset.is_none() { return None; }
    let end = maybe_offset.unwrap() + start;

    Some((start, end))
}

fn find_include_src(s: &str) -> Option<&str> {
    let maybe_offset = s.find('"');
    if maybe_offset.is_none() { return None; }
    let start = maybe_offset.unwrap() + 1;

    let maybe_offset = &s[start..].find('"');
    if maybe_offset.is_none() { return None; }
    let end = maybe_offset.unwrap() + start;

    let slice = &s[start..end];
    Some(slice)
}

fn remove_offsets_insert_str(s: &mut String,
                             start: usize,
                             end: usize,
                             slice: &str)
{
    s.drain(start..end);
    s.insert_str(start, slice);
}

fn file_to_string(file: &mut File) -> Result<String, String> {
    let mut s = String::with_capacity(1024);
    let result = file.read_to_string(&mut s);
    if result.is_err() {
        let err = result.unwrap_err();
        let err_str = format!("{}", err.description());
        return Err(err_str);
    }

    Ok(s)
}
