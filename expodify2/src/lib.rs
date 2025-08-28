use std::error::Error;
use std::fmt::Display;
use std::{fmt, fs, io};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use audiotags::Tag;
use log::{debug, info, warn};
use rand::distr::Alphanumeric;
use rand::Rng;

/// The paths to search for media in
const SEARCH_PATHS: &[&str] = &[
    // This works for most iPods
    "iPod_Control/Music",
    // I think this is only present on iTunes compatible Rokr phones
    "iTunes_Control/iPod_Control/Music",
];

/// Extract all media from a device and copy it to the destination
pub fn extract<P: AsRef<Path>>(source: P, destination: P) -> Result<(), ExtractError> {
    info!("Extracting from \"{}\" to \"{}\"", source.as_ref().display(), destination.as_ref().display());

    let mut found_media_dir = false;

    for search_path in SEARCH_PATHS {
        let media_dir = source.as_ref().join(search_path);
        debug!("Checking if \"{}\" exists", media_dir.display());
        if media_dir.exists() {
            found_media_dir = true;
            info!("Found media directory at: {}", search_path);

            // Get the F00, F01, F02, etc folders
            let contents = media_dir.read_dir().map_err(ExtractError::IoError)?;

            // Read each F folder
            for f_folder in contents {
                let f_folder = f_folder.map_err(ExtractError::IoError)?;

                // If it isn't a directory then just continue to the next thing in the media directory
                if !f_folder.metadata().map_err(ExtractError::IoError)?.is_dir() {
                    continue;
                }

                // Loop over each file in each F folder
                for file in fs::read_dir(f_folder.path()).map_err(ExtractError::IoError)? {
                    let file = file.map_err(ExtractError::IoError)?;

                    let file_name = file.file_name().to_string_lossy().into_owned();

                    if !file.metadata().map_err(ExtractError::IoError)?.is_file() {
                        continue;
                    }

                    info!("Extracting file \"{}\"", file.path().display());

                    let dest_name;
                    let tag = Tag::new().read_from_path(file.path());
                    if let Err(err) = tag {
                        warn!("Error reading media metadata (using original filename): {}", err);
                        dest_name = file_name;
                    } else {
                        let tag = tag.unwrap();
                        let title = tag.title().map(|title| title.to_string()).unwrap();
                        dest_name = title + "." + Path::new(&file_name)
                            .extension()
                            .and_then(OsStr::to_str)
                            .unwrap_or("");
                    }

                    let mut full_destination = destination.as_ref().join(&dest_name);

                    if fs::exists(&full_destination).map_err(ExtractError::IoError)? {
                        let new_file_name = rand::rng()
                            .sample_iter(&Alphanumeric)
                            .take(5)
                            .map(char::from)
                            .collect::<String>() + "_" + &dest_name;

                        warn!("\"{}\" already exists, it will be saved as \"{}\" instead", full_destination.file_name().unwrap().display(), new_file_name);
                        full_destination.set_file_name(new_file_name);
                    }

                    fs::copy(file.path(), full_destination).map_err(ExtractError::IoError)?;
                }
            }
        }
    }

    if !found_media_dir {
        return Err(ExtractError::NoMediaDirFound(SEARCH_PATHS));
    }

    info!("Finished extracting");
    Ok(())
}

#[derive(Debug)]
pub enum ExtractError {
    IoError(io::Error),
    NoMediaDirFound(&'static [&'static str]),
}

impl Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractError::IoError(err) => write!(f, "{}", err),
            ExtractError::NoMediaDirFound(search_paths) => write!(f, "No media directory found after searching these paths: {:?}", search_paths),
        }
    }
}

impl Error for ExtractError {}