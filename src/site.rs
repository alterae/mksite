//! Types and methods for modeling a website.

use std::{fs, path::Path};

use crate::{config, Error, Result};

/// Structure representing the site as a whole.
pub(crate) struct Site {
    /// The config for the site.
    pub config: config::Config,

    /// Tera rendering engine holding all the page template files for a site.
    pub templates: tera::Tera,

    /// Tera rendering engine holding all the layouts for a site.
    pub layouts: Option<tera::Tera>,
}

impl Site {
    /// Constructs a new site using the information in the config.
    pub fn new(config: config::Config) -> Result<Self> {
        log::debug!("Building templates");
        let templates = tera::Tera::new(&format!("{}/**/*", config.dirs.src))?;

        let layouts = if Path::new(&config.dirs.layout).exists() {
            log::debug!("Building layouts");
            Some(tera::Tera::new(&format!("{}/**/*", config.dirs.layout))?)
        } else {
            log::warn!(
                "Layouts directory '{}' does not exist, so no layouts will be applied",
                config.dirs.layout
            );
            None
        };

        Ok(Self {
            config,
            templates,
            layouts,
        })
    }

    /// Builds the site, writing the outputs to the configured out directory.
    pub fn build(&self) -> Result<()> {
        let context = tera::Context::from_serialize(&self.config)?;

        for template in self.templates.get_template_names() {
            log::info!("Rendering '{template}'");
            let output = self.templates.render(template, &context)?;

            let path = Path::new(&self.config.dirs.out).join(template);

            if let Some(p) = path.parent() {
                fs::create_dir_all(p).map_err(|source| Error::Io {
                    msg: format!("Cannot create '{}'", p.display()),
                    source,
                })?;
            }

            match path.extension().and_then(std::ffi::OsStr::to_str) {
                Some(ext) if self.config.transforms.contains_key(ext) => {
                    for (ext, transform) in &self.config.transforms[ext] {
                        let path = &path.with_extension(ext);

                        log::info!("Transforming '{}'", path.display());
                        let output = transform.apply(output.as_bytes())?;

                        // Only apply layouts if the layouts dir exists.
                        let output = if self.layouts.is_some() {
                            self.apply_layout(path, &output)?
                        } else {
                            output
                        };

                        log::info!("Writing '{}'", path.display());
                        fs::write(path, output).map_err(|source| Error::Io {
                            msg: format!("Cannot write '{}'", path.display()),
                            source,
                        })?;
                    }
                }
                _ => {
                    // Only apply layouts if the layouts dir exists.
                    let output = if self.layouts.is_some() {
                        self.apply_layout(&path, output.as_bytes())?
                    } else {
                        output.into_bytes()
                    };

                    log::info!("Writing '{}'", path.display());
                    fs::write(&path, output).map_err(|source| Error::Io {
                        msg: format!("Cannot write '{}'", path.display()),
                        source,
                    })?;
                }
            }
        }

        if Path::new(&self.config.dirs.r#static).exists() {
            log::info!("Copying static files");

            // TODO: implement this manually at some point because `fs_extra` is a
            // poorly documented black with limited introspection capabilities.
            fs_extra::dir::copy(
                &self.config.dirs.r#static,
                &self.config.dirs.out,
                &fs_extra::dir::CopyOptions {
                    overwrite: true,
                    content_only: true,
                    ..Default::default()
                },
            )?;
        } else {
            log::warn!(
                "Skipping copying static files: no '{}' directory",
                self.config.dirs.r#static
            )
        }

        Ok(())
    }

    // TODO: Refactor layouts to have their own dedicated type(s)
    /// Applies a layout to a page.
    pub(crate) fn apply_layout(&self, path: &Path, body: &[u8]) -> Result<Vec<u8>> {
        let layouts = match &self.layouts {
            Some(layouts) => layouts,
            None => {
                log::error!(
                    "Tried to apply layout '{}' when layout directory '{}' does not exist",
                    path.display(),
                    self.config.dirs.layout
                );
                return Ok(body.to_owned());
            }
        };

        let stripped =
            path.strip_prefix(&self.config.dirs.out)
                .map_err(|source| Error::StripPath {
                    path: path.into(),
                    prefix: self.config.dirs.out.clone(),
                    source,
                })?;
        let path = Path::new(&self.config.dirs.layout).join(stripped);
        let wildcard = format!(
            "_{}",
            match path.extension().and_then(std::ffi::OsStr::to_str) {
                Some(ext) => ".".to_owned() + ext,
                None => "".to_owned(),
            }
        );

        // Try to find a default layout.
        let layout = if path.exists() {
            Some(stripped.to_owned())
        } else {
            // Walk up the file tree until we find a matching default layout
            // or run out of parents.
            let mut res = None;
            for ancestor in path.ancestors() {
                let path = ancestor.join(&wildcard);
                res = if path.exists() {
                    // The layout path is the path to the layout _file_ with the
                    // name of the layout directory removed, so that we can pass
                    // the path to Tera
                    let layout = path
                        .strip_prefix(&self.config.dirs.layout)
                        .map_err(|source| Error::StripPath {
                            path: path.clone(),
                            prefix: self.config.dirs.layout.clone(),
                            source,
                        })?
                        .to_owned();
                    Some(layout)
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
            log::info!("Applying layout '{}'", layout.display());
            let mut context = tera::Context::from_serialize(&self.config)?;
            context.insert("data", &self.config.data);
            context.insert(
                "content",
                &String::from_utf8(body.to_owned()).map_err(|source| Error::FromUtf8 {
                    msg: format!(
                        "Cannot apply layout '{}' to '{}'",
                        layout.display(),
                        path.display()
                    ),
                    source,
                })?,
            );

            let layout = layout
                .to_str()
                .ok_or_else(|| Error::PathConversion(layout.clone()))?;

            Ok(layouts.render(layout, &context)?.into_bytes())
        } else {
            Ok(body.to_owned())
        }
    }
}
