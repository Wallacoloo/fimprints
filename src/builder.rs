use handlebars::Handlebars;
use pathdiff::diff_paths;
use std::fs::OpenOptions;
use std::io;
use std::path::{Path, PathBuf};

use story::Story;

pub struct Builder {
    src_tree_root: PathBuf,
    build_tree_root: PathBuf,
    reg: Handlebars,
    stories: Vec<StoryData>,
}

#[derive(Serialize)]
pub struct StoryData {
    meta: Story,
    /// Path to story data, relative to src/build root.
    dir: PathBuf,
}

impl Builder {
    pub fn new<S, B>(src_tree_root: S, build_tree_root: B) -> Self
        where S: Into<PathBuf> + AsRef<Path>, B: Into<PathBuf>
    {
        let templates_dir = src_tree_root.as_ref().join("templates");

        let reg = init_templates(templates_dir);

        let mut me = Self {
            src_tree_root: src_tree_root.into(),
            build_tree_root: build_tree_root.into(),
            reg: reg,
            stories: vec![],
        };
        // Collect all the stories from the input directory
        let stories : io::Result<Vec<StoryData>> = me.src_stories_dir().read_dir()
            .expect("unable to read input directory")
            .map(|maybe_dir_entry| {
                Ok(StoryData::from_dir(&me, maybe_dir_entry?.path()))
            }).collect();
        me.stories = stories.expect("unable to read stories from input directory");
        me
    }
    /// Render the page and write it to disk.
    /// template is the human-friendly template name,
    /// page is the path in the build tree at which to place the built page.
    pub fn build_page<P: AsRef<Path>>(&self, page: P, template: &str) {
        #[derive(Serialize)]
        struct RenderData<'a> {
            src_tree_root: &'a PathBuf,
            build_tree_root: &'a PathBuf,
            stories: &'a Vec<StoryData>,
        }
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(page)
            .expect("Unable to open output file for writing");
        let data = RenderData {
            src_tree_root: &self.src_tree_root,
            build_tree_root: &self.build_tree_root,
            stories: &self.stories,
        };
        self.reg.render_to_write(template, &data, &mut file)
            .expect("failed to build page");
    }
    fn src_stories_dir(&self) -> PathBuf {
        self.src_tree_root.join("stories")
    }
}

impl StoryData {
    fn from_dir<D: AsRef<Path>>(b: &Builder, dir: D) -> Self {
        Self {
            meta: Story::from_path(dir.as_ref().join("meta.toml")),
            dir: diff_paths(dir.as_ref(), &b.src_stories_dir())
                .expect("failed to diff paths"),
        }
    }
}

fn init_templates(templates_dir: PathBuf) -> Handlebars {
    //fn hex_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    //    // just for example, add error check for unwrap
    //    let param = h.param(0).unwrap().value();
    //    let rendered = format!("0x{:x}", param.as_u64().unwrap());
    //    try!(rc.writer.write(rendered.into_bytes().as_ref()));
    //    Ok(())
    //}
    //fn render_path(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    //    let param = h.param(0).expect("helper expected one parameter; found none").value();
    //    let output_file = rc.evaluate("output_path_from_build_root")
    //        .expect("Unknown output path");
    //    Ok(())
    //}

    let mut reg = Handlebars::new();
    // Error on using undefined variables
    reg.set_strict_mode(true);
    reg.register_template_file("story_brief", templates_dir.join("story_brief.hbs"))
        .expect("Unable to load 'story_brief.hbs' template");
    reg.register_template_file("index", templates_dir.join("index.hbs"))
        .expect("Unable to load 'index.hbs' template");
    reg
}
