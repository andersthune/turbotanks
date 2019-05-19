use serde_derive::{Deserialize, Serialize};
use sfml::window::VideoMode;
use std::fs;
use std::io::Error;
use Resolution::{Desktop, Size};

#[derive(Serialize, Deserialize)]
pub enum Resolution {
    Size(u32, u32),
    Desktop,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub framerate_limit: u32,
    pub resolution: Resolution,
}

impl Settings {
    pub fn new(settings_path: &'static str) -> Settings {
        toml::from_str(&fs::read_to_string(settings_path).unwrap_or_default()).unwrap_or_default()
    }

    pub fn write(&self, settings_path: &'static str) -> Result<(), Error> {
        fs::write(settings_path, toml::to_string(&self).unwrap())
    }

    pub fn get_resolution(&self) -> VideoMode {
        match self.resolution {
            Size(h, w) => From::from((h, w)),
            Desktop => VideoMode::desktop_mode(),
        }
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            framerate_limit: 120,
            resolution: Desktop,
        }
    }
}
