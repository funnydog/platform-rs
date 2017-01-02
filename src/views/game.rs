// src/views/game.rs

use glm;

use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::*;

use sdl2::render::Renderer;
use sdl2::pixels;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// constants
const DEBUG: bool = true;

const TILE_WIDTH: f64 = 40.0;
const TILE_HEIGHT: f64 = 32.0;

struct GameLevel {
    pub layers: Vec<Sprite>,
    pub tiles: Vec<Vec<Tile>>,
    pub gems: Vec<Box<Gem>>,
    pub start: glm::Vector2<f64>,
    pub exit: glm::Vector2<f64>,
    pub width: usize,
    pub height: usize,
}

impl GameLevel {
    pub fn load(phi: &mut Phi, path: &str) -> GameLevel {
        let f = File::open(path).unwrap();
        let file = BufReader::new(&f);

        let mut gems: Vec<Box<Gem>> = Vec::new();
        let gem_sprite = Sprite::load(&phi.renderer, "assets/sprites/gem.png").unwrap();

        let mut lines: Vec<String> = Vec::new();
        let mut width: usize = 0;

        for line in file.lines() {
            let buffer = line.unwrap().to_string();
            width = buffer.len();
            lines.push(buffer);
        }

        let height: usize = lines.len();
        let mut yvec: Vec<Vec<Tile>> = Vec::with_capacity(height);
        let mut exit: glm::Vector2<f64> = glm::Vector2::new(0.0, 0.0);
        let mut start: glm::Vector2<f64> = glm::Vector2::new(0.0, 0.0);
        for yth in 0..height {
            let mut xvec: Vec<Tile> = Vec::with_capacity(width);
            for (xth, tile_type) in lines[yth].chars().enumerate() {
                xvec.push(match tile_type {
                    '.' => {
                        // Blank space
                        Tile::new(None, TileCollision::Passable)
                    },
                    'X' => {
                        // Exit point
                        exit.x = xth as f64 * TILE_WIDTH + TILE_WIDTH / 2.0;
                        exit.y = yth as f64 * TILE_HEIGHT + TILE_HEIGHT / 2.0;
                        Tile::load(phi, "assets/tiles/exit.png", TileCollision::Passable)
                    },
                    'G' => {
                        // Gem
                        let pos = glm::Vector2::new(
                            xth as f64 * TILE_WIDTH + TILE_WIDTH / 2.0,
                            yth as f64 * TILE_HEIGHT + TILE_HEIGHT / 2.0,
                        );

                        // put the gem into the gem list
                        gems.push(Box::new(Gem::new(&gem_sprite, pos)));
                        Tile::new(None, TileCollision::Passable)
                    },
                    '-' => {
                        // Floating platform
                        Tile::load(phi, "assets/tiles/platform.png", TileCollision::Platform)
                    },
                    'A' => {
                        // TODO: add the enemy to the enemy list
                        Tile::new(None, TileCollision::Passable)
                    },
                    'B' => {
                        // TODO: add the enemy to the enemy list
                        Tile::new(None, TileCollision::Passable)
                    },
                    'C' => {
                        // TODO: add the enemy to the enemy list
                        Tile::new(None, TileCollision::Passable)
                    },
                    'D' => {
                        // TODO: add the enemy to the enemy list
                        Tile::new(None, TileCollision::Passable)
                    },
                    '~' => {
                        // Platform block
                        GameLevel::load_random_tile(phi, "assets/tiles/blockb", 2, TileCollision::Platform)
                    },
                    ':' => {
                        // Passable block
                        GameLevel::load_random_tile(phi, "assets/tiles/blockb", 2, TileCollision::Passable)
                    },
                    '1' => {
                        // player start point
                        start.x = xth as f64 * TILE_WIDTH + TILE_WIDTH / 2.0;
                        start.y = yth as f64 * TILE_HEIGHT + TILE_HEIGHT / 2.0;
                        Tile::new(None, TileCollision::Passable)
                    },
                    '#' => {
                        // Impassable block
                        GameLevel::load_random_tile(phi, "assets/tiles/blocka", 7, TileCollision::Impassable)
                    },
                    _ => { panic!("Unsupported tile type '{}'", tile_type); }
                });
            }
            yvec.push(xvec);
        }

        GameLevel {
            layers: vec![
                Sprite::load(&mut phi.renderer, "assets/background0.png").unwrap(),
                Sprite::load(&mut phi.renderer, "assets/background1.png").unwrap(),
                Sprite::load(&mut phi.renderer, "assets/background2.png").unwrap(),
            ],
            tiles: yvec,
            gems: gems,
            start: start,
            exit: exit,
            width: width,
            height: height,
        }
    }

