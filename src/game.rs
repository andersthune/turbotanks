use crate::settings::Settings;
use sfml::graphics::{CircleShape, RenderTarget, RenderWindow, Transformable};
use sfml::system::{Vector2f, Vector2u};
use sfml::window::{Event, Style};

/// The main struct representing a running game
pub struct Game {
    window: RenderWindow,
    settings_path: &'static str,
    settings: Settings,
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

    /// Run the game
    pub fn run(&mut self) {
        while self.window.is_open() {
            self.update()
        }
    }

    fn update(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if let Event::Closed = event {
                self.window.close()
            }
        }
        let mut circle = CircleShape::new(10.0, 50);
        let Vector2u { x, y } = self.window.size();
        let x = (x as f32) / 2.0;
        let y = (y as f32) / 2.0;

        circle.set_position(Vector2f { x, y });

        self.window.set_active(true);

        self.window.draw(&circle);

        self.window.display()
    }
}
