use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    /// Story id on fimfiction.net.
    /// e.g. if the story is found at https://www.fimfiction.net/story/141549/the-celestia-code
    /// its id is 141549.
    pub fimfic_id: Option<u32>,
    /// Link to where one can purchase the story
    pub order_url: Option<String>,
    /// Path to a story thumbnail, relative to the story's .toml file
    pub thumb_path: Option<String>,
    /// Name of author with desired capitalization, e.g. 'iisaw'.
    pub author: Option<String>,
    /// Name of book with desired capitalization, e.g. 'The Celestia Code'.
    pub title: Option<String>,
    /// Brief text-only synopsis. Expect html codes to be escaped.
    pub synopsis: Option<String>,
}

impl Story {
    /// Parse a story from a file on disk
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }
}
