#![feature(fs_read_write)]

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

    let builder = Builder::new(args.in_dir, args.out_dir);

    builder.build_page("index.html", "index");
}

