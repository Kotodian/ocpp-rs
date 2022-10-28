use std::{env, time::Duration};

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

    let mut connection_string: String = config.csms_url.to_owned();
    connection_string.push_str("/");
    connection_string.push_str(&config.station_id);
    let connection_string = connection_string.as_str();
    // loop {
    let exponential_back_off = backoff::ExponentialBackoffBuilder::default()
        .with_initial_interval(Duration::new(30, 0))
        .with_randomization_factor(0.5)
        .with_max_interval(Duration::new(5 * 60, 0))
        .build();
    let _ = backoff::retry(exponential_back_off, || {
        connect(connection_string, |out| client::Client { out }).map_err(backoff::Error::transient)
    });
    // }
}
