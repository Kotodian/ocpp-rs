use std::env;

use ws::connect;

pub mod client;
pub mod evses;
pub mod transactions;

#[derive(Debug)]
struct Config {
    csms_url: String,
    station_id: String,
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let csms_url = match env::var("CSMS_URL") {
        Ok(var) => var,
        Err(e) => panic!("Couldn't read CSMS_URL ({})", e),
    };

    let station_id = match env::var("STATION_ID") {
        Ok(var) => var,
        Err(e) => panic!("Couldn't read STATION_ID ({})", e),
    };

    let config = Config {
        csms_url,
        station_id,
    };

    // todo log

    let mut connection_string: String = config.csms_url.to_owned();
    connection_string.push_str("/");
    connection_string.push_str(&config.station_id);

    connect(connection_string, |out| client::Client { out: out }).unwrap()
}
