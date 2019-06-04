use crate::settings::Settings;
use sfml::graphics::{CircleShape, RenderTarget, RenderWindow, Transformable};
use sfml::system::{Vector2f, Vector2u};
use sfml::window::{Event, Key, Style};

/// The main struct representing a running game
pub struct Game {
    pub settings: Settings,
    window: RenderWindow,
    settings_path: &'static str,
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
            window,
            settings_path,
            settings,
        }
    }

    pub fn is_running(&self) -> bool {
        self.window.is_open()
    }

    pub fn close(&mut self) {
        self.window.close()
    }
}

pub enum MenuAction {
    Up,
    Down,
    Left,
    Right,
    Enter,
}

impl MenuAction {
    pub fn get_menu_action_from_key(code: Key) -> Option<MenuAction> {
        match code {
            Key::Up | Key::W => Some(MenuAction::Up),
            Key::Down | Key::S => Some(MenuAction::Down),
            Key::Right | Key::D => Some(MenuAction::Right),
            Key::Left | Key::A => Some(MenuAction::Left),
            Key::Return => Some(MenuAction::Enter),
            _ => None,
        }
    }
}

pub struct MenuState<'a> {
    game: &'a mut Game,
}

impl<'a> MenuState<'a> {
    pub fn new(game: &'a mut Game) -> MenuState<'a> {
        MenuState { game }
    }

    pub fn run_state(&mut self) {
        while self.game.is_running() {
            self.act_on_events();
        }
    }

    fn act_on_events(&mut self) {
        while let Some(event) = self.game.window.poll_event() {
            match event {
                Event::KeyPressed { code, .. } => self.handle_key_press(code),
                Event::Closed => self.game.close(),
                _ => (),
            }
        }
    }

    fn handle_key_press(&mut self, key: Key) {
        if let Some(action) = MenuAction::get_menu_action_from_key(key) {
            match action {
                MenuAction::Up => {}

                MenuAction::Down => {}
            }
        }
    }
}
