//! The `mksite build` subcommand.

use crate::{config, site};

use crate::Result;

/// Loads all the templates in the `src/` directory and renders them using the
/// metadata defined in `mksite.toml`.
pub(crate) fn cmd() -> Result<()> {
    site::Site::new(config::load()?)?.build()
}
