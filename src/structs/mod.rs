pub mod money;

use money::Money;

pub struct Room {
    name: String,
    lowest_price: Money,
    total_price: Money,
}


impl Room {
    pub fn new(
        name: String,
        lowest_price: Money,
        total_price: Money,
        url: String
    )-> Room {
        Self {
            name,
            lowest_price,
            total_price,
        }
    }
    
    pub fn print_room(&self) {
        println!("This is the {} room, with the lowest price of {} and total of {}", 
                 self.name, self.lowest_price, self.total_price);
    }
}

pub struct Hostel {
    pub name: String,
    pub room_options: Vec<Room>,
    link: String,
}

impl Hostel {
    pub fn new(name: String, link: String) -> Self {
        Self {
            name,
            room_options: vec![],
            link
        }
    }
    pub fn add_room_option(&mut self, room_option: Room) {
        self.room_options.push(room_option);
    }
    
    pub fn print_hostel(&self) {
        println!("Hostel {}:", self.name);
        for room in self.room_options.iter() {
            print!("    ");
            room.print_room();
        }
    }
}

pub struct City {
    pub name: String,
    pub ano_url: String,
    pub hostels: Vec<Hostel>,
}

impl City {
    pub fn new(name: String, ano_url: String) -> Self {
        Self {
            name,
            ano_url,
            hostels: Vec::new(),
        }
    }
    pub fn add_hostels(&mut self, mut hostels: Vec<Hostel>) {
        self.hostels.append(&mut hostels);
    }
    
    pub fn print_city(&self) {
        println!("City {}:", self.name);
        for hostel in self.hostels.iter() {
            hostel.print_hostel();
        }
    }
}

#[allow(unused_variables)]
pub struct Country {
    pub cities: Vec<City>,
}