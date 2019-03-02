use std::fs;
use std::io;
use std::path::{ Path, PathBuf };
use std::vec::Vec;

#[derive(Debug)]
enum GetNonzeroLengthError {
    IOError(io::Error),
    ZeroLength
}

#[derive(Debug)]
enum ProcessSourcesError {
    GetNonzeroLengthError(GetNonzeroLengthError)
}

fn read_sources(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let canonical_dir = dir.canonicalize()?;
    let mut source_paths = Vec::new();
    for entry in fs::read_dir(canonical_dir)? {
        let entry = entry?;
        let path = entry.path();
        match path.extension() {
            Some(os_str) => {
                if os_str == "hs" {
                    source_paths.push(path)
                }
            }
            _ => ()
        }
    }
    Ok(source_paths)
}

fn get_nonzero_length(path: &Path)
    -> Result<u64, GetNonzeroLengthError> {
    let metadata = fs::metadata(path).map_err(|e| GetNonzeroLengthError::IOError(e))?;
    if metadata.len() == 0 {
        Err(GetNonzeroLengthError::ZeroLength)
    } else {
        Ok(metadata.len())
    }
}

fn process_sources(paths: &Vec<PathBuf>) -> Result<(), ProcessSourcesError> {
    for path in paths {
        let len = get_nonzero_length(path).map_err(|e| ProcessSourcesError::GetNonzeroLengthError(e))?;
        println!("{:?}", len)
    }
    Ok(())
}

fn main() {
    match read_sources(Path::new("../quivela/src/Quivela")) {
        Ok(paths) => {
            match process_sources(&paths) {
                Ok(()) => println!("Success"),
                Err(e) => println!("Failure: {:?}", e)
            }
        },
        Err(e) => println!("Failure: {:?}", e)
    }
}
