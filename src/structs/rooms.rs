use crate::structs::money::Money;

pub enum RoomType {
    ANO,
    HostelClub,
}

pub trait Room: Send {
    fn print_room(&self);

    fn get_room_string(&self) -> String;

    fn get_room(&self) -> &dyn Room
    where
        Self: Sized,
    {
        self
    }

    fn get_name(&self) -> String;

    fn get_price(&self) -> String;

    fn get_url(&self) -> String;

    fn clone_box(&self) -> Box<dyn Room>;

    fn is_type(&self) -> RoomType;
}
impl Clone for Box<dyn Room> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct RoomAnO {
    pub(crate) name: String,
    pub(crate) lowest_price: Money,
    pub(crate) total_price: Money,
    pub(crate) url: String,
}

impl RoomAnO {
    pub fn new(name: String, lowest_price: Money, total_price: Money, url: String) -> RoomAnO {
        Self {
            name,
            lowest_price,
            total_price,
            url,
        }
    }
}

impl Room for RoomAnO {
    fn print_room(&self) {
        println!(
            "This is the {} room, with the lowest price of {} and total of {}",
            self.name, self.lowest_price, self.total_price
        );
    }
    fn get_room_string(&self) -> String {
        format!(
            "This is the {} room, with the lowest price of {} and total of {}",
            self.name, self.lowest_price, self.total_price
        )
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn clone_box(&self) -> Box<dyn Room> {
        Box::new(self.clone())
    }

    fn is_type(&self) -> RoomType {
        RoomType::ANO
    }

    fn get_price(&self) -> String {
        format!("{}", self.total_price)
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct RoomHostelClub {
    pub(crate) name: String,
    pub(crate) price: Money,
    pub(crate) beds: u8,
}

impl RoomHostelClub {
    pub fn new(name: String, price: Money, beds: u8) -> RoomHostelClub {
        Self { name, price, beds }
    }
}
impl Room for RoomHostelClub {
    fn print_room(&self) {
        println!(
            "This is the {} room, with the price of {}",
            self.name, self.price
        );
    }
    fn get_room_string(&self) -> String {
        format!(
            "This is the {} room, with the price of {}",
            self.name, self.price
        )
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn clone_box(&self) -> Box<dyn Room> {
        Box::new(self.clone())
    }

    fn is_type(&self) -> RoomType {
        RoomType::HostelClub
    }

    fn get_price(&self) -> String {
        format!("{}", self.price)
    }

    fn get_url(&self) -> String {
        String::new()
    }
}

#[derive(Clone)]
pub struct GenericRoom {
    pub name: String,
    pub price: Money,
    pub url: String,
}
impl Room for GenericRoom {
    fn print_room(&self) {
        println!(
            "This is the {} room, with the price of {}",
            self.name, self.price
        );
    }
    fn get_room_string(&self) -> String {
        format!(
            "This is the {} room, with the price of {}",
            self.name, self.price
        )
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_type(&self) -> RoomType {
        RoomType::HostelClub
    }

    fn get_price(&self) -> String {
        format!("{}", self.price)
    }

    fn get_url(&self) -> String {
        String::new()
    }

    fn clone_box(&self) -> Box<dyn Room> {
        Box::new(self.clone())
    }
}
