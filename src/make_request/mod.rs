use std::collections::HashMap;
pub mod a_and_o;
pub mod hostelsclub;
mod hashmaps_cities;

fn get_hashmap_cities_to_urls_a_and_o<'a>() -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("Aachen".to_string(), "aachen".to_string());
    hashmap.insert("Antwerpen".to_string(), "antwerpen".to_string());
    hashmap.insert("Berlin".to_string(), "berlin".to_string());
    hashmap.insert("Bremen".to_string(), "bremen".to_string());
    hashmap.insert("Brighton".to_string(), "brighton".to_string());
    hashmap.insert("Brüssel".to_string(), "bruessel".to_string());
    hashmap.insert("Budapest".to_string(), "budapest".to_string());
    hashmap.insert("Dortmund".to_string(), "dortmund".to_string());
    hashmap.insert("Dresden".to_string(), "dresden".to_string());
    hashmap.insert("Düsseldorf".to_string(), "duesseldorf".to_string());
    hashmap.insert("Edinburgh".to_string(), "edinburgh".to_string());
    hashmap.insert("Florenz".to_string(), "florenz".to_string());
    hashmap.insert("Frankfurt".to_string(), "frankfurt".to_string());
    hashmap.insert("Graz".to_string(), "graz".to_string());
    hashmap.insert("Hamburg".to_string(), "hamburg".to_string());
    hashmap.insert("Köln".to_string(), "koeln".to_string());
    hashmap.insert("Kopenhagen".to_string(), "kopenhagen".to_string());
    hashmap.insert("Leipzig".to_string(), "leipzig".to_string());
    hashmap.insert("Mailand".to_string(), "mailand".to_string());
    hashmap.insert("München".to_string(), "muenchen".to_string());
    hashmap.insert("Nürnberg".to_string(), "nuernberg".to_string());
    hashmap.insert("Prag".to_string(), "prag".to_string());
    hashmap.insert("Rotterdam".to_string(), "rotterdam".to_string());
    hashmap.insert("Salzburg".to_string(), "salzburg".to_string());
    hashmap.insert("Stuttgart".to_string(), "stuttgart".to_string());
    hashmap.insert("Venedig".to_string(), "venedig".to_string());
    hashmap.insert("Warschau".to_string(), "warschau".to_string());
    hashmap.insert("Weimar".to_string(), "weimar".to_string());
    hashmap.insert("Wien".to_string(), "wien".to_string());
    hashmap
}

pub fn curry_get_city_url_a_and_o() -> impl FnMut(&String) -> Option<String> {
    let city_name_to_url = get_hashmap_cities_to_urls_a_and_o();
    move |city_name: &String| {
        if let Some(url) = city_name_to_url.get(city_name) {
            return Some(url.to_string());
        }
        None
    }
}

