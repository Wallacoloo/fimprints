## Contributing new stories
To contribute a new story, create a new directory inside `stories/` (for
consistency, name it something like "\<author\>-\<title\>") and populate `toml.ml`
with the following:
```toml
# Populate this with the ID of the story on fimfiction.
# e.g. if the story can be found at https://www.fimfiction.net/story/141549/the-celestia-code
# then the id is 141549.
fimfic_id = 141549
# This should be a link to any website/signup form where the book can be ordered.
order_url = 'http://www.lulu.com/shop/iisaw/the-celestia-code/hardcover/product-23154141.html'
```
You'll see some other `meta.toml` files have `author`, `title`, etc fields.
In the future, these will be auto-filled by using the fimfiction API, but for
now you should also complete those.

**Note**: Github allows one to create a new file and open a pull request without
even cloning the repository. Just look for the "Create new file" button (or
click [here](https://github.com/Wallacoloo/fimprints/new/master/stories))!


## Building
To build the website, clone the repository, install `rustup`, switch to the
nightly version of rust with `rustup default nightly` and then:
```
[fimprints]$ cargo build
[fimprints]$ cargo run -- --input . --output website_root
```
This builds fimprints as a static website. You can open `website_root/index.html`
in a webbrowser, or publish that directory to some domain name, etc.

## Development
Templates use the `handlebars` library.
We store templates in a directory structure similar to that outlined
[here](https://cloudfour.com/thinks/the-hidden-power-of-handlebars-partials/)
`pages/` contains the top-level templates used to render each page.
`partials/layouts/` contains the templates from which each page inherits its
structure.
`partials/` contains the partial templates used to render excerpts that might
be shared across pages (like story blurbs).

