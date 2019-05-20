use serde_derive::{Deserialize, Serialize};
use sfml::window::VideoMode;
use std::fs;
use std::io::Error;
use Resolution::{Desktop, Size};

/// An enum for resolution settings: either a specified size or the
/// desktop default.
#[derive(Serialize, Deserialize)]
pub enum Resolution {
    Size(u32, u32),
    Desktop,
}

/// A struct containing the relevant settings for configuring a run of
/// Turbo Tanks.
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub framerate_limit: u32,
    pub resolution: Resolution,
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
