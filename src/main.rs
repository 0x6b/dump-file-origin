use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser)]
#[clap(about, version)]
struct Opts {
    /// Path to the file, or directory to check
    path: PathBuf,
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Opts { path } = Opts::parse();

    collect_files(&path)
        .iter()
        .for_each(|file| {
            if let Some(origin) = get_downloaded_url(&file) {
                println!("{}\t{}", file.display(), origin);
            }
        });

    Ok(())
}

fn collect_files(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        vec![path.to_path_buf()]
    } else if path.is_dir() {
        walkdir::WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect()
    } else {
        vec![]
    }
}

fn get_downloaded_url(entry: &dyn AsRef<Path>) -> Option<String> {
    if let Ok(Some(attr)) = xattr::get(entry, "com.apple.metadata:kMDItemWhereFroms") {
        if let Ok(val) = plist::Value::from_reader(std::io::Cursor::new(&attr[..])) {
            if let Some(array) = val.as_array() {
                if array.len() == 2 {
                    let origin = array.get(1).unwrap().as_string().unwrap();
                    if !origin.trim().is_empty() {
                        return Some(origin.trim().to_string());
                    }
                }
            }
        }
    }
    None
}
