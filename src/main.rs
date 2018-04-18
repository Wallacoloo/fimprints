extern crate polly;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate toml;

mod story;

use polly::Template;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

use story::Story;

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
    let stories_dir = args.in_dir.join("stories");
    let templates_dir = args.in_dir.join("templates");

    // Collect all the stories from the input directory
    let stories : io::Result<Vec<Story>> = stories_dir.read_dir()
        .expect("unable to read input directory")
        .map(|maybe_dir_entry| {
            Ok(Story::from_path(maybe_dir_entry?.path()))
        }).collect();
    let stories = stories.expect("unable to read stories from input directory");

    // Load templates
    for s in stories {
        let story_brief_template = Template::load(templates_dir.join("story_brief.polly"))
            .expect("Unable to load story_brief.polly template");
        //let as_val = serde_json::to_value(&s).unwrap();
        //let json = as_val.as_object().unwrap().clone().into_iter().collect();
        //story_brief_template.json(json);
        println!("Story: {:?}", s);
    }
}
