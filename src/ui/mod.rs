use iced::{
    widget::{Column, PickList, Text, Container},
    Alignment, Element, Sandbox, Settings, Length,
    Size
};
use url::Url;
use std::str::FromStr;
use crate::structs::{
    City,
    Hostel,
    rooms::{RoomHostelClub, RoomAnO},
    money::Money,
};

#[derive(Debug, Clone)]
pub enum Message {
    CitySelected(usize),
    HostelSelected(usize),
    RoomSelected(usize),
}

pub struct CityHostelBrowser {
    cities: Vec<City>,
    selected_city: Option<usize>,
    selected_hostel: Option<usize>,
    selected_room: Option<usize>,
    city_names: Vec<String>,
}

impl Sandbox for CityHostelBrowser {
    type Message = Message;

    fn new() -> Self {
        let cities = vec![
            City {
                name: "Paris".to_string(),
                ano_url: "https://paris.example.com".to_string(),
                hostels: vec![
                    Hostel {
                        name: "Paris Hostel A".to_string(),
                        link: "https://hostela.paris.com".to_string(),
                        room_options: vec![
                            Box::new(RoomAnO {
                                name: "Double Room".to_string(),
                                lowest_price: Money::new(50.0, "EUR").unwrap(),
                                total_price: Money::new(60.0, "EUR").unwrap(),
                                url: Url::from_str("https://room1.paris.com").unwrap(),
                            }),
                            Box::new(RoomHostelClub {
                                name: "Dorm 4 Beds".to_string(),
                                price: Money::new(25.0, "EUR").unwrap(),
                                beds: 4,
                            }),
                        ],
                    },
                    Hostel {
                        name: "Paris Hostel B".to_string(),
                        link: "https://hostelb.paris.com".to_string(),
                        room_options: vec![
                            Box::new(RoomAnO {
                                name: "Single Room".to_string(),
                                lowest_price: Money::new(40.0, "EUR").unwrap(),
                                total_price: Money::new(48.0, "EUR").unwrap(),
                                url: Url::from_str("https://room2.paris.com").unwrap(),
                            }),
                        ],
                    },
                ],
            },
            City {
                name: "Berlin".to_string(),
                ano_url: "https://berlin.example.com".to_string(),
                hostels: vec![
                    Hostel {
                        name: "Berlin Hostel X".to_string(),
                        link: "https://hostelx.berlin.com".to_string(),
                        room_options: vec![
                            Box::new(RoomHostelClub {
                                name: "Dorm 6 Beds".to_string(),
                                price: Money::new(20.0, "EUR").unwrap(),
                                beds: 6,
                            }),
                        ],
                    },
                ],
            },
        ];

        let city_names = cities.iter().map(|c| c.name.clone()).collect();

        CityHostelBrowser {
            cities,
            selected_city: Some(0),
            selected_hostel: None,
            selected_room: None,
            city_names,
        }
    }

    fn title(&self) -> String {
        String::from("City Hostel Browser")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CitySelected(idx) => {
                self.selected_city = Some(idx);
                self.selected_hostel = Some(0);
                self.selected_room = Some(0);
            }
            Message::HostelSelected(idx) => {
                self.selected_hostel = Some(idx);
                self.selected_room = Some(0);
            }
            Message::RoomSelected(idx) => {
                self.selected_room = Some(idx);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let city_picker = PickList::new(
            self.city_names.clone(), // Clone city_names since it's owned by self
            self.selected_city.map(|i| self.city_names[i].clone()),
            |name| {
                let idx = self.city_names.iter().position(|n| n == &name).unwrap();
                Message::CitySelected(idx)
            },
        )
            .placeholder("Select City");

        let hostel_picker = if let Some(city_idx) = self.selected_city {
            let hostels = &self.cities[city_idx].hostels;
            let hostel_names: Vec<String> = hostels.iter().map(|h| h.name.clone()).collect();
            PickList::new(
                hostel_names.clone(), // Pass ownership directly
                self.selected_hostel
                    .and_then(|i| hostel_names.get(i).cloned()),
                move |name| {
                    let idx = hostel_names.iter().position(|n| n == &name).unwrap();
                    Message::HostelSelected(idx)
                },
            )
                .placeholder("Select Hostel")
        } else {
            PickList::new(vec![], None, |_| Message::HostelSelected(0))
                .placeholder("Select Hostel")
        };

        let room_picker = if let (Some(city_idx), Some(hostel_idx)) = (self.selected_city, self.selected_hostel) {
            let rooms = &self.cities[city_idx].hostels[hostel_idx].room_options;
            let room_names: Vec<String> = rooms.iter().map(|r| r.get_name()).collect();
            PickList::new(
                room_names.clone(), // Pass ownership directly
                self.selected_room
                    .and_then(|i| room_names.get(i).cloned()),
                move |name| {
                    let idx = room_names.iter().position(|n| n == &name).unwrap();
                    Message::RoomSelected(idx)
                },
            )
                .placeholder("Select Room")
        } else {
            PickList::new(vec![], None, |_| Message::RoomSelected(0))
                .placeholder("Select Room")
        };

        let room_info = if let (Some(city_idx), Some(hostel_idx), Some(room_idx)) = (
            self.selected_city,
            self.selected_hostel,
            self.selected_room,
        ) {
            Text::new(
                self.cities[city_idx].hostels[hostel_idx].room_options[room_idx]
                    .get_room_string(),
            )
        } else {
            Text::new("")
        };

        let content = Column::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(Text::new("Select City:"))
            .push(city_picker)
            .push(Text::new("Select Hostel:"))
            .push(hostel_picker)
            .push(Text::new("Select Room:"))
            .push(room_picker)
            .push(room_info);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn main() -> iced::Result {
    CityHostelBrowser::run(Settings {
        window: iced::window::Settings {
            size: Size::new(600.0, 400.0),
            ..Default::default()
        },
        ..Default::default()
    })
}


