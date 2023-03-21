use std::env;
use dotenv::*;
use openapi::{apis as twilio, models::ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberMobile};

struct ListAvailablePhoneNumberMobileParams {
    area_code: Option<i32>, 
    contains: Option<&'static str>, 
    sms_enabled: Option<bool>, 
    mms_enabled: Option<bool>, 
    voice_enabled: Option<bool>, 
    exclude_all_address_required: Option<bool>, 
    exclude_local_address_required: Option<bool>, 
    exclude_foreign_address_required: Option<bool>, 
    beta: Option<bool>, 
    near_number: Option<&'static str>, 
    near_lat_long: Option<&'static str>, 
    distance: Option<i32>, 
    in_postal_code: Option<&'static str>, 
    in_region: Option<&'static str>, 
    in_rate_center: Option<&'static str>, 
    in_lata: Option<&'static str>, 
    in_locality: Option<&'static str>, 
    fax_enabled: Option<bool>, 
    page_size: Option<i32>, 
    page: Option<i32>, 
    page_token: Option<&'static str>,
}

impl ListAvailablePhoneNumberMobileParams {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for ListAvailablePhoneNumberMobileParams {
    fn default() -> Self {
        ListAvailablePhoneNumberMobileParams { area_code: None, contains: None, sms_enabled: None, mms_enabled: None, voice_enabled: None, exclude_all_address_required: None, exclude_local_address_required: None, exclude_foreign_address_required: None, beta: None, near_number: None, near_lat_long: None, distance: None, in_postal_code: None, in_region: None, in_rate_center: None, in_lata: None, in_locality: None, fax_enabled: None, page_size: None, page: None, page_token: None }
    }
}

async fn list_available_phone_number_mobile(config: &twilio::configuration::Configuration, p: &ListAvailablePhoneNumberMobileParams) -> Vec<String> {
   
    let account_id = &config.basic_auth.as_ref().unwrap().0;
    let result = match twilio::default_api::list_available_phone_number_mobile(config, account_id, "GB", 
        p.area_code, p.contains, p.sms_enabled, p.mms_enabled, p.voice_enabled, p.exclude_all_address_required, p.exclude_local_address_required, 
        p.exclude_foreign_address_required, p.beta, p.near_number, p.near_lat_long, p.distance, p.in_postal_code, p.in_region, p.in_rate_center, 
        p.in_lata, p.in_locality, p.fax_enabled, p.page_size, p.page, p.page_token).await {
        Ok(res) => res,
        Err(error) => panic!("{:?}", error),
    };

    let numbers: &Vec<ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberMobile> = &(result.available_phone_numbers).unwrap();

    numbers.iter().map(|x| x.clone().friendly_name.unwrap().unwrap()).collect::<Vec<String>>()
}

#[tokio::main]
async fn main() {
    dotenv().expect("unable to load environment variables");

    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("unable to retrieve account sid");
    let auth_token = env::var("TWILIO_AUTH_TOKEN").expect("unable to retrieve auth token");

    let mut twilio_config = twilio::configuration::Configuration::new();
    twilio_config.basic_auth = Some((account_sid.clone(), Some(auth_token)));

    let res = list_available_phone_number_mobile(&twilio_config, &ListAvailablePhoneNumberMobileParams::new()).await;
    res.iter().for_each(|s| println!("{s}"));
}
