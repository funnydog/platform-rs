// src/views/game.rs

use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{AnimatedSprite, CopySprite};
use sdl2::pixels::Color;

// constants
const DEBUG: bool = true;

const GRAVITY: f64 = 24.0;

const PLAYER_SPEED: f64 = 120.0;
const PLAYER_HEIGHT: f64 = 48.0;
const PLAYER_WIDTH: f64 = 32.0;
const PLAYER_FPS: f64 = 15.0;
const PLAYER_JUMP_IMPULSE: f64 = -10.0;

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

enum PlayerDirection {
    Left,
    Right,
}

// types
struct Player {
    yvel: f64,
    rect: Rectangle,
    sprites: Vec<AnimatedSprite>,
    current: PlayerFrame,
    direction: PlayerDirection,
}

impl Player {
    pub fn new(phi: &mut Phi) -> Player {
        use ::phi::gfx::ASDescr::*;
        let sprites = vec![
            AnimatedSprite::load(phi, SingleFrame {
                image_path: "assets/player_still_left.png",
                frame_x: PLAYER_WIDTH,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, SingleFrame {
                image_path: "assets/player_still_right.png",
                frame_x: PLAYER_WIDTH,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, SingleFrame {
                image_path: "assets/player_still_left.png",
                frame_x: 0.0,
                frame_y: PLAYER_HEIGHT,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, SingleFrame {
                image_path: "assets/player_still_right.png",
                frame_x: 0.0,
                frame_y: PLAYER_HEIGHT,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, LoadFromStart {
                image_path: "assets/player_walk_left.png",
                total_frames: 14,
                frames_high: 4,
                frames_wide: 4,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, PLAYER_FPS),

            AnimatedSprite::load(phi, LoadFromStart {
                image_path: "assets/player_walk_right.png",
                total_frames: 14,
                frames_high: 4,
                frames_wide: 4,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, PLAYER_FPS),

            AnimatedSprite::load(phi, SingleFrame {
                image_path: "assets/player_still_left.png",
                frame_x: 0.0,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),

            AnimatedSprite::load(phi, SingleFrame {
                image_path: "assets/player_still_right.png",
                frame_x: 0.0,
                frame_y: 0.0,
                frame_w: PLAYER_WIDTH,
                frame_h: PLAYER_HEIGHT,
            }, 1.0),
        ];

        Player {
            yvel: 0.0,
            rect: Rectangle {
                x: 64.0,
                y: 64.0,
                w: PLAYER_WIDTH,
                h: PLAYER_HEIGHT,
            },
            sprites: sprites,
            current: PlayerFrame::StandRight,
            direction: PlayerDirection::Right,
        }
    }

    pub fn update(&mut self, phi: &mut Phi, elapsed: f64) {
        use self::PlayerFrame::*;
        use self::PlayerDirection::*;

        let moved = PLAYER_SPEED * elapsed;

        let mut dx: f64 = 0.0;
        if phi.events.key_left {
            dx -= moved;
        }
        if phi.events.key_right {
            dx += moved;
        }

        // determine the facing direction of the player
        if dx < 0.0 {
            self.direction = Left;
        } else if dx > 0.0 {
            self.direction = Right;
        }

        self.rect.y += self.yvel;
        self.rect.x += dx;
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0,
            h: phi.output_size().1 * 0.7,
        };

        self.rect = self.rect.move_inside(movable_region).unwrap();

        let touchground: bool = self.rect.y + self.rect.h >= movable_region.h;
        if !touchground {
            self.yvel += GRAVITY * elapsed;
        } else if phi.events.key_up {
            self.yvel = PLAYER_JUMP_IMPULSE;
        } else {
            self.yvel = 0.0;
        }

        match self.direction {
            Left => {
                if dx == 0.0 && touchground {
                    if phi.events.key_down {
                        self.current = SitLeft;
                    } else {
                        self.current = StandLeft;
                    }
                } else if !touchground {
                    self.current = JumpLeft;
                } else {
                    self.current = WalkLeft;
                }
            },
            Right => {
                if dx == 0.0 && touchground {
                    if phi.events.key_down {
                        self.current = SitRight;
                    } else {
                        self.current = StandRight;
                    }
                } else if !touchground {
                    self.current = JumpRight;
                } else {
                    self.current = WalkRight;
                }
            },
        };

        self.sprites[self.current as usize].add_time(elapsed);
    }

    pub fn render(&self, phi: &mut Phi) {
        let cursprite = &self.sprites[self.current as usize];
        if DEBUG {
            let movable_region = Rectangle {
                x: 0.0,
                y: 0.0,
                w: phi.output_size().0,
                h: phi.output_size().1 * 0.7,
            };
            phi.renderer.set_draw_color(Color::RGB(200,100,30));
            phi.renderer.fill_rect(movable_region.to_sdl());

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
