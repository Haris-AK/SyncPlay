mod user_files;
use user_files::get_list;
mod playlist;
use playlist::generate_playlist;
mod spotify_data;
use spotify_data::auth;
mod user_config;
use user_config::{read_config, Config};

#[tokio::main]
async fn main() {
    let config: Config = read_config();
    let x = get_list(&config.path);
    generate_playlist(&x);
    let info = auth::authenticate_user().await;
    auth::write_to_toml(info, "Config.toml");
}
