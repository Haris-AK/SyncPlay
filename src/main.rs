mod user_config;

#[tokio::main]
async fn main() {
    // let config: Config = read_config();
    // let x = get_list(&config.path);
    // generate_playlist(&x);
    // let info = auth::authenticate_user().await;
    // auth::write_to_toml(info, "Config.toml");
    // let var_name = vec![1, 2, 3, 4];
    // let x = var_name.into_iter();
    // for i in x {
    //     let var_name = println!("{:#?}", i);
    //     var_name
    // }
    let mut my_vec: Vec<i64> = Vec::new();
    for number in 1..100000000 {
        my_vec.push(number)
    }
    let my_new_var: Vec<i64> = my_vec.iter().map(|r| r * *r).collect();
    for res in my_new_var {
        println!("{res}")
    }
}
