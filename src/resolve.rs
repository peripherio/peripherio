use error::CannotResolveError;

use failure::Error;

use std::env;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};

pub fn resolve(name: &str, environ: &'static str, file: &'static str) -> Result<PathBuf, Error> {
    paths(environ, file)?
        .find(|path| path.join(file).is_file())
        .map(|path| path.to_path_buf())
        .ok_or(
            CannotResolveError {
                name: name.to_owned(),
            }.into(),
        )
}

pub fn paths(
    environ: &'static str,
    file: &'static str,
) -> Result<impl Iterator<Item = PathBuf>, Error> {
    Ok(env::var(environ)
        .as_ref()
        .map(|val| val.split(';').collect())
        .unwrap_or(vec![])
        .iter()
        .map(|path| Path::new(path).read_dir())
        .collect::<Result<Vec<ReadDir>, _>>()?
        .into_iter()
        .flat_map(|re| re)
        .collect::<Result<Vec<DirEntry>, _>>()?
        .into_iter()
        .map(|de| de.path())
        .filter(move |path| path.join(file).is_file()))
}
