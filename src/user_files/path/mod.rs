use config::Config;
use std::collections::HashMap;

// TODO: Change use of hashmap 
pub fn get_path() -> HashMap<String, String> {
    let path = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("./Config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("path"));
    let path = path
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    return path;
}
