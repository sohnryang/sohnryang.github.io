use anyhow::{Context, Result};
use tera::{Tera, context};

use crate::resume::Resume;

pub fn render(templates_glob: &str, resume: &Resume) -> Result<String> {
    let mut tera = Tera::default();
    tera.load_from_glob(templates_glob)?;
    let context = context! {resume => &resume};
    tera.render("index.html", &context)
        .context("Failed to render template")
}
