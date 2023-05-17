mod user_files;
use user_files::{get_list, write_list_to_file};
mod spotify_data;

fn main() {
    let x = get_list();
    let y = x.clone();
    write_list_to_file(&y);
    println!("{:?}", y);
}

