mod user_files;
// use user_files::get_list;
mod user_config;

#[tokio::main]
async fn main() {
    // let config: Config = read_config();
    // let x = get_list(&config.path);
    // generate_playlist(&x);
    // let info = auth::authenticate_user().await;
    // auth::write_to_toml(info, "Config.toml");
    let x = user_files::get_list(&String::from("/run/media/hak/16TB HDD/Main Music/"));
    for i in x {
        println!("{:#?}", i)
    }
    let x = vec![1, 2, 3, 4].into_iter();
}
