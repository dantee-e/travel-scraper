use std::collections::HashMap;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use crate::structs::{City, Hostel};
use crate::structs::money::{Currency, Money};
use crate::structs::rooms::RoomHostelClub;

pub struct HostelsClub {
    pub guests: String,
    pub checkin: String,
    pub checkout: String,
    pub city_slug: String,
    pub currency: Currency

}
impl HostelsClub {
    pub fn new(
        city: &City,
        guests: u8,
        checkin: String,
        checkout: String,
        currency: Currency
    ) -> Self {
        Self {
            city_slug: city.ano_url.clone(),
            guests: guests.to_string(),
            checkin,
            checkout,
            currency,
        }
    }
}




async fn get_partner_auth_token(client: &Client) -> Result<String, reqwest::Error> {

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_static("api.hostelsclub.com"));
    headers.insert("App_secret", HeaderValue::from_static("Z2PEwCAd5FbEVkb3qVyv"));
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Linux\""));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Not.A/Brand\";v=\"99\", \"Chromium\";v=\"136\""));
    headers.insert("App_id", HeaderValue::from_static("hostelsclub-production"));
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert(
        "User-Agent",
        HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36"),
    );
    headers.insert("Accept", HeaderValue::from_static("application/json, text/*"));
    headers.insert("Origin", HeaderValue::from_static("https://www.hostelsclub.com"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Referer", HeaderValue::from_static("https://www.hostelsclub.com/"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("Priority", HeaderValue::from_static("u=1, i"));

    let response = client
        .get("https://api.hostelsclub.com/v1/destinations?term=Berlin&lang=en")
        .headers(headers)
        .send()
        .await?;

    let header_name = "partner-auth-token";

    match response.headers().get(header_name) {
        Some(header_value) => {
            let header_value = header_value.to_str().unwrap();
            Ok(header_value.to_string())
        }
        None => {
            panic!("Could not find header '{}'", header_name);
        }
    }

}

async fn make_request_hostel_club(
    request: HostelsClub,
) -> Result<Value, reqwest::Error> {
    // Criar um cliente HTTP
    let client = Client::builder()
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .build()?;

    let partner_auth_token = get_partner_auth_token(&client).await?;
    let currency = match request.currency {
        Currency::Dollar => {"DOL"}
        Currency::Euro => {"EUR"}
        Currency::None => panic!("No currency provided!")
    };

    // Definir os headers
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("application/json, text/*"));
    headers.insert("accept-encoding", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.6"));
    headers.insert("content-type", HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
    headers.insert("origin", HeaderValue::from_static("https://www.hostelsclub.com"));
    headers.insert("priority", HeaderValue::from_static("u=1, i"));
    headers.insert("partner-auth-token", HeaderValue::from_str(partner_auth_token.as_str()).unwrap());
    
    let mut form_data = HashMap::new();
    form_data.insert("checkin", request.checkin.as_str());
    form_data.insert("checkout", request.checkout.as_str());
    form_data.insert("city_slug", request.city_slug.as_str());
    form_data.insert("currency", currency);
    form_data.insert("guests", request.guests.as_str());
    form_data.insert("room_type", "0");
    form_data.insert("rooms", "1");

    // Fazer a requisicao POST
    let response = client
        .post("https://api.hostelsclub.com/v1/properties/search?lang=de")
        .headers(headers)
        .form(&form_data)
        .send()
        .await?;

    // Obter o status e o corpo da resposta
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        println!("Error: {}", status);
    }
    let body = response.text().await?;
    

    Ok(serde_json::from_str(body.as_str()).unwrap())
}


fn get_from_hostel(
    hostel_json: Value,
    currency: &Currency,
) -> Option<Hostel> {
    let hostel_name = hostel_json["name"]
        .clone()
        .to_string();


    let mut hostel = Hostel::new(hostel_name, hostel_json["link"].to_string());

    let room_types = hostel_json["rooms"].as_array();
    if let Some(rooms) = room_types {
        for room in rooms {
            if room["availability"].as_i64().unwrap() > 1 {
                let price = room["price"].clone();
                let beds = match room["beds"].clone() {
                    Value::String(beds_str) => {
                        beds_str.parse::<u8>().unwrap_or(0)
                    },
                    Value::Number(beds_n) => {
                        beds_n.as_u64().unwrap_or(0) as u8
                    }
                    _ => {
                        println!("Could not convert the number of beds, returning 0");
                        0
                    }
                };
                hostel.add_room_option(
                    Box::new(
                        RoomHostelClub::new(
                            room["name"].to_string(),
                            Money::from_json_number(price, currency.clone()).unwrap(),
                            beds,
                        )
                    )
                );
            }
        }
    };
    if hostel.room_options.is_empty(){
        return None
    }
    Some(hostel)

}

pub async fn request_hostelworld(req: HostelsClub) -> Option<Vec<Hostel>> {
    let currency = req.currency.clone();
    let request_result = match make_request_hostel_club(req).await {
        Ok(res) => res,
        Err(e) => {
            println!("Error in the request_a_and_o function: {e}");
            return None;
        }
    };
    let mut hostels: Vec<Hostel> = Vec::new();


    if let Some(hostels_json) = request_result.as_array() {
        for hostel_json in hostels_json {
            if let Some(hostel) = get_from_hostel(hostel_json.clone(), &currency){
                hostels.push(hostel)
            };
        }
    }

    if hostels.iter().len() == 0 {
        return None
    }

    Some(hostels)
}

pub async fn get_all_hostel_world() -> Option<Vec<City>> {
    let mut cities: Vec<City> = Vec::new();
    let cities_names = vec![
        "Berlin".to_string(),
        "Dresden".to_string(),
        "Munich".to_string(),
        "Frankfurt".to_string(),
        "Nuremberg".to_string(),
        "Bremen".to_string(),
        "Hamburg".to_string(),
        "Hanover".to_string(),
        "Cologne".to_string(),
    ];

    let mut city_urls = super::hashmaps_cities::get_hashmap_cities_to_urls_hostel_world();


    // Adds the URLs to the cities structs
    for city in cities_names {
        let url = city_urls.get(&city).unwrap();
        cities.push( City::new(city, url.clone()))
    }

    for city in &mut cities {
        match request_hostelworld(HostelsClub::new(
            &city,
            2,
            "08.06.2025".to_string(),
            "09.06.2025".to_string(),
            Currency::Euro,
        )).await {
            Some(result) => { city.add_hostels(result) }
            None => { println!("Could not find available hostels in {}", city.name) }
        };
    }

    Some(cities)
}