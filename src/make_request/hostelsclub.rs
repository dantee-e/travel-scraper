use std::collections::HashMap;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use crate::structs::{Hostel, Room};
use crate::structs::money::{Currency, Money};

pub struct HostelsClub {
    pub guests: String,
    pub checkin: String,
    pub checkout: String,
    pub city_slug: String,
}

pub async fn make_request_hostel_club(
    request: HostelsClub,
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
    headers.insert("accept", HeaderValue::from_static("application/json, text/*"));
    headers.insert("accept-encoding", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.6"));
    headers.insert("content-type", HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
    headers.insert("origin", HeaderValue::from_static("https://www.hostelsclub.com"));
    headers.insert("priority", HeaderValue::from_static("u=1, i"));
    headers.insert("partner-auth-token", HeaderValue::from_static(
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkYXRhIjp7ImF1dGhfaGVhZGVycyI6eyJhcGktdXNlcm5hbWUiOiIyMzQzIiwiYXBpLXBhc3N3b3JkIjoiSEMyMDE4TVZsYWJzIiwiYXBpLWVudiI6IlByb2R1Y3Rpb24ifX0sImlhdCI6MTc0OTIyNzQwNiwiZXhwIjoxNzQ5MjMxMDA2fQ.ZA07RakdwNiDnr7KrC64FhmhLni-SfRq-VXdsXM3QOQ"
    ));
    
    
    


    let mut form_data = HashMap::new();
    form_data.insert("checkin", request.checkin.as_str());
    form_data.insert("checkout", request.checkout.as_str());
    form_data.insert("city_slug", request.city_slug.as_str());
    form_data.insert("currency", "EUR");
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
    hostel_json: serde_json::Value,
) -> Option<Hostel> {
    let hostel_name = hostel_json["name"]
        .clone()
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
    todo!();

}

pub async fn request_hostelworld(req: HostelsClub) -> Option<Vec<Hostel>> {
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