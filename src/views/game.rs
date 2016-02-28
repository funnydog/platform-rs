// src/views/game.rs

use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;

// constants

// types
pub struct GameView;

impl GameView {
    pub fn new(phi: &mut Phi) -> GameView {
        GameView
    }
}

impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit
        }

        phi.renderer.set_draw_color(Color::RGB(0,0,50));
        phi.renderer.clear();

        ViewAction::None
    }
}
