//! Types and methods for modeling and building the website.

use std::{ffi::OsStr, fs, path::PathBuf};

use crate::{config, transform, util, Error, Result};

/// Structure representing the site as a whole, containing all the pages and
/// layouts, the site configuration, and the templating engine.
pub(crate) struct Site {
    /// The site configuration defined in the `mksite.toml` file.
    config: config::Config,

    /// The rendering engine for all templating and layouts.
    tera: tera::Tera,

    /// The paths of all the in the source directory.
    sources: Vec<PathBuf>,

    /// The paths of all the layouts to use, if the layouts directory exists.
    layouts: Option<Vec<PathBuf>>,

    /// List of mappings from sources to outputs.
    mappings: Vec<Mapping>,
}

impl Site {
    /// Constructs a new site using the information in the given config.
    pub fn new(config: config::Config) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            tera: tera::Tera::default(),
            sources: util::walk_dir(&config.dirs.src)?,
            mappings: Vec::new(),
            layouts: if config.dirs.layout.exists() {
                Some(util::walk_dir(&config.dirs.layout)?)
            } else {
                None
            },
        })
    }

    /// Builds templates, renders them, applies transforms and layouts, and
    /// copies the results to the configured output directory.
    pub fn build(mut self) -> Result<()> {
        self.build_templates()?;
        let rendered_pages = self.render_pages()?;
        self.prepare_mappings(rendered_pages)?;
        self.apply_transforms()?;
        self.apply_layouts_and_write_output()?;
        self.copy_statics()
    }

    /// Builds (but does not render) Tera templates for the site.
    fn build_templates(&mut self) -> Result<()> {
        // build page templates
        let dir = &self.config.dirs.src;

        log::debug!("Building page templates");
        self.tera
            .add_template_files(util::walk_dir(dir)?.iter().map(|p| (p, None::<String>)))?;

        // build layout templates if they exist
        if self.config.dirs.layout.exists() {
            log::debug!("Building layout templates");
            let dir = &self.config.dirs.layout;

            self.tera
                .add_template_files(util::walk_dir(dir)?.iter().map(|p| (p, None::<String>)))?;
        } else {
            log::info!(
                "Layout directory '{}' does not exist. Layout step will be skipped",
                self.config.dirs.layout.display()
            )
        }

        log::debug!(
            "Added templates: {:#?}",
            self.tera.get_template_names().collect::<Vec<_>>()
        );

        Ok(())
    }

    /// Renders tera templates to produce [Page]s in preparation for transforms and layouts.
    fn render_pages(&mut self) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let mut context = tera::Context::new();
        context.insert("data", &self.config.data);

        // render page contents
        self.render_page_templates(context)
    }

    /// Prepares the mappings required for each [Page] based on transform configurations.
    fn prepare_mappings(&mut self, rendered_pages: Vec<(PathBuf, Vec<u8>)>) -> Result<()> {
        for (source, content) in rendered_pages {
            let mut destination =
                util::swap_prefix(&source, &self.config.dirs.src, &self.config.dirs.out)?;

            if self.config.ignores.transform.contains(&destination) {
                log::info!(
                    "Skipping transform step for '{}' as it is in the transform ignore list",
                    destination.display()
                );
            }

            match source.extension().and_then(OsStr::to_str) {
                Some(ext)
                    if self.config.transforms.contains_key(ext)
                        && !self.config.ignores.transform.contains(&destination) =>
                {
                    log::debug!("Transforms apply to source '{}'", source.display());

                    for (target_ext, transform) in &self.config.transforms[ext] {
                        destination.set_extension(target_ext);

                        log::debug!(
                            "Mapping '{}' -> '{}' via {transform}",
                            source.display(),
                            destination.display()
                        );

                        self.mappings.push(Mapping {
                            source: source.to_owned(),
                            destination: destination.clone(),
                            transform: Some(transform.to_owned()),
                            content: content.to_owned(),
                        })
                    }
                }
                _ => {
                    log::debug!("No transforms apply to source 'src/index.md'");

                    log::debug!(
                        "Mapping '{}' -> '{}'",
                        source.display(),
                        destination.display()
                    );

                    self.mappings.push(Mapping {
                        source: source.to_owned(),
                        destination,
                        transform: None,
                        content,
                    });
                }
            }
        }

        log::debug!(
            "Mapped {} page{}",
            self.mappings.len(),
            if self.mappings.len() != 1 { "s" } else { "" }
        );

        Ok(())
    }

    /// Renders all page templates and returns their contents as byte vecs
    /// (except pages in the templating ignore list, which are simply read and
    /// returned).
    fn render_page_templates(&self, mut context: tera::Context) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let mut res = Vec::new();

        for path in &self.sources {
            if !self.config.ignores.template.contains(path) {
                // we can render this template
                log::info!("Rendering '{}'", path.display());

                let template_name = path
                    .to_str()
                    .ok_or_else(|| Error::PathConversion(path.to_path_buf()))?;

                // TODO: flesh out this hashmap
                context.insert("page", &maplit::hashmap! {"source" => path});

                log::debug!("Using page rendering {context:#?}");

                res.push((
                    path.to_owned(),
                    self.tera.render(template_name, &context)?.into_bytes(),
                ))
            } else {
                log::info!(
                    "Skipping template rendering for '{}' as it is in the template ignore list",
                    path.display()
                );

                // since we didn't render anything, we just grab the file
                // contents directly
                res.push((
                    path.to_owned(),
                    fs::read(path).map_err(|source| Error::Io {
                        msg: format!("Could not read '{}'", path.display()),
                        source,
                    })?,
                ))
            }
        }
        Ok(res)
    }

    /// Apply layouts and write the generated files.
    fn apply_layouts_and_write_output(&self) -> Result<()> {
        for mapping in &self.mappings {
            let layout = self.find_layout(mapping)?;

            let output = match layout {
                // if there's no layout to apply, just transform the mapping's
                // content and use that
                None => mapping.content.clone(),

                // if there is a layout, apply it
                Some(layout) => {
                    log::info!(
                        "Applying layout '{}' to '{}'",
                        layout.display(),
                        mapping.destination.display()
                    );

                    let mut context = tera::Context::new();
                    context.insert("data", &self.config.data);

                    log::debug!(
                        "NOTE: Context field `page.content` is omitted from debug output\nUsing layout rendering {context:#?}"
                    );

                    context.insert(
                        "page",
                        &maplit::hashmap! {
                        "content" => String::from_utf8(mapping.content.clone())
                            .map_err(|source| Error::FromUtf8 {
                                msg: format!(
                                    "Cannot apply layout '{}' to '{}'",
                                    layout.display(),
                                    mapping.destination.display()
                                ),
                                source,
                            })?,
                        // FIXME: replace this unwrap with better code
                        "source_path" => mapping.source.to_str().unwrap().to_owned()},
                    );

                    let layout_name = layout
                        .to_str()
                        .ok_or_else(|| Error::PathConversion(layout.to_path_buf()))?;

                    self.tera.render(layout_name, &context)?.into_bytes()
                }
            };

            log::info!("Writing '{}'", mapping.destination.display());

            if let Some(p) = mapping.destination.parent() {
                fs::create_dir_all(p).map_err(|source| Error::Io {
                    msg: format!("Cannot create '{}'", p.display()),
                    source,
                })?;
            }

            fs::write(&mapping.destination, output).map_err(|source| Error::Io {
                msg: format!("Cannot write '{}'", mapping.destination.display()),
                source,
            })?;
        }

        Ok(())
    }

    /// Returns the path to the applicable layout for a Mapping, if one exists.
    fn find_layout(&self, mapping: &Mapping) -> Result<Option<PathBuf>> {
        if self.config.ignores.layout.contains(&mapping.destination) {
            log::info!(
                "Skipping layout for '{}' as it is in the layout ignore list",
                mapping.destination.display()
            );

            return Ok(None);
        }

        match &self.layouts {
            None => {
                // just do nothing if there's no layout folder
                log::debug!(
                    "Skipping layout for {} as layout directory {} does not exist",
                    mapping.destination.display(),
                    self.config.dirs.layout.display()
                );
                Ok(None)
            }

            Some(layouts) => {
                // if there is a layout folder, look for an applicable layout

                // start with the corresponding path
                let layout_path = util::swap_prefix(
                    &mapping.destination,
                    &self.config.dirs.out,
                    &self.config.dirs.layout,
                )?;

                // if that doesn't exist we'd better go looking for it
                if !layouts.contains(&layout_path) {
                    log::debug!(
                        "Exact layout match for '{}' not found",
                        mapping.destination.display()
                    );

                    // all this work to concatenate a file extension with an
                    // underscore :/
                    let wildcard = "_".to_owned()
                        + &match layout_path.extension() {
                            None => "".to_owned(),
                            Some(ext) => {
                                if let Some(ext) = ext.to_str() {
                                    ".".to_owned() + ext
                                } else {
                                    "".to_owned()
                                }
                            }
                        };

                    // iterate up the directory tree until we find a matching
                    // wildcard layout
                    for ancestor in layout_path.ancestors() {
                        log::debug!(
                            "Searching for wildcard layout '{}' in '{}/'",
                            wildcard,
                            ancestor.display()
                        );
                        let layout_path = ancestor.join(&wildcard);
                        if layouts.contains(&layout_path) {
                            log::debug!("Found layout '{}'", layout_path.display());
                            return Ok(Some(layout_path));
                        }
                    }
                }

                // If we get here we didn't find it.
                Ok(None)
            }
        }
    }

    /// Applies the transforms for every mapping, mutating them.
    fn apply_transforms(&mut self) -> Result<()> {
        for mapping in &mut self.mappings {
            mapping.transform()?;
        }

        Ok(())
    }

    /// Copies the contents of the static dir to the output dir.
    fn copy_statics(&self) -> Result<()> {
        for asset in util::walk_dir(&self.config.dirs.r#static)? {
            let destination =
                util::swap_prefix(&asset, &self.config.dirs.r#static, &self.config.dirs.out)?;

            if destination.exists() {
                log::debug!(
                    "'{}' already exists and will be overwritten.",
                    destination.display()
                );
            }

            log::info!(
                "Copying '{}' to '{}'",
                asset.display(),
                destination.display()
            );

            if let Some(p) = destination.parent() {
                fs::create_dir_all(p).map_err(|source| Error::Io {
                    msg: format!("Cannot create '{}'", p.display()),
                    source,
                })?;
            }

            fs::copy(&asset, &destination).map_err(|source| Error::Io {
                msg: format!(
                    "Cannot copy static asset '{}' to '{}'",
                    asset.display(),
                    destination.display()
                ),
                source,
            })?;
        }

        Ok(())
    }
}

/// Maps a rendered source template to a destination page via a transform.
#[derive(serde::Serialize)]
struct Mapping {
    /// The path to the source file this page was generated from, relative to
    /// the project root (eg `src/index.md`).
    source: PathBuf,

    /// The path this page will be written to, relative to the project root
    /// (eg `out/index.html`).
    destination: PathBuf,

    /// The transform to apply to this page, if any is applicable.
    transform: Option<transform::Transform>,

    /// The contents of this page after templating. Stored as a byte vec so we
    /// can handle non-UTF-8 inputs via the ignore list.
    content: Vec<u8>,
}

impl Mapping {
    /// Applies this mapping's transform to its content, if one applies.
    pub fn transform(&mut self) -> Result<()> {
        if let Some(transform) = &self.transform {
            log::info!(
                "Applying transform {} to '{}'",
                transform,
                self.destination.display()
            );

            self.content = transform.apply(&self.content)?;
        };

        Ok(())
    }
}
