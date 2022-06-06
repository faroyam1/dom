use std::collections::HashMap;

pub trait Device {
    fn device_state(&self) -> Result<String, Box<dyn std::error::Error>>;
}

pub type Devices = HashMap<String, Box<dyn Device>>;

pub struct Room {
    pub title: String,
    pub devices: Devices,
}

impl Room {
    pub fn new(title: String) -> Self {
        Room {
            title,
            devices: HashMap::new(),
        }
    }

    // Обычная ссылка для чтения
    pub fn get_device(&self, title: &str) -> Option<&dyn Device> {
        self.devices.get(title).map(|b| b.as_ref())
    }

    /// Получаем мутабельную ссылку для модификации
    /*pub fn get_device_mut(&mut self, title: &str) -> Option<&mut Box<dyn Device>> {
        self.devices.get_mut(title)
    }*/

    pub fn insert_device(&mut self, title: String, device: Box<dyn Device>) {
        self.devices.insert(title, device);
    }

    pub fn remove_device(&mut self, title: String) {
        self.devices.remove_entry(&title);
    }
}
