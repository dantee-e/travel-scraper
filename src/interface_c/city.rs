use super::hostel::CHostel;
use super::string_to_const_char_ptr;
use crate::{interface_c::const_char_ptr_to_string, structs::City};
use std::ffi::{c_char, c_uint};

#[repr(C)]
pub struct CCity {
    pub name: *const c_char,
    pub ano_url: *const c_char,
    pub hostels: *const CHostel,
    pub hostels_len: c_uint,
}
impl CCity {
    pub fn from_city(city: &City) -> CCity {
        let hostels: Vec<CHostel> = city
            .hostels
            .clone()
            .into_iter()
            .map(|hostel| CHostel::from_hostel(hostel))
            .collect();
        let length = hostels.len() as c_uint;

        let ptr = Box::into_raw(hostels.into_boxed_slice());

        CCity {
            name: string_to_const_char_ptr(city.name.clone()),
            ano_url: string_to_const_char_ptr(city.ano_url.clone()),
            hostels: ptr as *const CHostel,
            hostels_len: length,
        }
    }

    pub fn to_city(&self) -> City {
        let name = unsafe { const_char_ptr_to_string(self.name) };
        let ano_url = unsafe { const_char_ptr_to_string(self.ano_url) };

        println!("Name: {}, ano_url: {}, getting hostels now", name, ano_url);

        let hostels = unsafe {
            std::slice::from_raw_parts(self.hostels, self.hostels_len as usize)
                .iter()
                .map(|hostel| {
                    let new_hostel = hostel.to_hostel();
                    println!("Got hostel with name {}", new_hostel.name);
                    new_hostel
                })
                .collect()
        };
        City {
            name,
            ano_url,
            hostels,
        }
    }
}

#[repr(C)]
pub struct ListCCity {
    pub cities: *const CCity,
    pub length: u32,
}
impl ListCCity {
    pub fn to_vec(&self) -> Vec<City> {
        if self.cities.is_null() || self.length == 0 {
            return Vec::new();
        }
        let cities_slice = unsafe { std::slice::from_raw_parts(self.cities, self.length as usize) };

        let result: Vec<City> = cities_slice
            .into_iter()
            .map(|city_ptr| city_ptr.to_city())
            .collect();

        result
    }

    pub fn from_vec(vec: Vec<City>) -> ListCCity {
        if vec.is_empty() {
            return ListCCity {
                cities: std::ptr::null(),
                length: 0,
            };
        }

        let len = vec.len() as u32;

        let ccity_vec: Vec<CCity> = vec
            .into_iter()
            .map(|city| {
                println!("Getting city with name {}", city.name);
                CCity::from_city(&city)
            })
            .collect();
        let ptr = ccity_vec.as_ptr();

        std::mem::forget(ccity_vec);

        ListCCity {
            cities: ptr as *const CCity,
            length: len,
        }
    }
}
