use error::CannotResolveError;

use failure::Error;

use std::env;
use std::path::{Path, PathBuf};

pub fn resolve(name: &str, environ: &str, file: &str) -> Result<PathBuf, Error> {
    env::var(environ).as_ref().map(|val| val.split(';').collect()).unwrap_or(vec![])
        .iter()
        .map(|path| Path::new(path).join(name).join(file))
        .find(|path| path.is_file()).as_ref().and_then(|path| path.parent())
        .map(|path| path.to_path_buf())
        .ok_or(CannotResolveError{name: name.to_owned()}.into())
}
