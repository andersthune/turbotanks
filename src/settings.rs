use serde_derive::{Deserialize, Serialize};
use sfml::window::VideoMode;
use std::fs;
use std::io::Error;

/// A struct containing the relevant settings for configuring a run of
/// Turbo Tanks.
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub framerate_limit: u32,
    /// The initial resolution of the game. None corresponds to the
    /// default desktop resolution.
    pub resolution: Option<(u32, u32)>,
}

impl Settings {
    /// Create a new Settings object from the TOML file at
    /// `settings_path`. Will return default settings if the file
    /// couldn't be loaded.
    pub fn new(settings_path: &'static str) -> Settings {
        toml::from_str(&fs::read_to_string(settings_path).unwrap_or_default()).unwrap_or_default()
    }

    /// Serialize settings to TOML and write to the specified path.
    pub fn write(&self, settings_path: &'static str) -> Result<(), Error> {
        fs::write(settings_path, toml::to_string(&self).unwrap())
    }

    /// Get an SFML VideoMode object corresponding to the given
    /// resolution.
    pub fn get_resolution(&self) -> VideoMode {
        match self.resolution {
            Some(res) => From::from(res),
            None => VideoMode::desktop_mode(),
        }
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            framerate_limit: 120,
            resolution: None,
        }
    }
}
