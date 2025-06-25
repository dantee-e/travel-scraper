use super::string_to_const_char_ptr;
use crate::structs::money::Money;
use crate::structs::rooms::Room;
use crate::structs::{self, rooms::GenericRoom};
use std::ffi::{CStr, c_char, c_uint};
use std::str::Utf8Error;

#[derive(Debug)]
pub enum RoomError {
    Utf8(Utf8Error),
    InvalidPriceFormat(String),
}

#[repr(C)]
#[derive(Clone)]
pub struct CRoom {
    pub name: *const c_char,
    pub price: *const c_char,
    pub url: *const c_char,
    pub a_n_o_hostel_club: c_uint,
}
impl CRoom {
    pub fn from_room(room: &Box<dyn Room>) -> CRoom {
        let room_type: c_uint = match room.is_type() {
            structs::rooms::RoomType::ANO => 0,
            structs::rooms::RoomType::HostelClub => 1,
        };

        CRoom {
            name: string_to_const_char_ptr(room.get_name()),
            price: string_to_const_char_ptr(room.get_price()),
            url: string_to_const_char_ptr(room.get_url()),
            a_n_o_hostel_club: room_type,
        }
    }

    pub fn to_room(&self) -> Result<Box<dyn Room>, RoomError> {
        // Convert C strings to Rust strings, handling null pointers
        let name = if self.name.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(self.name).to_str().unwrap().to_string() }
        };

        let price_str = if self.price.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(self.price).to_str().unwrap().to_string() }
        };

        let url = if self.url.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(self.url).to_str().unwrap().to_string() }
        };

        // Parse price string into Money
        let (currency, whole, cents) = if price_str.is_empty() {
            ("NONE", 0, 0)
        } else {
            let (currency, amount) = match price_str.chars().next() {
                Some('$') => ("DOL", &price_str[3..]),
                Some('€') => ("EUR", &price_str[3..]),
                _ => ("NONE", price_str.as_str()),
            };

            let parts: Vec<&str> = amount.split('.').collect();
            if parts.len() != 2 {
                return Err(RoomError::InvalidPriceFormat(price_str));
            }

            let whole: u64 = parts[0]
                .parse()
                .map_err(|_| RoomError::InvalidPriceFormat(price_str.clone()))?;
            let cents: u64 = parts[1]
                .parse()
                .map_err(|_| RoomError::InvalidPriceFormat(price_str))?;

            (currency, whole, cents)
        };

        Ok(Box::new(GenericRoom {
            name,
            price: Money::new(whole as f64 + cents as f64 / 100.0, currency)
                .unwrap_or(Money::new(0.0, "NONE").unwrap()),
            url,
        }))
    }
}
