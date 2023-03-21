use std::env;
use dotenv::*;
use openapi::apis as twilio;

#[tokio::main]
async fn main() {
    dotenv().expect("unable to load environment variables");

    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("unable to retrieve account sid");
    let auth_token = env::var("TWILIO_AUTH_TOKEN").expect("unable to retrieve auth token");

    let mut twilio_config = twilio::configuration::Configuration::new();
    twilio_config.basic_auth = Some((account_sid.clone(), Some(auth_token)));

    let result = match twilio::default_api::list_available_phone_number_mobile(&twilio_config, &account_sid, "GB", None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None).await {
        Ok(res) => res,
        Err(error) => panic!("{:?}", error),
    };

    println!("{:?}", result);
}
