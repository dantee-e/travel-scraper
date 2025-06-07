use url::Url;
use crate::structs::money::Money;

pub trait Room {
    fn print_room(&self);
    
    fn get_room(&self) -> String;
}

#[derive(Clone)]
pub struct RoomAnO {
    name: String,
    lowest_price: Money,
    total_price: Money,
    url: Url
}

impl RoomAnO {
    pub fn new(
        name: String,
        lowest_price: Money,
        total_price: Money,
        url: String
    )-> RoomAnO {
        Self {
            name,
            lowest_price,
            total_price,
            url: Url::parse("https://www.google.de/maps").unwrap()
        }
    }
}

impl Room for RoomAnO {
    fn print_room(&self) {
        println!("This is the {} room, with the lowest price of {} and total of {}",
             self.name, self.lowest_price, self.total_price);
    }
    
    fn get_room(&self) -> String {
        format!("This is the {} room, with the lowest price of {} and total of {}",
                self.name, self.lowest_price, self.total_price)
    }
}

#[derive(Clone)]
pub struct RoomHostelClub {
    name: String,
    price: Money,
    beds: u8,
}

impl RoomHostelClub {
    pub fn new(name: String, price: Money, beds: u8) -> RoomHostelClub {
        Self {
            name,
            price,
            beds,
        }        
    }
}
impl Room for RoomHostelClub {
    fn print_room(&self) {
        println!("This is the {} room, with the price of {}", self.name, self.price);
    }
    fn get_room(&self) -> String {
        format!("This is the {} room, with the price of {}", self.name, self.price)
    }
}