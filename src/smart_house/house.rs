use crate::smart_house::Room;
use std::collections::HashMap;

pub type Rooms = HashMap<String, Room>;

pub struct House {
    pub title: String,
    pub rooms: Rooms,
}

pub trait HouseInfo {
    fn info(&self, room_title: String, device_title: String) -> String;
}

impl House {
    pub fn new(title: String) -> Self {
        House {
            title,
            rooms: HashMap::new(),
        }
    }

    pub fn get_room(&self, title: &str) -> Option<&Room> {
        self.rooms.get(title)
    }

    pub fn get_room_mut(&mut self, title: &str) -> Option<&mut Room> {
        self.rooms.get_mut(title)
    }

    pub fn insert_room(&mut self, title: String, room: Room) {
        self.rooms.insert(title, room);
    }

    pub fn remove_room(&mut self, title: String) {
        self.rooms.remove_entry(&title);
    }
}

impl HouseInfo for House {
    fn info(&self, room_title: String, device_title: String) -> String {
        match self.get_room(&room_title) {
            Some(room) => match room.get_device(&device_title) {
                Some(device) => {
                    format!(
                        "house '{}'; room '{}'; device '{}', state: {}",
                        self.title,
                        room.title,
                        device_title,
                        device
                            .device_state()
                            .unwrap_or_else(|_| { "error while getting device state".to_string() })
                    )
                }
                None => format!("room: '{}' is empty", room_title),
            },

            None => format!("room '{}' does not exist", room_title),
        }
    }
}
