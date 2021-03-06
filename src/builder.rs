use bbcode::BBCode;
use handlebars as hb;
use handlebars::Handlebars;
use pathdiff::diff_paths;
use serde_json;
use std::fs::{create_dir_all, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

use story::Story;

pub struct Builder {
    src_tree_root: PathBuf,
    reg: Handlebars,
    pub stories: Vec<StoryData>,
}

#[derive(Serialize)]
pub struct StoryData {
    pub meta: Story,
    /// Path to story data, relative to src/build root.
    dir: PathBuf,
    /// Path to the thumb, relative to src/build root.
    thumb_path: Option<PathBuf>,
}

impl Builder {
    pub fn new<S>(src_tree_root: S) -> Self
        where S: Into<PathBuf> + AsRef<Path>
    {
        let mut me = Self {
            src_tree_root: src_tree_root.into(),
            reg: Handlebars::new(),
            stories: vec![],
        };
        init_templates(&mut me.reg, &me.src_tree_root);

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
    pub fn build_page<P: AsRef<Path>, B: AsRef<Path>>(&self, page: P, template: &str, build_tree_root: B) {
        #[derive(Serialize)]
        struct RenderData<'a> {
            src_tree_root: &'a PathBuf,
            build_tree_root: &'a PathBuf,
            stories: &'a Vec<StoryData>,
            output_path_from_build_root: &'a Path,
        }
        let out_path = build_tree_root.as_ref().join(page.as_ref());
        create_dir_all(out_path.parent().unwrap())
            .expect("Unable to create output directory");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(out_path)
            .expect("Unable to open output file for writing");
        let data = RenderData {
            src_tree_root: &self.src_tree_root,
            build_tree_root: &build_tree_root.as_ref().to_owned(),
            stories: &self.stories,
            output_path_from_build_root: page.as_ref(),
        };
        self.reg.render_to_write(&("pages/".to_string() + template), &data, &mut file)
            .expect("failed to build page");
    }
    fn src_stories_dir(&self) -> PathBuf {
        self.src_tree_root.join("stories")
    }
}

impl StoryData {
    fn from_dir<D: AsRef<Path>>(b: &Builder, dir: D) -> Self {
        let meta = Story::from_path(dir.as_ref().join("meta.toml"));
        let dir = diff_paths(dir.as_ref(), &b.src_tree_root)
                .expect("failed to diff paths");
        let thumb_path = meta.thumb_path.clone().map(|p| dir.join(p));
        Self {
            meta,
            dir,
            thumb_path,
        }
    }
    pub fn update_on_disk<S: AsRef<Path>>(&self, src_tree_root: S) {
        // TODO: we need to get the _absolute path_
        let path = src_tree_root.as_ref().join(&self.dir).join("meta.toml");
        self.meta.to_path(path);
    }
}

fn init_templates(reg: &mut Handlebars, templates_dir: &Path) {
    /// Given a bbcode string, decode it to html.
    fn bbcode_to_html(h: &hb::Helper, _: &Handlebars, rc: &mut hb::RenderContext) -> Result<(), hb::RenderError> {
        let param = h.param(0).expect("helper expected one parameter; found none").value();
        let bbcode: String = serde_json::from_value(param.clone()).unwrap();
        let html = bbcode.as_str().as_html();
        rc.writer.write(html.as_bytes()).unwrap();
        Ok(())
    }
    /// Given a path to some resource inside the repository root, emits into
    /// the template a path relative to the page _currently being rendered_.
    /// This provides a way to track absolute paths internally, and transform
    /// them to relative, HTML-safe paths at render time.
    fn render_path(h: &hb::Helper, _: &Handlebars, rc: &mut hb::RenderContext) -> Result<(), hb::RenderError> {
        let param = h.param(0).expect("helper expected one parameter; found none").value();
        let param: PathBuf = serde_json::from_value(param.clone()).unwrap();
        let output_file = rc.evaluate_absolute("output_path_from_build_root", true)
            .expect("Unknown output path");
        let output_file: PathBuf = serde_json::from_value(output_file.clone()).unwrap();
        let output_dir = output_file.parent().unwrap();
        let relative_path = diff_paths(&param, &output_dir).unwrap();
        rc.writer.write(relative_path.to_str().unwrap().as_bytes()).unwrap();
        Ok(())
    }

    // Error on using undefined variables
    reg.set_strict_mode(true);
    let pages_dir = templates_dir.join("pages");
    let partials_dir = templates_dir.join("partials");
    let layouts_dir = partials_dir.join("layouts");

    reg.register_helper("bbcode_to_html", Box::new(bbcode_to_html));
    reg.register_helper("render_path", Box::new(render_path));
    reg.register_template_file("layouts/base", layouts_dir.join("base.hbs"))
        .expect("Unable to load 'layouts/base' template");
    reg.register_template_file("partials/story_brief", partials_dir.join("story_brief.hbs"))
        .expect("Unable to load 'partials/story_brief' template");
    reg.register_template_file("pages/index", pages_dir.join("index.hbs"))
        .expect("Unable to load 'pages/index' template");
}
