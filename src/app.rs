use winit::window::{self, Window};

pub struct App {
    window: Window
}

impl App {
    pub fn new() -> App;
    pub fn default() -> App {
        App {
            window: winit::window::Window
        }
    }
}


