pub mod money;
pub mod rooms;

use rooms::Room;

#[derive(Clone)]
#[repr(C)]
pub struct Hostel {
    pub name: String,
    pub room_options: Vec<Box<dyn Room>>,
    pub(crate) link: String,
}

impl Hostel {
    pub fn new(name: String, link: String) -> Self {
        Self {
            name,
            room_options: vec![],
            link,
        }
    }
    pub fn add_room_option(&mut self, room_option: Box<dyn Room>) {
        self.room_options.push(room_option);
    }

    pub fn print_hostel(&self) {
        println!("Hostel {}:", self.name);
        for room in self.room_options.iter() {
            print!("    ");
            room.print_room();
        }
    }

    pub fn get_hostel(&self) -> String {
        let str = self
            .room_options
            .iter()
            .map(|room| room.get_room_string())
            .collect::<Vec<String>>()
            .join("\n");
        format!("Hostel {}:\n{}", self.name, str)
    }
}

#[derive(Clone)]
#[repr(C)]
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
        self.hostels.append(&mut Box::new(hostels));
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
