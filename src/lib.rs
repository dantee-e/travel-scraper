use std::error::Error;
use std::ffi::{CStr, CString};
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::io::AsyncReadExt;
use crossbeam::channel;

use crate::make_request::a_and_o::{request_a_and_o, ANORequest};
use crate::make_request::curry_get_city_url_a_and_o;
use crate::structs::City;

use libc::{c_char, c_int, size_t};

mod extract_options;
mod make_request;
mod structs;


#[repr(C)]
pub struct ListCCity {
    cities: *const City,
    len: u32,
}
impl ListCCity {
    fn from_vec(vec: Vec<City>) -> ListCCity {
        if vec.is_empty() {
            return ListCCity {cities: std::ptr::null(), len: 0};
        }
        
        let len = vec.len() as u32;
        let ptr = vec.as_ptr();
        
        std::mem::forget(vec);
        
        ListCCity {cities: ptr as *const City, len }
    }
}


#[repr(C)]
pub struct ListCString {   
    strings: *const CString,
    length: u32,
}
impl ListCString {
    fn to_vec(&self) -> Vec<String> {
        if self.strings.is_null() || self.length == 0 {
            return Vec::new();
        }
        let cities_slice = unsafe { std::slice::from_raw_parts(self.strings, self.length as usize) };
        
        let mut result = Vec::with_capacity(cities_slice.len());
        
        for c_string in cities_slice {
            if let Ok(c_string) = c_string.to_str(){
                result.push(c_string.to_string());
            } else {println!("Error converting {c_string:?} to string");}
        }
        
        result
    }
    
    // fn from_vec(vec: Vec<String>) -> ListCString {
    //     if vec.is_empty() {
    //         return ListCString { strings: std::ptr::null(), length: 0};
    //     }
    //     
    //     let c_strings = vec.iter()
    //         .map(|string| CString::new(string.as_str()).unwrap())
    //         .collect::<Vec<CString>>();
    //     
    //     let len = c_strings.len();
    //     let mut c_strings = c_strings;
    //     let ptr = c_strings.as_mut_ptr();
    //     std::mem::forget(c_strings);
    //     
    //     ListCString { strings: ptr, length: len as u32}
    // }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_many_cities(
    cities_names_c: ListCString, date_start: CString, date_end: CString,
) -> ListCCity {
    let handle = Handle::current();
    let (tx, rx) = channel::bounded(1);
    
    let cities_names = cities_names_c.to_vec();
    let date_start = date_start.to_str().unwrap().to_string();
    let date_end = date_end.to_str().unwrap().to_string();
    
    handle.spawn(async move{
        let score_res = async_get_many_cities(cities_names, date_start, date_end).await;
        let _ = tx.send(score_res);
    });
    ListCCity::from_vec(rx.recv().unwrap().unwrap())
}


async fn async_get_many_cities(
    cities_names: Vec<String>, date_start: String, date_end: String
)  -> Option<Vec<City>>{
    let mut cities: Vec<City> = Vec::new();

    let date1 = date_start.to_string();
    let date2 = date_end.to_string();

    let mut get_city_url = curry_get_city_url_a_and_o();

    // Adds the URLs to the cities structs
    for city in cities_names {
        let url = get_city_url(&city).unwrap();
        cities.push( City::new( city, url ))
    }

    for city in &mut cities {
        match request_a_and_o(ANORequest::new(
            &city,
            2,
            date1.clone(),
            date2.clone(),
        )).await {
            Some(result) => { city.add_hostels(result) }
            None => { println!("Could not find available hostels in {}", city.name) }
        };
    }

    Some(cities)
}