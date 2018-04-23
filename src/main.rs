#![feature(nll)]
#![feature(fs_read_write)]

extern crate fs_extra;
extern crate handlebars;
extern crate pathdiff;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate toml;

mod builder;
mod story;

use fs_extra::{copy_items, dir::CopyOptions};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use builder::Builder;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "fimprints")]
struct CliArgs {
    /// path to directory containing the site sources (e.g. story files)
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    in_dir: PathBuf,
    /// path to directory in which to assemble the website
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    out_dir: PathBuf,
}


fn main() {
    // Parse arguments
    let args = CliArgs::from_args();

    let in_dir = fs::canonicalize(args.in_dir).unwrap();
    fs::create_dir_all(&args.out_dir)
        .expect("Unable to create output directory");
    let out_dir = fs::canonicalize(args.out_dir).unwrap();

    let builder = Builder::new(&in_dir, &out_dir);

    builder.build_page("index.html", "index");
    // Copy resources
    copy_items(&vec![in_dir.join("stories")], &out_dir.join("stories"), &CopyOptions {
        overwrite: true,
        skip_exist: true,
        buffer_size: 64000,
        copy_inside: true,
        depth: 0,
    }).expect("Unable to copy story data to output directory");
}

