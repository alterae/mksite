//! The `mksite build` subcommand.

use std::{fs, path::Path};

use fs_extra::dir::CopyOptions;
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

    println!("Building templates:\n");

    for template in tera.get_template_names() {
        println!("Rendering {template}...");
        let output = tera.render(template, &context)?;

        let path = Path::new(&config.dirs.out).join(template);

        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?;
        }

        // TODO: processing, layouts

        println!("  Writing {path:?}...");

        fs::write(path, output)?;
    }

    if Path::new(&config.dirs.r#static).exists() {
        println!("\nCopying static files...\n");

        // TODO: implement this manually at some point because `fs_extra` is a
        // poorly documented black with limited introspection capabilities.
        fs_extra::dir::copy(
            config.dirs.r#static,
            config.dirs.out,
            &CopyOptions {
                overwrite: true,
                content_only: true,
                ..Default::default()
            },
        )?;
    } else {
        println!(
            "\nSkipping copying static files: no {:?} directory\n",
            config.dirs.r#static
        )
    }

    Ok(())
}
