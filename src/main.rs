extern crate discord_rpc_client;
extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use std::{thread, time};
use discord_rpc_client::Client as DiscordRPC;
use yaml_rust::{YamlLoader};

fn main() {
    let mut f = File::open("config.yaml").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let conf = &docs[0];
    let mut drpc = DiscordRPC::new(conf["client_id"].as_i64().unwrap() as u64)
        .expect("Failed to create client");

    drpc.start();

    println!("Connected to Discord RPC");

    let mut current = 0;
    let mut conf_interval = conf["interval"].as_i64().unwrap();
    if conf_interval < 15000 {
        conf_interval = 15000;
    }
    println!("Changing status ever {} milliseconds", conf_interval);
    let interval = time::Duration::from_millis(conf_interval as u64);
    loop {
        if let Err(why) = drpc.set_activity(|a| a
            .state(conf["statuses"][current]["state"].as_str().unwrap())
            .details(conf["statuses"][current]["details"].as_str().unwrap())
            .assets(|ass| ass
                .large_image(conf["statuses"][current]["assets"]["large_image"].as_str().unwrap())
                .large_text(conf["statuses"][current]["assets"]["large_text"].as_str().unwrap())
                .small_image(conf["statuses"][current]["assets"]["small_image"].as_str().unwrap())
                .small_text(conf["statuses"][current]["assets"]["small_text"].as_str().unwrap())))
            {
                println!("Failed to set presence: {}", why);
            }
        current += 1;
        if current == conf["statuses"].as_vec().unwrap().len() {
            current = 0;
        }
        thread::sleep(interval);
    }
}
