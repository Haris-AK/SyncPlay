use audiotags;
use std::fs;
use std::path::{Path, PathBuf};
use kdam::tqdm;
use config::Config;

fn main() {
    
    let path = Path::new("");
    let res = get_files(path).unwrap();
    println!("{:?}", get_metadata(res))
}

fn get_metadata(file_vec: Vec<PathBuf>) -> Vec<String> {
    println!("Generating a list of songs in your library");
    let mut metadata_list = vec![];
    for i in tqdm!(file_vec.iter()) {
        let tag = audiotags::Tag::default().read_from_path(&i).unwrap();
        let title = tag.title().unwrap();
        metadata_list.push(String::from(title))
    };
    metadata_list
}

fn get_files(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut file_list: Vec<PathBuf> = Vec::new();

    traverse_directory(path, &mut file_list);

    Ok(file_list)
}

fn traverse_directory(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
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
            traverse_directory(&file_path, files)?;
        }
    }

    Ok(())
}

fn is_supported_audio_file(file: &str) -> bool {
    let supported_extensions = ["flac", "mp3"];
    supported_extensions.contains(&file)
}
