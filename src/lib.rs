use sfml::graphics::{CircleShape, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};

pub struct Game {
    window: RenderWindow,
}

impl Game {
    pub fn new() -> Game {
        let mut window = RenderWindow::new(
            (800, 600),
            "Turbo Tanks",
            Style::DEFAULT,
            &Default::default(),
        );
        window.set_framerate_limit(60);
        Game { window }
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
