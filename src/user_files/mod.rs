use audiotags;
use kdam::tqdm;
use std::fs;
use std::path::{Path, PathBuf};
mod path;

// TODO: Clean up conversion of strings and unwraps
pub fn get_list() -> Vec<String> {
    let path = path::get_path();
    let path = path.get("path").unwrap();
    let res = get_files(&path).unwrap();
    let res = get_metadata(res);
    return res
}

fn get_metadata(file_vec: Vec<PathBuf>) -> Vec<String> {
    println!("Generating a list of songs in your library");
    let mut metadata_list = vec![];
    for i in tqdm!(file_vec.iter()) {
        let tag = audiotags::Tag::default().read_from_path(&i).unwrap();
        let title = tag.title().unwrap();
        metadata_list.push(String::from(title))
    }
    metadata_list
}

fn get_files(path: &String) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut file_list: Vec<PathBuf> = Vec::new();

    traverse_directory(path, &mut file_list);

    Ok(file_list)
}

fn traverse_directory(path: &String, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            let file_extension = file_path.extension().unwrap().to_str().unwrap();
            if is_supported_audio_file(file_extension) {
                files.push(file_path);
            }
        } else if file_path.is_dir() {
            let file_path = file_path.into_os_string().into_string().unwrap();
            traverse_directory(&file_path, files)?;
        }
    }

    Ok(())
}

fn is_supported_audio_file(file: &str) -> bool {
    let supported_extensions = ["flac", "mp3"];
    supported_extensions.contains(&file)
}
