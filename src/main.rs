/*
    DiscordRP - Simple Discord Rich Presence
    Copyright (C) 2018  Jacob Wiltse

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate discord_rpc_client;
extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use std::{thread, time};
use discord_rpc_client::Client as DiscordRPC;
use yaml_rust::{YamlLoader};

fn main() {
    let notice = "    DiscordRP  Copyright (C) 2018  Jacob Wiltse
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.\n";

    println!("{}", notice);

    // Read in YAML config
    let mut f = File::open("config.yaml").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    // yaml-rust supports multiple docs per file, we only use the first one
    let conf = &docs[0];
    // Create DiscordRPC object using app client ID from config
    let mut drpc = DiscordRPC::new(conf["client_id"].as_i64().unwrap() as u64)
        .expect("Failed to create client");

    // Open the RPC connection
    drpc.start();

    println!("Connected to Discord RPC");

    // Current statuses state
    let mut current = 0;

    // Discord imposes a rate limit of one presence update every 15 seconds
    let mut conf_interval = conf["interval"].as_i64().unwrap();
    if conf_interval < 15000 {
        conf_interval = 15000;
    }
    println!("Changing status every {} milliseconds", conf_interval);
    let interval = time::Duration::from_millis(conf_interval as u64);
    loop {
        // Update the users activity using the current status
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
