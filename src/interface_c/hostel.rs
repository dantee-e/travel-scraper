use super::room::CRoom;
use super::string_to_const_char_ptr;
use crate::structs::Hostel;
use crate::{interface_c::const_char_ptr_to_string, structs::rooms::Room};
use std::ffi::{c_char, c_uint};

#[repr(C)]
pub struct CHostel {
    pub name: *const c_char,
    pub room_options: *const CRoom,
    pub link: *const c_char,
    pub number_of_rooms: c_uint,
}
impl CHostel {
    pub fn from_hostel(hostel: Hostel) -> CHostel {
        let length = hostel.room_options.len() as c_uint;

        let rooms: Vec<CRoom> = hostel
            .room_options
            .into_iter()
            .map(|room| CRoom::from_room(&room))
            .collect();

        let ptr = Box::into_raw(rooms.into_boxed_slice());

        let name = string_to_const_char_ptr(hostel.name);
        let link = string_to_const_char_ptr(hostel.link);

        CHostel {
            name,
            room_options: ptr as *const CRoom,
            link,
            number_of_rooms: length,
        }
    }
    pub fn to_hostel(&self) -> Hostel {
        if self.room_options.is_null() {
            panic!("Room options eh null");
        }

        println!("Getting rooms now");
        let rooms =
            unsafe { std::slice::from_raw_parts(self.room_options, self.number_of_rooms as usize) };

        println!("Rooms ok");
        let room_options: Vec<Box<dyn Room>> = rooms
            .into_iter()
            .map(|room| room.to_room().expect("Couldnt return to room"))
            .collect();
        println!("Room options ok");
        Hostel {
            name: unsafe { const_char_ptr_to_string(self.name) },
            room_options: room_options,
            link: unsafe { const_char_ptr_to_string(self.link) },
        }
    }
}