    fn load_random_tile(phi: &mut Phi, base: &str, count: usize, collision: TileCollision) -> Tile {
        let x = ::rand::random::<usize>() % count;
        let name = format!("{}{}.png", base, x);

        Tile::load(phi, &name, collision)
    }

    fn get_collision(&self, x: i32, y: i32) -> TileCollision {
        if y < 0 || y >= self.height as i32 {
            TileCollision::Passable
        } else if x < 0 || x >= self.width as i32 {
            TileCollision::Impassable
        } else {
            self.tiles[y as usize][x as usize].collision
        }
    }

    pub fn update(&mut self, phi: &mut Phi, elapsed: f64) {
        // update the gems
        let mut old_gems = ::std::mem::replace(&mut self.gems, vec![]);
        while let Some(mut gem) = old_gems.pop() {
            // TODO: instead of false check for intersection with player
            if false {
                // TODO: add the gem points to the score
                // TODO: collect the gem
            } else {
                gem.update(phi, elapsed);
                self.gems.push(gem);
            }
        }

        // TODO: falling off the bottom kills the player

        // TODO: update the enemies
    }

    pub fn render(&self, phi: &mut Phi) {
        // Draw the background layers
        for layer in &self.layers {
            let (w, h) = layer.size();
            let dest = Rectangle {
                x: 0.0, y: 0.0,
                w: w, h: h,
            };
            layer.render(&mut phi.renderer, &dest.to_sdl(), RenderFx::None);
        }

        // Draw the tiles
        let mut rect = Rectangle::with_size(TILE_WIDTH, TILE_HEIGHT);
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                let srect = rect.to_sdl();
                self.tiles[y][x].render(&mut phi.renderer, &srect, RenderFx::None);
                rect.x += TILE_WIDTH;
            }
            rect.x = 0.0;
            rect.y += TILE_HEIGHT;
        }

        // Render the gems
        for gem in &self.gems {
            gem.render(phi);
        }

        // TODO: invert the logic
        // render the player

        // render the enemies
    }
}

const PLAYER_WIDTH: f64 = 64.0;
const PLAYER_HEIGHT: f64 = 64.0;
const PLAYER_FPS: f64 = 15.0;

// horizontal movements
const PLAYER_MOVE_ACCEL: f32 = 13000.0_f32;
const PLAYER_MAX_SPEED: f32 = 1750.0_f32;
const PLAYER_GROUND_DRAG: f32 = 0.48_f32;
const PLAYER_AIR_DRAG: f32 = 0.58_f32;

// vertical movements
const PLAYER_MAX_JUMP_TIME: f32 = 0.35_f32;
const PLAYER_JUMP_LAUNCH_VEL: f32 = -3500.0_f32;
const PLAYER_GRAVITY_ACCEL: f32 = 3400.0_f32;
const PLAYER_MAX_FALL_SPEED: f32 = 550.0_f32;
const PLAYER_JUMP_POWER: f32 = 0.14_f32;

#[derive(Clone, Copy)]
enum PlayerFrame {
    Idle = 0,
    Run = 1,
    Jump = 2,
    Celebrate = 3,
    Die = 4,
}

enum PlayerDirection {
    Left,
    Right,
}

struct Player {
    pos: glm::Vector2<f64>,
    vel: glm::Vector2<f32>,

    // jumping state
    on_ground: bool,
    is_jumping: bool,
    jump_time: f32,
    previous_bottom: f32,

    sprites: Vec<AnimatedSprite>,
    current: PlayerFrame,
    direction: PlayerDirection,
    level: GameLevel,
}

