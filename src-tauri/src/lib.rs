mod owner;

use chrono::{DateTime, Local};
use owner::Owner;
use std::{error::Error, fs, os::unix::fs::MetadataExt, path::PathBuf};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(serde::Serialize)]
pub struct FileInfo {
    is_dir: bool,
    perms: String,
    nlink: u64,
    user: String,
    group: String,
    len: u64,
    modified: String,
    path: String,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn find_files(path: &PathBuf, show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut results = vec![];

    match fs::metadata(path) {
        Err(e) => eprintln!("{:?}: {}", path, e),
        Ok(meta) => {
            if meta.is_dir() {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    let is_hidden = path.file_name().map_or(false, |file_name| {
                        file_name.to_string_lossy().starts_with('.')
                    });
                    if !is_hidden || show_hidden {
                        results.push(entry.path());
                    }
                }
            } else {
                results.push(PathBuf::from(path));
            }
        }
    }

    Ok(results)
}

pub fn find_files_info(paths: &[PathBuf]) -> MyResult<Vec<FileInfo>> {
    let mut file_info_arr: Vec<FileInfo> = Vec::new();

    // 12345678
    for path in paths {
        let metadata = path.metadata()?;
        let uid = metadata.uid();
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string());
        let gid = metadata.gid();
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| gid.to_string());
        let perms = format_mode(metadata.mode());
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

        file_info_arr.push(FileInfo {
            is_dir: path.is_dir(),
            perms,
            nlink: metadata.nlink(),
            user,
            group,
            len: metadata.len(),
            modified: modified.to_string(),
            path: path.display().to_string(),
        })
    }

    Ok(file_info_arr)
}

/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
fn format_mode(mode: u32) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}

/// Given an octal number like 0o500 and an [`Owner`], /// return a string like "r-x"
pub fn mk_triple(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}
