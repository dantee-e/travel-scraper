use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use serde_json::Value;
use crate::structs::{City, Hostel, Room, money::Money};
use crate::structs::money::Currency;

pub struct ANORequest {
    pub(crate) city: String,
    pub(crate) adults: i8,
    pub(crate) arrival: String,
    pub(crate) departure: String,
    pub(crate) city_url: String,
    pub(crate) date_range: String,
}

impl ANORequest {
    pub fn new(
        city: &City,
        adults: i8,
        arrival: String,
        departure: String,
    ) -> Self {
        let date_range = format!("{} - {}", arrival.clone(), departure);
        Self {
            city: city.name.clone(),
            city_url: city.ano_url.clone(),
            adults,
            arrival,
            departure,
            date_range,
        }
    }
}

fn get_hashmap_cities_to_urls<'a>() -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("Berlin".to_string(), "berlin".to_string());
    hashmap.insert("Frankfurt".to_string(), "frankfurt".to_string());
    hashmap.insert("Nuremberg".to_string(), "nuernberg".to_string());
    hashmap
}

pub fn curry_get_city_name() -> impl FnMut(&String) -> Option<String> {
    let city_name_to_url = get_hashmap_cities_to_urls();
    move |city_name: &String| {
        if let Some(url) = city_name_to_url.get(city_name) {
            return Some(url.to_string());
        }
        None
    }
}

pub async fn make_request_a_and_o(
    request: ANORequest,
) -> Result<serde_json::Value, reqwest::Error> {

    // Criar um cliente HTTP
    let client = Client::builder()
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .build()?;

    // Definir os headers
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("*/*"));
    headers.insert("accept-encoding", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.8"));
    headers.insert("content-type", HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
    headers.insert("origin", HeaderValue::from_static("https://www.aohostels.com"));
    headers.insert("priority", HeaderValue::from_static("u=1, i"));
    
    // From my tests, this information is not needed, but I'll leave it here anyway just in case
    // headers.insert("referer", HeaderValue::from_static("https://www.aohostels.com/de/berlin/"));
    // headers.insert("sec-ch-ua", HeaderValue::from_static("\"Brave\";v=\"135\", \"Not-A.Brand\";v=\"8\", \"Chromium\";v=\"135\""));
    // headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    // headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Linux\""));
    // headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    // headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    // headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    // headers.insert("sec-gpc", HeaderValue::from_static("1"));
    // headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36"));
    // headers.insert("x-requested-with", HeaderValue::from_static("XMLHttpRequest"));

    let mut form_data = HashMap::new();
    form_data.insert("city", request.city.as_str());
    form_data.insert("city_url", request.city_url.as_str());
    form_data.insert("house", "0");
    form_data.insert("currency", "EUR");
    form_data.insert("studytrips", "");
    form_data.insert("daterange", request.date_range.as_str());
    form_data.insert("arrival", request.arrival.as_str());
    form_data.insert("departure", request.departure.as_str());
    form_data.insert("adults", "2");
    form_data.insert("childs", "0");
    form_data.insert("child1", "0");
    form_data.insert("child2", "0");
    form_data.insert("child3", "0");
    form_data.insert("child4", "0");

    // Fazer a requisição POST
    let response = client
        .post("https://www.aohostels.com/de/test/city/")
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
    hostel_json: serde_json::Value,
) -> Option<Hostel> {
    if hostel_json["availability"].as_i64().unwrap() > 0 {
        let hostel_name = hostel_json["name"]
            .clone()
            .as_str()
            .unwrap()[3..]
            .to_string();

        let currency: Currency = if let Value::String(cur) =  hostel_json["currency"].clone() {
            match cur.as_str() {
                "EUR" => Currency::Euro,
                "DOL" => Currency::Dollar,
                _ => Currency::None,
            }
        } else { Currency::None };

        let mut hostel = Hostel::new(hostel_name, hostel_json["link"].to_string());

        let room_types = hostel_json["categories"].as_array();
        if let Some(rooms) = room_types {
            for room in rooms {
                if room["availability"].as_i64().unwrap() > 1 {
                    let lowest_price = room["lowest_price"].clone();
                    let total_price = room["total_price"].clone();
                    hostel.add_room_option(
                        Room::new(
                            room["name"].to_string(),
                            Money::from_json_number(lowest_price, currency.clone()).unwrap(),
                            Money::from_json_number(total_price, currency.clone()).unwrap(),
                            room["name"].to_string()
                        )
                    );
                }
            }
        };

        return Some(hostel)
    }
    None
}

pub async fn request_a_and_o(
    a_n_o_request: ANORequest,
) -> Option<Vec<Hostel>> {
    let request_result = match make_request_a_and_o(a_n_o_request).await {
        Ok(res) => res,
        Err(e) => {
            println!("Error in the request_a_and_o function: {e}");
            return None;
        }
    };
    let mut hostels: Vec<Hostel> = Vec::new();


    if let Some(hostels_json) = request_result.as_array() {
        for hostel_json in hostels_json {
            if let Some(hostel) = get_from_hostel(hostel_json.clone()){
                hostels.push(hostel)
            };
        }
    }

    if hostels.iter().len() == 0 {
        return None
    }

    Some(hostels)
}
