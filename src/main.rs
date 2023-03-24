#![allow(unused_imports)]
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;

use serde_json::Value;
use serde_json::json;

use bson::Document;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bisonn <file.bson>");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);

    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).unwrap();

    let doc = Document::from_reader(&bytes[..]).unwrap();
    let doc = replace_bin(doc);
    let jdoc = serde_json::to_string_pretty(&doc).unwrap();
    println!("{}", jdoc);
}

fn replace_bin(data: Document) -> Document {
    let mut new_doc = Document::new();

    for (key, value) in data {
        let value = match value {
            bson::Bson::Binary(_) => bson::Bson::String("BINARY".to_owned()),
            bson::Bson::Document(doc) => bson::Bson::Document(replace_bin(doc)),
            bson::Bson::Array(arr) => bson::Bson::Array(replace_bin_array(arr)),
            _ => value,
        };

        new_doc.insert(key, value);
    }

    new_doc
}

fn replace_bin_array(data: bson::Array) -> bson::Array {
    let mut new_arr = bson::Array::new();

    for value in data {
        let value = match value {
            bson::Bson::Binary(_) => bson::Bson::String("BINARY".to_owned()),
            bson::Bson::Document(doc) => bson::Bson::Document(replace_bin(doc)),
            bson::Bson::Array(arr) => bson::Bson::Array(replace_bin_array(arr)),
            _ => value,
        };

        new_arr.push(value);
    }

    new_arr
}
