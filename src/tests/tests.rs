use crate::travel_scraper::structs::rooms::{GenericRoom, Room};

#[test]
fn test_from_to_hostel() {
    let room_options: Vec<Box<dyn Room>> = vec![Box::new(GenericRoom {
        name: "4 camas".to_string(),
        price: Money::new(4.50, "EUR").unwrap(),
        url: "url".to_string(),
    })];
    let hostel = Hostel {
        name: "Hostelepico".to_string(),
        room_options,
        link: "hostelepico.com".to_string(),
    };

    let city = City {
        name: "Sanca".to_string(),
        ano_url: "sanca.com".to_string(),
        hostels: vec![hostel],
    };
    let ccity = CCity::from_city(&city);
    let old_city = ccity.to_city();
    old_city.print_city();
}
