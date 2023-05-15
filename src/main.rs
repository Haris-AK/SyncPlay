mod user_files;
use user_files::get_list;

fn main() {
    let x = get_list();
    println!("{:?}", x);
}

