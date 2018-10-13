#![feature(nll)]

extern crate env_logger;
extern crate fimfiction_api;
extern crate fs_extra;
extern crate handlebars;
#[macro_use]
extern crate log;
extern crate pathdiff;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate structopt;
extern crate toml;

mod builder;
mod story;

use fimfiction_api::{Application, ApiResponse};
use fs_extra::{copy_items, dir::CopyOptions};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use builder::Builder;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "fimprints")]
struct CliArgs {
    /// path to directory containing the site sources (e.g. story files)
    #[structopt(short = "i", long = "input", parse(from_os_str), default_value = ".")]
    in_dir: PathBuf,
    /// path to directory in which to assemble the website
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    out_dir: Option<PathBuf>,
    /// Pull fresh metadata for all stories (from fimfiction.net)
    #[structopt(short = "u", long = "update")]
    update_stories: bool,
}

/// Data needed to obtain API access from fimfiction.net.
#[derive(Deserialize)]
struct FimficApiInfo {
    client_id: String,
    client_secret: String,
}


fn main() {
    env_logger::init();
    // Parse arguments
    let args = CliArgs::from_args();

    let in_dir = fs::canonicalize(args.in_dir).unwrap();
    let mut builder = Builder::new(&in_dir);

    if args.update_stories {
        info!("Updating stories");
        //let resp: ApiResponse = serde_json::from_reader(File::open(".debug.story").unwrap()).unwrap();
        //println!("parsed from disk: {:?}", resp);
        let api_info: FimficApiInfo = toml::from_str(
            &fs::read_to_string(in_dir.join("fimfic-api.toml"))
                .expect("to use the auto-updater, populate fimfic-api.toml with your fimfiction `client_id` and `client_secret`.")
        ).unwrap();
        let app = Application::authorize_from_client_credentials(&api_info.client_id, &api_info.client_secret).unwrap();
        for story in builder.stories.iter_mut() {
            if let Some(id) = story.meta.fimfic_id {
                info!("Updating story {}", id);
                match app.story(id) {
                    Err(err) => warn!("Unable to fetch fimfiction info for story {}: {:?}", id, err),
                    Ok(res) => {
                        story.meta.title = Some(res.data.attributes.title);
                        story.meta.synopsis = Some(res.data.attributes.description);
                        story.meta.num_words = Some(res.data.attributes.num_words);
                        story.meta.total_num_views = Some(res.data.attributes.total_num_views);
                        story.meta.content_rating = Some(res.data.attributes.content_rating);
                        story.meta.num_likes = Some(res.data.attributes.num_likes);
                        story.meta.num_dislikes = Some(res.data.attributes.num_dislikes);
                        //let author_id = res.data.relationships.author.data.id;
                        //match app.user(author_id) {
                        //    Err(err) => warn!("Unable to fetch fimfiction info for author {}: {:?}", author_id, err),
                        //    Ok(user) => {
                        //        story.meta.author = Some(user.data.name);
                        //    }
                        //}
                        story.update_on_disk();
                    }
                }
            }
        }
        //println!("blog_post: {:?}", app.blog_post(808406));
        //println!("bookshelf: {:?}", app.bookshelf(16299));
        ////println!("chapter: {:?}", app.chapter(0));
        //println!("group: {:?}", app.group(209275));
        //println!("story: {:?}", app.story(141549));
        //println!("user: {:?}", app.user(33084));
    }

    if let Some(out_dir) = args.out_dir {
        let out_dir = fs::canonicalize(out_dir).unwrap();
        fs::create_dir_all(&out_dir)
            .expect("Unable to create output directory");

        builder.build_page("index.html", "index", &out_dir);
        // Copy resources
        copy_items(&vec![in_dir.join("static"), in_dir.join("stories")], &out_dir, &CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: true,
            depth: 0,
        }).expect("Unable to copy static data to output directory");
    }
}

