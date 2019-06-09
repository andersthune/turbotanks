use crate::game::{Game, GameAction, GameState};
use sfml::graphics::{Drawable, RenderStates, RenderTarget, Sprite, Text};
use sfml::window::{Event, Key};

trait Clickable: Drawable {
    fn select(&self);
    fn mark(&self);
    fn unmark(&self);
}

trait Scrollable: Drawable {
    fn scroll_next(&mut self);
    fn scroll_previous(&mut self);
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
                GameAction::Select => self.buttons.select(),
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
