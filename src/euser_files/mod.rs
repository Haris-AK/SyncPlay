use audiotags;
use kdam::tqdm;
use std::fs::read_dir;
use std::path::PathBuf;

// TODO: Clean up conversion of strings and unwraps
pub fn get_list(file_directory: &String) -> Vec<MusicFile> {
    let res = get_files(&file_directory).unwrap();
    let res = get_metadata(res);
    return res;
}

#[derive(Debug)]
pub struct MusicFile {
    pub name: String,
    pub path: PathBuf,
}
fn get_metadata(file_vec: Vec<PathBuf>) -> Vec<MusicFile> {
    println!("Generating a list of songs in your library");
    let mut metadata_list: Vec<MusicFile> = vec![];
    for i in tqdm!(file_vec.iter()) {
        let tag = audiotags::Tag::default().read_from_path(&i).unwrap();
        let title = tag.title().unwrap();
        let x = MusicFile {
            name: String::from(title),
            path: i.to_owned(),
        };
        metadata_list.push(x)
    }
    metadata_list
}

fn get_files(path: &String) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut file_list: Vec<PathBuf> = Vec::new();
    // TODO: Make this more of a 'pure' function by not using the traverse_directory function as a
    // side effect, but instead take in the path and return a vector of paths
    traverse_directory(path, &mut file_list);
    Ok(file_list)
}

fn traverse_directory(path: &String, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    let entries = read_dir(path)?;

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