impl Player {
    pub fn new(phi: &mut Phi) -> Player {
        let sprites = vec![
            SpriteBuilder::new(phi, "assets/sprites/player/idle.png")
                .size(PLAYER_WIDTH, PLAYER_HEIGHT)
                .finalize(),

            SpriteBuilder::new(phi, "assets/sprites/player/run.png")
                 .size(PLAYER_WIDTH, PLAYER_HEIGHT)
                 .fps(PLAYER_FPS)
                 .count(10)
                 .finalize(),

            SpriteBuilder::new(phi, "assets/sprites/player/jump.png")
                .size(PLAYER_WIDTH, PLAYER_HEIGHT)
                .fps(PLAYER_FPS)
                .count(11)
                .finalize(),

            SpriteBuilder::new(phi, "assets/sprites/player/celebrate.png")
                .size(PLAYER_WIDTH, PLAYER_HEIGHT)
                .fps(PLAYER_FPS)
                .count(11)
                .finalize(),

            SpriteBuilder::new(phi, "assets/sprites/player/die.png")
                .size(PLAYER_WIDTH, PLAYER_HEIGHT)
                .fps(PLAYER_FPS)
                .count(11)
                .finalize(),
        ];

        Player {
            pos: glm::Vector2::new(64.0, 64.0),
            vel: glm::Vector2::new(64.0, 64.0),

            on_ground: true,
            is_jumping: false,
            jump_time: 0.0_f32,
            previous_bottom: 0.0_f32,

            sprites: sprites,
            current: PlayerFrame::Idle,
            direction: PlayerDirection::Right,
            level: GameLevel::load(phi, "assets/level-0.txt"),
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

        self.level.update(phi, elapsed);

        // apply physics
        let dx = if phi.events.key_left {
            -1.0f32
        } else if phi.events.key_right {
            1.0f32
        } else {
            0.0f32
        };

        // the base velocity is a combination of horizontal movement control
        // and acceleration downwards due to gravity.
        self.vel.x += dx * PLAYER_MOVE_ACCEL * elapsed as f32;
        self.vel.y = glm::clamp(
            self.vel.y + PLAYER_GRAVITY_ACCEL * elapsed as f32,
            -PLAYER_MAX_FALL_SPEED,
            PLAYER_MAX_FALL_SPEED
        );

        // apply the jump logic
        if phi.events.key_up {
            if (!self.is_jumping && self.on_ground) || self.jump_time > 0.0f32 {
                self.jump_time += elapsed as f32;
            }

            if 0.0_f32 < self.jump_time && self.jump_time <= PLAYER_MAX_JUMP_TIME {
                self.vel.y = PLAYER_JUMP_LAUNCH_VEL *
                    (1.0f32 - glm::pow(self.jump_time / PLAYER_MAX_JUMP_TIME,
                                       PLAYER_JUMP_POWER));
            } else {
                self.jump_time = 0.0_f32;
            }
        } else {
            self.jump_time = 0.0_f32;
        }

        self.vel.x *= if self.on_ground { PLAYER_GROUND_DRAG } else { PLAYER_AIR_DRAG };
        self.vel.x = glm::clamp(self.vel.x, -PLAYER_MAX_SPEED, PLAYER_MAX_SPEED);

        let old_position = self.pos;
        self.pos.x = self.pos.x + self.vel.x as f64 * elapsed;
        self.pos.y = self.pos.y + self.vel.y as f64 * elapsed;

        // handle collisions
        let mut bound_rect = Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        };

        // the values can go out of bounds but we are saved by the fact that
        // self.level.get_collision() handles out of bounds values
        let left_tile = glm::floor(bound_rect.x / TILE_WIDTH) as i32;
        let right_tile = glm::ceil((bound_rect.x + bound_rect.w) / TILE_WIDTH) as i32;
        let top_tile = glm::floor(bound_rect.y / TILE_HEIGHT) as i32;
        let bottom_tile = glm::ceil((bound_rect.y + bound_rect.h) / TILE_HEIGHT) as i32;

        self.on_ground = false;
        let mut tile_bounds = Rectangle {
            x: left_tile as f64 * TILE_WIDTH,
            y: top_tile as f64 * TILE_HEIGHT,
            w: TILE_WIDTH, h: TILE_HEIGHT
        };
        for yth in top_tile..bottom_tile {
            for xth in left_tile..right_tile {
                let collision = self.level.get_collision(xth, yth);

                if collision != TileCollision::Passable {
                    if let Some(depth) = bound_rect.intersection_depth(&tile_bounds) {
                        if depth.y.abs() < depth.x.abs() || collision == TileCollision::Platform {
                            if self.previous_bottom <= tile_bounds.y as f32 {
                                self.on_ground = true;
                            }

                            if collision == TileCollision::Impassable || self.on_ground {
                                self.pos = glm::Vector2 {
                                    x: self.pos.x,
                                    y: self.pos.y + depth.y,
                                };
                                bound_rect.x = self.pos.x;
                                bound_rect.y = self.pos.y;
                            }
                        } else if collision == TileCollision::Impassable {
                            self.pos = glm::Vector2 {
                                x: self.pos.x + depth.x,
                                y: self.pos.y,
                            };
                            bound_rect.x = self.pos.x;
                            bound_rect.y = self.pos.y;
                        }
                    }
                }
                tile_bounds.x += TILE_WIDTH;
            }
            tile_bounds.x = left_tile as f64 * TILE_WIDTH;
            tile_bounds.y += TILE_HEIGHT;
        }
        self.previous_bottom = (self.pos.y + PLAYER_HEIGHT) as f32;

        // reset the velocity if a collision stopped the player
        if old_position.x == self.pos.x {
            self.vel.x = 0f32;
        }

        if old_position.y == self.pos.y {
            self.vel.y = 0f32;
        }

        // determine the facing direction of the player
        if dx == 0.0 && self.on_ground {
            self.current = Idle;
        } else if !self.on_ground {
            self.current = Jump;
        } else {
            self.current = Run;
        }

        if dx < 0.0 {
            self.direction = Left;
        } else if dx > 0.0 {
            self.direction = Right;
        }
        self.sprites[self.current as usize].add_time(elapsed);
    }

