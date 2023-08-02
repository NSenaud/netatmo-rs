use netatmo_rs::{ClientCredentials, Netatmo, NetatmoClient, Scope};
use std::env;

fn main() {
    env_logger::init();

    let client_id = env::var_os("NETATMO_CLIENT_ID")
        .expect("Environment variable 'NETATMO_CLIENT_ID' is not set.")
        .to_string_lossy()
        .to_string();
    let client_secret = env::var_os("NETATMO_CLIENT_SECRET")
        .expect("Environment variable 'NETATMO_CLIENT_SECRET' is not set.")
        .to_string_lossy()
        .to_string();
    let refresh_token = env::var_os("NETATMO_REFRESH_TOKEN")
        .expect("Environment variable 'NETATMO_REFRESH_TOKEN' is not set.")
        .to_string_lossy()
        .to_string();
    let device_id = env::var_os("NETATMO_DEVICE_ID")
        .expect("Environment variable 'NETATMO_DEVICE_ID' is not set")
        .to_string_lossy()
        .to_string();

    let client_credentials = ClientCredentials {
        client_id: &client_id,
        client_secret: &client_secret,
    };

    let station_data = NetatmoClient::new(&client_credentials)
        .authenticate(&refresh_token)
        .expect("Failed to authenticate")
        .get_station_data(&device_id)
        .expect("Failed to get station data");

    println!("{:#?}", station_data);
}
