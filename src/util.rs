//! Miscellaneous utility fns that don't go anywhere else.

use crate::Error;

use std::fs;

use std::path::PathBuf;

use crate::Result;

use std::path::Path;

/// Walks a directory and returns every path in it. Probably not very performant.
pub(crate) fn walk_dir(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let read_dir = fs::read_dir(&dir).map_err(|source| Error::Io {
        msg: format!("Cannot read source directory '{}'", dir.as_ref().display()),
        source,
    })?;

    let mut res = Vec::new();

    for entry in read_dir {
        let entry = entry.map_err(|source| Error::Io {
            msg: format!("Cannot get entry in '{}'", dir.as_ref().display()),
            source,
        })?;

        let path = entry.path();

        if path.is_dir() {
            res.append(&mut walk_dir(path)?);
        } else {
            res.push(path);
        }
    }

    Ok(res)
}

/// Strips the `old` prefix from a path and replaces it with `new`.
pub(crate) fn swap_prefix(
    path: impl AsRef<Path>,
    old: impl AsRef<Path>,
    new: impl AsRef<Path>,
) -> Result<PathBuf> {
    let stripped = path
        .as_ref()
        .strip_prefix(&old)
        .map_err(|source| Error::StripPath {
            path: path.as_ref().to_owned(),
            prefix: old.as_ref().to_owned(),
            source,
        })?;
    Ok(new.as_ref().join(stripped))
}
