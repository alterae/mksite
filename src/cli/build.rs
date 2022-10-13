//! The `mksite build` subcommand.

use std::{fs, path::Path};

use tera::Tera;

use crate::config;

/// Loads all the templates in the `src/` directory and renders them using the
/// metadata defined in `mksite.toml`.
pub(crate) fn cmd() -> anyhow::Result<()> {
    anyhow::ensure!(
        config::exists(),
        "Cannot build site: {} not found",
        config::FILE_NAME
    );
    let config = config::load()?;
    let context = tera::Context::from_serialize(config.data)?;
    let tera = Tera::new(
        Path::new(&config.dirs.src)
            .join("**")
            .join("*")
            .to_str()
            .unwrap(),
    )?;
    for template in tera.get_template_names() {
        let output = tera.render(template, &context)?;

        let path = Path::new(&config.dirs.out).join(template);

        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?;
        }

        // TODO: actuallly use these variables.
        let (_, _, _) = (
            &config.dirs.r#static,
            &config.dirs.layout,
            &config.processors,
        );

        fs::write(path, output)?;
    }
    Ok(())
}
