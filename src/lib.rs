#![allow(dead_code)]
use std::ffi::{CString, c_char, c_uint};
use std::slice::from_raw_parts;

use crate::interface_c::free::free_ccity;
use crate::interface_c::{city::ListCCity, const_char_ptr_to_string, listcstring::ListCString};
use crate::make_request::a_and_o::{ANORequest, request_a_and_o};
use crate::make_request::curry_get_city_url_a_and_o;
use crate::structs::City;
use tokio::runtime::Runtime;

mod extract_options;
mod interface_c;
mod make_request;
mod structs;

async fn async_get_many_cities(
    cities_names: Vec<String>,
    date_start: String,
    date_end: String,
) -> Option<Vec<City>> {
    let mut cities: Vec<City> = Vec::new();

    let date1 = date_start.to_string();
    let date2 = date_end.to_string();

    let mut get_city_url = curry_get_city_url_a_and_o();

    // Adds the URLs to the cities structs
    for city in cities_names {
        if let Some(url) = get_city_url(&city) {
            cities.push(City::new(city, url))
        } else {
            println!("Error getting {city} url");
        }
    }

    for city in &mut cities {
        match request_a_and_o(ANORequest::new(&city, 2, date1.clone(), date2.clone())).await {
            Some(result) => city.add_hostels(result),
            None => {
                println!("Could not find available hostels in {}", city.name)
            }
        };
    }

    Some(cities)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_many_cities(
    cities_names_c: ListCString,
    date_start: *mut std::ffi::c_char,
    date_end: *mut std::ffi::c_char,
) -> *const ListCCity {
    let rt = Runtime::new().unwrap();

    let cities_names = cities_names_c.to_vec();

    let date_start = unsafe { const_char_ptr_to_string(date_start) };
    let date_end = unsafe { const_char_ptr_to_string(date_end) };

    let cities = rt
        .block_on(async_get_many_cities(cities_names, date_start, date_end))
        .unwrap();

    let c_list_cities = ListCCity::from_vec(cities);
    Box::into_raw(Box::new(c_list_cities))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_list_c_string(ptr: *mut ListCString, len: c_uint) {
    if ptr.is_null() {
        println!("pointer provided to free city list was null");
        return;
    }

    if len == 0 {
        return;
    }

    unsafe {
        let city_ls = std::ptr::read(ptr);

        let strings = from_raw_parts(city_ls.strings, city_ls.length as usize);

        for city in strings {
            let _ = CString::from_raw(*city as *mut c_char);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_city_list(ptr: *mut ListCCity) {
    if ptr.is_null() {
        println!("pointer provided to free city list was null");
        return;
    }
    println!("Freeing city");

    unsafe {
        let list = &*ptr;
        let cities_slice = std::slice::from_raw_parts(list.cities, list.length as usize);

        for city in cities_slice {
            free_ccity(city);
        }
    }
}
