
extern crate time;
extern crate argparse;
extern crate regex;


use std::path::Path;

use std::io::{BufReader, BufRead, BufWriter, Write, Read};
use std::fs::File;
use std::prelude::*;
use std::str;
use std::time::Instant;
use argparse::{ArgumentParser, Collect, StoreTrue, Store, StoreFalse};
use regex::Regex;

// deprecated
fn is_sep(r: &str) -> bool {
    let s = r.trim();
    /*if s.starts_with("/=======") && s.ends_with("======/") {
        return true
    }
    false*/
    s.starts_with("/=======") && s.ends_with("======/")
}

fn main() {
    let start = Instant::now();
    
    let mut file: String = String::new();
    let mut names: Vec<String> = Vec::new();
    let mut sep: String = "<!-- ---------- -->".to_string();
    let mut ow: bool = false;
    
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Splits a file into multiple files using a separator.  Optionally specify names to name the split files, otherwise they will be named <file>01 etc.");
        ap.refer(&mut file).add_option(&["-f", "--file"], Store, "Specify the input file.").required();
        ap.refer(&mut sep).add_option(&["-s", "--sep"], Store, "Specify the string used as a separator.");
        ap.refer(&mut names).add_argument("arglist", Collect, "List of names to use for the files.  Put a --- to skip writing a file to disk.");
        ap.refer(&mut ow).add_option(&["-o", "--overwrite"], StoreTrue, "Check if the files exist before writing the new chunks to new files, if they exist and this flag is specified exit the program.");
        ap.parse_args_or_exit();
    }
    
    let path = Path::new(&file);
    if !path.exists() || !path.is_file() { panic!("Specified file does not exist or is not a valid file."); }
    
    let mut f = File::open(&file).unwrap();
    // if !f.exists() { 
    //     panic!("File does not exist."); 
    // }
    
    let mut b: Vec<u8> = Vec::new();
    let size = f.read_to_end(&mut b);
    let contents = str::from_utf8(&b).unwrap();
    let mut files: Vec<&str> = Vec::new();
    
    /*if !contents.contains(&sep) {
        panic!("Separator not found.");
    }
    */
    let mut text = &contents[..];
    // let mut has_sep: bool = text.contains(&sep);
    
    
    
    
    
    let re = regex::Regex::new(&sep).unwrap();
    for chunk in re.split(text) {
        files.push(chunk);
    }
    if files.len() == 0 {
        panic!("No chunks found");
    }
    println!("Found {} chunks", files.len());
    
    /*
    let find_chunks = |text: &str, list: Vec<&str>| -> Vec<&str> {
        if text.contains(&sep) {
            let pos = text.find(&sep);
            match pos {
                Some() => {},
                _ => {
                    return list;
                },
            }
            let split = text.split_at(pos);
            
        }
    };*/
    
    /*
    while has_sep == true {
        // let (chunk, txt) = text.split_at(text.find(&sep).unwrap());
        // text = txt;
        let split = text.split_at(text.find(&sep).unwrap());
        let chunk = split.0;
        text = split.1;
        
        files.push(chunk);
        println!("found chunk");
        has_sep = text.contains(&sep);
    }
    */
    
    /*
    let mut pos = contents.find(&sep).unwrap();
    
    // while let (chunk, text) = text.split_at(pos) {
    println!("finding chunks...");
    let mut t = 1;
    loop {
        let (chunk, text) = text.split_at(pos);
        files.push(chunk);
        println!("chunk {} found.", t);
        if !text.contains(&sep) {
            break;
        }
        pos = text.find(&sep).unwrap();
        t += 1;
    }
    println!("{} chunks found.", files.len());
    */
    /*
        0 1 2 3 4
        a b c d
        len = 4
        0 < 4, 1 < 4, 2 < 4, 3 < 4
    */
    
    // if no filenames specified or there are more file chunks than names,
    // intelligently name each split file with the same path and filename 
    // and add an underscore followed by a number, ex c:\file_01.txt
    let mut i = 0usize; // index of file chunk
    let mut j = 0usize; // after names are exhausted start numbering files
    let file_path = path.parent().unwrap().to_str().unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let file_ext = path.extension().unwrap().to_str().unwrap();
    
    for chunk in &files {
        let mut tmp: String;
        let cur_name: &str;
        if i < names.len() {
            tmp = names[i].to_string();
            if !&names[i].contains(".") {
                tmp.push_str(".");
                tmp.push_str(file_ext);
            }
            cur_name = tmp.as_str();
            // cur_name = &names[i];
            if cur_name == "..." {
                i += 1;
                continue; // skip writing this file to disk
            }
        } else {
            // let tmp: String = file_path.to_string();
            // if tmp != "" { tmp.push_str("/"); }
            // tmp.push_str(file_name);
            
            // let mut tmp: String = file_name.to_string();
            tmp = file_name.to_string();
            tmp.push_str("_");
            tmp.push_str(&j.to_string());
            if file_ext != "" {
                tmp.push_str(".");
                tmp.push_str(file_ext);
            }
            cur_name = tmp.as_str();
            j += 1;
        }
        let cur_path = Path::new(cur_name);
        if ow && cur_path.exists() {
            panic!("Existing file found with no overwrite specified.");
        }
        let mut o = BufWriter::new(File::create(cur_name).expect("Could not create output text file."));
        
        o.write(chunk.as_bytes());
        
        i += 1;
    }
    
    let end = start.elapsed();
    println!("Finished in: {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
}

