// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.extern crate rustc_serialize;

extern crate handlebars;
extern crate toml;
extern crate rustc_serialize;

use std::io::{self, Write, Read};
use std::process;
use std::env;
use std::fs::File;

use toml::Value;

pub mod util;
use util::convert;
use util::handlebars_helpers;

fn usage(cmd: &str) -> ! {
    writeln!(&mut io::stderr(), "Usage: {} TEMPLATE TOMLFILE", cmd)
        .ok();
    process::exit(1);
}

fn parse_toml(filename: &str) -> Value {
    let mut content = String::new();
    File::open(&filename).and_then(|mut f| {
            f.read_to_string(&mut content)
            }).unwrap();
    let mut parser = toml::Parser::new(&content);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         filename, loline, locol, hiline, hicol, err.desc);
            }
            process::exit(1);
        }
    };
    let mut final_toml = toml::Table::new();
    final_toml.insert(String::from("cfg"), Value::Table(toml));
    return Value::Table(final_toml);
}

pub fn never_escape_fn(data: &str) -> String {
    String::from(data)
}

fn main() {
    let mut args = env::args();
    let cmd = args.next().unwrap();
    let (filename, tomlfilename) = match (args.next(), args.next()) {
        (Some(filename), Some(tomlfilename)) => (filename, tomlfilename),
        _ => usage(&cmd),
    };
    let data = parse_toml(&tomlfilename);
    let json_data = convert::toml_to_json(data);

    let mut handlebars = handlebars::Handlebars::new();

    handlebars.register_helper("json", Box::new(handlebars_helpers::json_helper));
    handlebars.register_helper("toml", Box::new(handlebars_helpers::toml_helper));

    // By default, handlebars escapes HTML. We don't want that.
    handlebars.register_escape_fn(never_escape_fn);

    match handlebars.register_template_file(&filename, &filename) {
        Ok(data) => data,
        Err(e) => {
            println!("Error loading template {}: {:?}", filename, e);
            process::exit(2);
        }
    }

    match handlebars.render(&filename, &json_data) {
        Ok(data) => {
            print!("{}", data);
        }
        Err(e) => {
            println!("Error rendering {}: {}", filename, e);
            process::exit(2);
        }
    }
}
