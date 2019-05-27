//! Turbo Tanks: A 2D skill-based tank combat arena game

pub mod game;
mod settings;

pub use game::Game;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn settings_parsed_correctly() {
        let write_path = "test.toml";
        let string_to_write = String::from(
            r#"framerate_limit = 60
resolution = [1920, 1080]
"#,
        );
        fs::write(write_path, string_to_write).unwrap();
        let settings = settings::Settings::new(write_path);
        fs::remove_file(write_path).unwrap();

        assert_eq!(settings.framerate_limit, 60);
        assert_eq!(settings.resolution, Some((1920, 1080)));
    }

    #[test]
    fn default_settings_saved_correctly() {
        let write_path = "test.toml";
        let expected_string = String::from(
            r#"framerate_limit = 120
"#,
        );
        let settings = settings::Settings::new("");
        settings.write(write_path).expect("Failed to save settings");
        let settings_string = fs::read_to_string(write_path).unwrap();
        fs::remove_file(write_path).unwrap();

        assert_eq!(settings_string, expected_string)
    }

    #[test]
    fn game_loads_settings() {}
}
