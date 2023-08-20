use std::{
    error::Error,
    io::Cursor,
    iter::{empty, once},
    path::{Path, PathBuf}
};

use clap::Parser;
use dirs::home_dir;
use plist::Value;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(about, version)]
struct Opts {
    /// Path to a file, or a directory to check. Default: ~/Downloads
    path: Option<PathBuf>,

    /// Output file name, regardless of xattr kMDItemWhereFroms presence
    #[clap(short, long)]
    all: bool,
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn Error>> {
    let Opts { path, all } = Opts::parse();
    let path = path.unwrap_or_else(|| home_dir().expect("failed to get home directory. Please specify a path to check specifically.").join("Downloads"));

    collect_files(&path)
        .for_each(|file| {
            if let Some(origin) = get_downloaded_url(&file) {
                println!("{}\t{}", file.display(), origin);
            } else if all {
                println!("{}\t(none)", file.display());
            }
        });

    Ok(())
}

fn collect_files(path: &Path) -> Box<dyn Iterator<Item=PathBuf>> {
    if path.is_file() {
        Box::new(once(path.to_path_buf()))
    } else if path.is_dir() {
        Box::new(
            WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.path().is_file())
                .map(|e| e.into_path())
        )
    } else {
        Box::new(empty())
    }
}

fn get_downloaded_url(entry: &dyn AsRef<Path>) -> Option<String> {
    xattr::get(entry, "com.apple.metadata:kMDItemWhereFroms")
        .ok()
        .and_then(|v| v)
        .and_then(|attr| Value::from_reader(Cursor::new(&attr[..])).ok())
        .and_then(|val| val.into_array())
        .filter(|array| array.len() == 2)
        .and_then(|array| array.get(1).map(|v| v.as_string().map(|s| s.trim().to_string())))
        .flatten()
        .filter(|origin| !origin.trim().is_empty())
}
