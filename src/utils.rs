use std::path::{Path, PathBuf};

/// Gets common path inside a array of paths
pub fn common_path(paths: &[PathBuf]) -> Option<PathBuf> {
    if paths.len() < 2 {
        return None;
    }

    let mut iter = paths.into_iter();

    let mut ret = iter.next()?.clone();

    for path in iter {
        if let Some(r) = common(ret, path) {
            ret = r;
        } else {
            return None;
        }
    }

    Some(ret.to_owned())
}

fn common<A: AsRef<Path>, B: AsRef<Path>>(a: A, b: B) -> Option<PathBuf> {
    let a = a.as_ref().components();
    let b = b.as_ref().components();
    let mut ret = PathBuf::new();
    let mut found = false;
    for (one, two) in a.zip(b) {
        if one == two {
            ret.push(one);
            found = true;
        } else {
            break;
        }
    }
    if found {
        Some(ret)
    } else {
        None
    }
}
