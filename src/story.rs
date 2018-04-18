use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    /// Story id on fimfiction.net.
    /// e.g. if the story is found at https://www.fimfiction.net/story/141549/the-celestia-code
    /// its id is 141549.
    fimfic_id: Option<u32>,
    /// Link to where one can purchase the story
    order_page: Option<String>,
}

impl Story {
    /// Parse a story from a file on disk
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }
}