    pub fn render(&self, phi: &mut Phi) {
        self.level.render(phi);

        let cursprite = &self.sprites[self.current as usize];
        let rect = Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
        }.to_sdl();

        if DEBUG {
            phi.renderer.set_draw_color(pixels::Color::RGB(200,200,50));
            phi.renderer.fill_rect(rect).unwrap();
        }

        let fx = match self.direction {
            PlayerDirection::Left => RenderFx::None,
            PlayerDirection::Right => RenderFx::FlipX,
        };
        phi.renderer.copy_sprite(cursprite, &rect, fx);
    }
}

const GEM_WIDTH: f64 = 32.0;
const GEM_HEIGHT: f64 = 32.0;

struct Gem {
    sprite: Sprite,
    origin: glm::Vector2<f64>,

    // TODO:
    // collectedSound: Chunk,
    // color: pixels::Color,

    pos: glm::Vector2<f64>,
    time: f64,
    bounce: f64,
}

impl Gem {
    fn new<'a>(sprite: &'a Sprite, pos: glm::Vector2<f64>) -> Gem {
        let sprite = sprite.clone();
        let (width, height) = sprite.size();
        let origin = glm::Vector2::new(width / 2.0, height / 2.0);
        let pos = pos - origin;
        Gem {
            sprite: sprite,
            origin: origin,
            pos: pos,
            time: pos.x * 0.75,
            bounce: 0.0,
        }
    }

    // fn bounding_circle(&self) -> Circle<f64> {
    //     Circle { position: self.pos, TILE_WIDTH / 3.0f }
    // }

    pub fn update(&mut self, phi: &mut Phi, elapsed: f64) {
        use std::f64;
        self.time += elapsed * 6.0;
        while self.time > f64::consts::PI {
            self.time -= f64::consts::PI * 2.0;
        }
        self.bounce = f64::sin(self.time) * GEM_HEIGHT * 0.18;
    }

    pub fn render(&self, phi: &mut Phi) {
         let rect = Rectangle {
            x: self.pos.x,
            y: self.pos.y + self.bounce,
            w: GEM_WIDTH,
            h: GEM_HEIGHT,
        }.to_sdl();
        self.sprite.render(&mut phi.renderer, &rect, RenderFx::None);
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
    fn update(mut self: Box<Self>, phi: &mut Phi, elapsed: f64) -> ViewAction {
        // check if the player wants to exit
        if phi.events.now.quit {
            return ViewAction::Quit
        }

        // check if the player pressed escape
        if phi.events.now.key_escape == Some(true) {
            return ViewAction::Render(Box::new(
                ::views::menu::MenuView::new(phi)))
        }

        // update the player
        self.player.update(phi, elapsed);

        // TODO: update the gems

        // TODO: check if the player fell off the bottom of the level

        // TODO: update the enemies

        ViewAction::Render(self)
    }

    fn render(&self, phi: &mut Phi) {
        // Clear the screen
        phi.renderer.set_draw_color(pixels::Color::RGB(0,0,50));
        phi.renderer.clear();

        // Draw the player
        self.player.render(phi);

        // TODO: Draw the enemies
    }
}
