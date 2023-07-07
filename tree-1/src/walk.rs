use std::path::Path;

use crate::prelude::*;

// walk starts with the current file or dir and then visits each child file and dir
pub fn walk<F>(start: &str, f: F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    let start = Path::new(start);
    walk_path(start, f)
}

fn walk_path<F>(path: &Path, mut f: F) -> WalkResult<()>
where
    F: FnMut(String) -> (),
{
    let pstr = to_string(path);
    if !path.exists() {
        return Err(Error::NotFound(pstr.clone()));
    }
    f(pstr);
    Ok(())
}

fn to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
