use anyhow::{Context, Result};
use pulldown_cmark::{Parser, html};
use tera::{Kwargs, State, Tera, context};

use crate::resume::Resume;

/// Tera filter rendering a Markdown string to HTML.
fn markdown(text: &str, _kwargs: Kwargs, _state: &State) -> String {
    let mut rendered = String::new();
    html::push_html(&mut rendered, Parser::new(text));
    rendered
}

pub fn render(templates_glob: &str, resume: &Resume) -> Result<String> {
    let mut tera = Tera::default();
    tera.register_filter("markdown", markdown);
    tera.load_from_glob(templates_glob)?;
    let context = context! {resume => &resume};
    tera.render("index.html", &context)
        .context("Failed to render template")
}
