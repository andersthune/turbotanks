use crate::settings::Settings;
use sfml::graphics::{Drawable, RenderStates, RenderTarget, RenderWindow, Sprite, Text};
use sfml::window::{Event, Key, Style};

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
    Enter,
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
            Key::Return => Some(GameAction::Enter),
            _ => None,
        }
    }
}

trait Clickable: Drawable {
    fn mark(&self);
    fn unmark(&self);
    fn select(&self);
    fn toggle_right(&mut self) {}
    fn toggle_left(&mut self) {}
}

struct Button<'a> {
    text: Text<'a>,
    effect: fn(),
}

impl<'a> Clickable for Button<'a> {
    fn select(&self) {
        (self.effect)()
    }
    fn mark(&self) {}
    fn unmark(&self) {}
}

impl<'b> Drawable for Button<'b> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.text.draw(target, states);
    }
}

struct MenuButtonList {
    button_vec: Vec<Box<dyn Clickable>>,
    marked_index: usize,
}

impl MenuButtonList {
    fn new() -> MenuButtonList {
        MenuButtonList {
            button_vec: Vec::new(),
            marked_index: 0,
        }
    }

    fn push_button(&mut self, button: Box<dyn Clickable>) {
        self.button_vec.push(button)
    }

    fn get_marked_button(&self) -> Box<dyn Clickable> {
        self.button_vec[self.marked_index]
    }

    fn draw_buttons(&self, target: &mut RenderTarget) {
        self.button_vec.iter().map(|x| target.draw(&(**x)));
    }

    fn update_marked_index(&mut self, direction: GameAction) {
        let new_marker = match direction {
            GameAction::Up => self.marked_index - 1,
            GameAction::Down => self.marked_index + 1,
            _ => 0,
        };

        self.marked_index = new_marker % self.button_vec.len();
    }

    fn move_marker(&mut self, direction: GameAction) {
        self.get_marked_button().unmark();
        self.update_marked_index(direction);
        self.get_marked_button().mark();
    }

    fn go_right(&mut self) {
        self.get_marked_button().toggle_right();
    }
    fn go_left(&mut self) {
        self.get_marked_button().toggle_left();
    }
    fn select(&mut self) {
        self.get_marked_button().select();
    }
}

/// A struct representing a menu state in the game.
pub struct MenuState<'a> {
    game: &'a mut Game,
    images: Vec<Sprite<'a>>,
    text: Vec<Text<'a>>,
    buttons: MenuButtonList,
    _active: bool,
}

impl<'a> MenuState<'a> {
    /// Execute the state
    pub fn run_state(&mut self) {
        while self.is_active() {
            self.act_on_events();
            self.draw_menu();
        }
    }

    fn is_active(&self) -> bool {
        self.game.is_running() && self._active
    }

    fn exit(&self) {
        self._active = false
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
        if let Some(action) = GameAction::get_menu_action_from_key(key) {
            match action {
                GameAction::Up | GameAction::Down => self.buttons.move_marker(action),
                GameAction::Right => self.buttons.go_right(),
                GameAction::Left => self.buttons.go_left(),
                GameAction::Enter => self.buttons.select(),
            }
        }
    }

    fn draw_menu(&self) {
        let target = self.game.window;
        self.images.iter().map(|x| target.draw(x));
        self.text.iter().map(|x| target.draw(x));
        self.buttons.draw_buttons(&mut target);
    }
}
