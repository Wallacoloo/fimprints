
# Development
Templates use the `handlebars` library.
We store templates in a directory structure similar to that outlined
[here](https://cloudfour.com/thinks/the-hidden-power-of-handlebars-partials/)
`pages/` contains the top-level templates used to render each page.
`partials/layouts/` contains the templates from which each page inherits its
structure.
`partials/` contains the partial templates used to render excerpts that might
be shared across pages (like story blurbs).

