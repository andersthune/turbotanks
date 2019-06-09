use crate::settings::Settings;
use sfml::graphics::{Drawable, RenderWindow, Sprite, Text};
use sfml::window::{Key, Style};

/// The main struct representing a running game
pub struct Game {
    pub settings: Settings,
    settings_path: &'static str,
    window: RenderWindow,
}

impl Game {
    /// Create a new game, loading settings from the specified path
    pub fn new(settings_path: &'static str) -> Game {
        let settings = Settings::new(settings_path);

        let mut window = RenderWindow::new(
            settings.get_resolution(),
            "Turbo Tanks",
            Style::DEFAULT,
            &Default::default(),
        );

        window.set_framerate_limit(settings.framerate_limit);

        Game {
            settings,
            settings_path,
            window,
        }
    }

    pub fn is_running(&self) -> bool {
        self.window.is_open()
    }

    pub fn close(&mut self) {
        self.window.close()
    }
}

/// An enumeration used to represent actions in the game. Different
/// keys might be bound to the same abstract action.
enum GameAction {
    Up,
    Down,
    Left,
    Right,
    Select,
    Shoot,
    Dodge,
}

impl GameAction {
    /// Get the GameAction corresponding to a given keycode. Later:
    /// let user customize through settings
    pub fn get_menu_action_from_key(code: Key) -> Option<GameAction> {
        match code {
            Key::Up | Key::W => Some(GameAction::Up),
            Key::Down | Key::S => Some(GameAction::Down),
            Key::Right | Key::D => Some(GameAction::Right),
            Key::Left | Key::A => Some(GameAction::Left),
            Key::Return => Some(GameAction::Select),
            _ => None,
        }
    }
}

trait GameState: Drawable {
    fn new();
    fn run_state(&mut self);
}
