use super::city::CCity;
use super::hostel::CHostel;
use super::room::CRoom;
use std::ffi::{CString, c_char};

pub unsafe fn free_croom(room: &CRoom) {
    unsafe {
        if !room.name.is_null() {
            let _ = CString::from_raw(room.name as *mut c_char);
        }
        if !room.price.is_null() {
            let _ = CString::from_raw(room.price as *mut c_char);
        }
        if !room.url.is_null() {
            let _ = CString::from_raw(room.url as *mut c_char);
        }
    }
}
pub unsafe fn free_chostel(hostel: &CHostel) {
    unsafe {
        if !hostel.name.is_null() {
            let _ = CString::from_raw(hostel.name as *mut c_char);
        }
        if !hostel.link.is_null() {
            let _ = CString::from_raw(hostel.link as *mut c_char);
        }
        if !hostel.room_options.is_null() {
            let rooms_slice =
                std::slice::from_raw_parts(hostel.room_options, hostel.number_of_rooms as usize);
            for room in rooms_slice {
                free_croom(room);
            }
        }
    }
}
pub unsafe fn free_ccity(city: &CCity) {
    unsafe {
        if !city.name.is_null() {
            let _ = CString::from_raw(city.name as *mut c_char);
        }
        if !city.ano_url.is_null() {
            let _ = CString::from_raw(city.ano_url as *mut c_char);
        }
        if !city.hostels.is_null() && city.hostels_len > 0 {
            let hostels_slice = std::slice::from_raw_parts(city.hostels, city.hostels_len as usize);

            for hostel in hostels_slice {
                free_chostel(hostel);
            }
        }
    }
}
