// src/views/game.rs

use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{AnimatedSprite, ASDescr, CopySprite, Sprite};
use sdl2::pixels::Color;

// constants
const DEBUG: bool = true;
const PLAYER_SPEED: f64 = 180.0;
const PLAYER_HEIGHT: f64 = 48.0;
const PLAYER_WIDTH: f64 = 32.0;
const PLAYER_FPS: f64 = 15.0;

#[derive(Clone, Copy)]
enum PlayerFrame {
    SitLeft = 0,
    SitRight = 1,
    StandLeft = 2,
    StandRight = 3,
    WalkLeft = 4,
    WalkRight = 5,
    JumpLeft = 6,
    JumpRight = 7,
}

// types
struct Player {
    rect: Rectangle,
    sprites: Vec<AnimatedSprite>,
    current: PlayerFrame,
}

impl Player {
    pub fn new(phi: &mut Phi) -> Player {
        let sprites = vec![
            AnimatedSprite::load(phi, ASDescr::SingleFrame {
                image_path: "assets/player_still_left.png",
                frame_x: PLAYER_WIDTH,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, ASDescr::SingleFrame {
                image_path: "assets/player_still_right.png",
                frame_x: PLAYER_WIDTH,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, ASDescr::SingleFrame {
                image_path: "assets/player_still_left.png",
                frame_x: 0.0,
                frame_y: PLAYER_HEIGHT,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, ASDescr::SingleFrame {
                image_path: "assets/player_still_right.png",
                frame_x: 0.0,
                frame_y: PLAYER_HEIGHT,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, ASDescr::LoadFromStart {
                image_path: "assets/player_walk_left.png",
                total_frames: 14,
                frames_high: 4,
                frames_wide: 4,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, PLAYER_FPS),

            AnimatedSprite::load(phi, ASDescr::LoadFromStart {
                image_path: "assets/player_walk_right.png",
                total_frames: 14,
                frames_high: 4,
                frames_wide: 4,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, PLAYER_FPS),

            AnimatedSprite::load(phi, ASDescr::SingleFrame {
                image_path: "assets/player_still_left.png",
                frame_x: 0.0,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, ASDescr::SingleFrame {
                image_path: "assets/player_still_right.png",
                frame_x: 0.0,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),
        ];

        Player {
            rect: Rectangle {
                x: 64.0,
                y: 64.0,
                w: PLAYER_WIDTH,
                h: PLAYER_HEIGHT,
            },
            sprites: sprites,
            current: PlayerFrame::StandRight,
        }
    }

    pub fn update(&mut self, phi: &mut Phi, elapsed: f64) {
        use self::PlayerFrame::*;

        let diagonal =
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.current = match self.current {
            WalkLeft if dx == 0.0 => StandLeft,
            WalkRight if dx == 0.0 => StandRight,
            _ if dx < 0.0 => WalkLeft,
            _ if dx > 0.0 => WalkRight,
            _ => self.current,
        };

        self.rect.x += dx;
        self.rect.y += dy;

        self.sprites[self.current as usize].add_time(elapsed);
    }

    pub fn render(&self, phi: &mut Phi) {
        let cursprite = &self.sprites[self.current as usize];
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200,200,50));
            phi.renderer.fill_rect(self.rect.to_sdl());
        }

        phi.renderer.copy_sprite(cursprite, self.rect);
    }
}

pub struct GameView {
    player: Player,
}

impl GameView {
    pub fn new(phi: &mut Phi) -> GameView {
        GameView {
            player: Player::new(phi),
        }
    }
}

impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit
        }

        self.player.update(phi, elapsed);

        phi.renderer.set_draw_color(Color::RGB(0,0,50));
        phi.renderer.clear();

        self.player.render(phi);

        ViewAction::None
    }
}
