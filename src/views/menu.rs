// src/views/menu.rs

use phi::{Phi, View, ViewAction};
use phi::data::{Rectangle};
use phi::gfx::{Sprite, CopySprite};
use sdl2::pixels::Color;

// constants
const FONT_NAME: &'static str = "assets/belligerent.ttf";
const LABEL_H: f64 = 50.0;
const BORDER_WIDTH: f64 = 3.0;
const BOX_W: f64 = 360.0;
const MARGIN_H: f64 = 10.0;

// types
struct Action {
    // function executed when the action is chosen
    func: Box<Fn(&mut Phi) -> ViewAction>,

    // sprite rendered when the player doesn't focus
    idle_sprite: Sprite,

    // sprite rendered when the player focuses
    hover_sprite: Sprite,
}

impl Action {
    fn new(phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi) -> ViewAction>) -> Action {
        Action {
            func: func,
            idle_sprite: phi.ttf_str_sprite(label, FONT_NAME, 32, Color::RGB(220,220,220)).unwrap(),
            hover_sprite: phi.ttf_str_sprite(label, FONT_NAME, 38, Color::RGB(255,255,255)).unwrap(),
        }
    }
}

pub struct MenuView {
    actions: Vec<Action>,
    selected: i8,
}

impl MenuView {
    pub fn new(phi: &mut Phi) -> MenuView {
        MenuView {
            actions: vec![
                Action::new(phi, "New Game", Box::new(|phi| {
                    ViewAction::ChangeView(Box::new((::views::game::GameView::new(phi))))
                })),

                Action::new(phi, "Quit", Box::new(|_| {
                    ViewAction::Quit
                })),
            ],

            selected: 0,
        }
    }
}

impl View for MenuView {
    fn render(&mut self, phi: &mut Phi, _: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit
        }

        if phi.events.now.key_space == Some(true) {
            return (self.actions[self.selected as usize].func)(phi)
        }

        if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }

        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }

        // rendering
        phi.renderer.set_draw_color(Color::RGB(0,0,20));
        phi.renderer.clear();

        // render the box background
        let (win_w, win_h) = phi.output_size();
        let box_h = self.actions.len() as f64 * LABEL_H;

        phi.renderer.set_draw_color(Color::RGB(70,15,70));
        phi.renderer.fill_rect(Rectangle {
            w: BOX_W + BORDER_WIDTH * 2.0,
            h: box_h + BORDER_WIDTH * 2.0 + MARGIN_H * 2.0,
            x: (win_w - BOX_W) / 2.0 - BORDER_WIDTH,
            y: (win_h - box_h) / 2.0 - MARGIN_H - BORDER_WIDTH,
        }.to_sdl());

        phi.renderer.set_draw_color(Color::RGB(140,30,140));
        phi.renderer.fill_rect(Rectangle {
            w: BOX_W,
            h: box_h + MARGIN_H * 2.0,
            x: (win_w - BOX_W) / 2.0,
            y: (win_h - box_h) / 2.0 - MARGIN_H,
        }.to_sdl());

        // render the labels
        for (i, action) in self.actions.iter().enumerate() {
            let sprite = if self.selected as usize == i {
                &action.hover_sprite
            } else {
                &action.idle_sprite
            };

            let (w, h) = sprite.size();
            phi.renderer.copy_sprite(sprite, Rectangle {
                w: w,
                h: h,
                x: (win_w - w) / 2.0,
                y: (win_h - box_h + LABEL_H - h) / 2.0 + LABEL_H * i as f64,
            });
        }

        ViewAction::None
    }
}
