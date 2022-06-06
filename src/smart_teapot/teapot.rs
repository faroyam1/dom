pub struct Teapot {}

impl Teapot {
    pub fn new() -> Self {
        Teapot {}
    }
}

impl crate::smart_house::Device for Teapot {
    fn device_state(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("boiling...".to_string())
    }
}

impl Default for Teapot {
    fn default() -> Self {
        Self::new()
    }
}
