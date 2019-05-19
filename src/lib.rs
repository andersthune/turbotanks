use sfml::graphics::{CircleShape, RenderTarget, RenderWindow};
use settings::Settings;
use sfml::window::{Event, Style};

pub mod settings;

pub struct Game {
    window: RenderWindow,
    settings_path: &'static str,
    settings: settings::Settings,
}

impl Game {
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

    pub fn running(&self) -> bool {
        self.window.is_open()
    }

    pub fn update(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                _ => (),
            }
        }
        let circle = CircleShape::new(10.0, 50);

        self.window.set_active(true);

        self.window.draw(&circle);

        self.window.display()
    }
}
