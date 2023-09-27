
pub struct Game {
    pub profile: GameProfile
}

#[derive(Debug)]
pub struct GameProfile {
    pub name: String,
    pub developer: String,
    pub publisher: String,
    pub release_date: String,
    pub website_url: String,
    pub wikipedia_page_url: String,
    pub platform_names: String
}

struct Platform {
    input_devices: Vec<PlatformInputDevice>
}

struct PlatformInputDevice {
    inputs: Vec<PlatformInputDeviceInput>
}

struct PlatformInputDeviceInput {
    name: String,
    symbol: String 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
