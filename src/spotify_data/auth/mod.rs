use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Serialize, Default, Debug, Deserialize)]
pub struct AuthDetails {
    access_token: String,
    refresh_token: String,
}

pub async fn authenticate_user() -> AuthDetails {
    // The credentials must be available in the environment. Enable the
    // `env-file` feature in order to read them from an `.env` file.
    let creds = Credentials::from_env().unwrap();

    // Using every possible scope
    let scopes = scopes!(
        "user-read-email",
        "user-read-private",
        "user-top-read",
        "user-read-recently-played",
        "user-follow-read",
        "user-library-read",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "playlist-read-collaborative",
        "playlist-read-private",
        "user-follow-modify",
        "user-library-modify",
        "user-modify-playback-state",
        "playlist-modify-public",
        "playlist-modify-private",
        "ugc-image-upload"
    );
    let oauth = OAuth::from_env(scopes).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).await.unwrap();

    let token = spotify.token.lock().await.unwrap();

    println!("Access token: {}", &token.as_ref().unwrap().access_token);
    println!(
        "Refresh token: {}",
        token.as_ref().unwrap().refresh_token.as_ref().unwrap()
    );

    AuthDetails {
        access_token: token.as_ref().unwrap().access_token.clone(),
        refresh_token: token
            .as_ref()
            .unwrap()
            .refresh_token
            .as_ref()
            .unwrap()
            .clone(),
    }
}

// TODO: Check if there are existing tokens before writing to a file
pub fn write_to_toml(info: AuthDetails, path: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .read(true)
        .open(path)
        .unwrap();
    let toml_string = toml::to_string(&info).unwrap();
    file.write_all(toml_string.as_bytes()).unwrap();

}
