//! The `mksite build` subcommand.

pub mod transform;

use std::{ffi::OsStr, fs, path::Path};

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

    println!("Loading config...");
    let config = config::load()?;
    let mut context = tera::Context::new();
    context.insert("data", &config.data);

    println!("Building templates...\n");
    let tera = Tera::new(
        Path::new(&config.dirs.src)
            .join("**")
            .join("*")
            .to_str()
            .unwrap(),
    )?;

    for template in tera.get_template_names() {
        println!("   Rendering {template}...");
        let output = tera.render(template, &context)?;

        let path = Path::new(&config.dirs.out).join(template);

        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?;
        }

        // FIXME: maybe use OsStrs in the config file?
        match path.extension().and_then(OsStr::to_str) {
            Some(ext) if config.transforms.contains_key(ext) => {
                for (ext, proc) in &config.transforms[ext] {
                    let path = &path.with_extension(ext);

                    println!("Transforming {path:?}...");
                    let output = proc.apply(output.as_bytes())?;

                    let output = apply_layout(path, &output)?;
                    println!("     Writing {path:?}...");
                    fs::write(path, output)?;
                }
            }
            _ => {
                let output = apply_layout(&path, output.as_bytes())?;
                println!("     Writing {path:?}...");
                fs::write(&path, output)?;
            }
        }
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

pub fn apply_layout(path: &Path, body: &[u8]) -> anyhow::Result<Vec<u8>> {
    let config = config::load()?;
    let stripped = path.strip_prefix(config.dirs.out)?;
    let path = Path::new(&config.dirs.layout).join(stripped);
    let wildcard = format!(
        "_{}",
        match path.extension().and_then(OsStr::to_str) {
            Some(ext) => ".".to_owned() + ext,
            None => "".to_owned(),
        }
    );

    let layouts = Tera::new(
        Path::new(&config.dirs.layout)
            .join("**")
            .join("*")
            .to_str()
            .unwrap(),
    )?;

    let layout = if path.exists() {
        Some(stripped.to_owned())
    } else {
        let mut res = None;
        for ancestor in path.ancestors() {
            let path = ancestor.join(&wildcard);
            res = if path.exists() {
                Some(path.strip_prefix(&config.dirs.layout)?.to_owned())
            } else {
                None
            };

            if res.is_some() {
                break;
            }
        }
        res
    };

    if let Some(layout) = layout {
        println!("    Applying layout {layout:?}...");
        let mut context = tera::Context::new();
        context.insert("data", &config.data);
        context.insert("content", &String::from_utf8(body.to_owned())?);

        Ok(layouts
            .render(layout.to_str().unwrap(), &context)?
            .into_bytes())
    } else {
        Ok(body.to_owned())
    }
}
