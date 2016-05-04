// src/views/game.rs

use glm::*;

use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{AnimatedSprite, CopySprite, RenderFx, Sprite};

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

struct Player {
    yvel: f64,
    pos: Vector2<f64>,
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
            pos: Vector2 {
                x: 64.0,
                y: 64.0,
            },
            sprites: sprites,
            current: PlayerFrame::StandRight,
            direction: PlayerDirection::Right,
        }
    }

    pub fn bounding_rect(&self) -> Rectangle {
        Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
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

        self.pos.x += dx;
        self.pos.y += self.yvel;
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0,
            h: phi.output_size().1 * 0.7,
        };

        if let Some(rect) = self.bounding_rect().move_inside(movable_region) {
            self.pos.x = rect.x;
            self.pos.y = rect.y;

            let touchground: bool = rect.y + rect.h >= movable_region.h;
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
        }

        self.sprites[self.current as usize].add_time(elapsed);
    }

    pub fn render(&self, phi: &mut Phi) {
        let cursprite = &self.sprites[self.current as usize];
        let rect = Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        }.to_sdl();

        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200,200,50));
            phi.renderer.fill_rect(rect).unwrap();
        }

        phi.renderer.copy_sprite(cursprite, &rect, RenderFx::None);
    }
}

pub struct GameView {
    player: Player,
    layers: Vec<Sprite>,
}

impl GameView {
    pub fn new(phi: &mut Phi) -> GameView {
        GameView {
            player: Player::new(phi),
            layers: vec![
                Sprite::load(&mut phi.renderer, "assets/background0.png").unwrap(),
                Sprite::load(&mut phi.renderer, "assets/background1.png").unwrap(),
                Sprite::load(&mut phi.renderer, "assets/background2.png").unwrap(),
            ],
        }
    }
}

impl View for GameView {
    fn update(mut self: Box<Self>, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit {
            return ViewAction::Quit
        }

        if phi.events.now.key_escape == Some(true) {
            return ViewAction::Render(Box::new(
                ::views::menu::MenuView::new(phi)))
        }

        self.player.update(phi, elapsed);

        ViewAction::Render(self)
    }

    fn render(&self, phi: &mut Phi) {
        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0,0,50));
        phi.renderer.clear();

        // Draw the background layers
        for layer in &self.layers {
            let (w, h) = layer.size();
            phi.renderer.copy_sprite(
                layer,
                &Rectangle {
                    x: 0.0,
                    y: 0.0,
                    w: w,
                    h: h,
                }.to_sdl(),
                RenderFx::None,
            );
        }

        // Draw the player
        self.player.render(phi);
    }
}
