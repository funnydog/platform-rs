// src/phi/gfx.rs

use phi::data::Rectangle;
use phi::Phi;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use sdl2::rect::Rect as SdlRect;
use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;

pub enum RenderFx {
    FlipX,
    None,
}

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer, dest: &SdlRect, fx: RenderFx);
}

#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

impl Sprite {
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: tex_query.width as f64,
                h: tex_query.height as f64,
                x: 0.0,
                y: 0.0,
            }
        }
    }

    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = Rectangle {
            x: rect.x + self.src.x,
            y: rect.y + self.src.y,
            ..rect
        };

        if self.src.contains(new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src,
            })
        } else {
            None
        }
    }

    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }
}

impl Renderable for Sprite {
    fn render(&self, renderer: &mut Renderer, dest: &SdlRect, fx: RenderFx) {
        match fx {
            RenderFx::None => {
                renderer.copy(
                    &mut self.tex.borrow_mut(),
                    Some(self.src.to_sdl()),
                    Some(*dest));
            },

            RenderFx::FlipX => {
                renderer.copy_ex(
                    &mut self.tex.borrow_mut(),
                    Some(self.src.to_sdl()),
                    Some(*dest),
                    0.0,
                    None,
                    true,
                    false).unwrap();
            }
        }
    }
}

pub enum ASDescr<'a> {
    LoadFromStart {
        image_path: &'a str,
        total_frames: usize,
        frames_high: usize,
        frames_wide: usize,
        frame_w: f64,
        frame_h: f64,
    },

    SingleFrame {
        image_path: &'a str,
        frame_x: f64,
        frame_y: f64,
        frame_w: f64,
        frame_h: f64,
    },

    LoadFromRegion {
        image_path: &'a str,
        rect: Rectangle,
        total_frames: usize,
        frame_w: f64,
        frame_h: f64,
    },
}

#[derive(Clone)]
pub struct AnimatedSprite {
    // frames to be rendered in order
    sprites: Rc<Vec<Sprite>>,

    // the time it takes to get from one frame to the next in seconds
    frame_delay: f64,

    // the total time the sprite has been alive from which
    // the current frame is derived
    current_time: f64,
    max_time: f64,
}

impl AnimatedSprite {
    pub fn new(sprites: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
        let max_time = sprites.len() as f64 * frame_delay;
        AnimatedSprite {
            sprites: Rc::new(sprites),
            frame_delay: frame_delay,
            current_time: 0.0,
            max_time: max_time,
        }
    }

    // number of frames composing the animation.
    pub fn frames(&self) -> usize {
        self.sprites.len()
    }

    // set the time it takes to get from one frame to the next in seconds
    // if the value is negative then we "rewind" the animation
    pub fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }

    // set the number of frames the animation goes through every second.
    // if the value is negative then we "rewind" the animation
    pub fn set_fps(&mut self, fps: f64) {
        if fps == 0.0 {
            panic!("Passed 0.0 to AnimatedSprite::set_fps()");
        }
        self.set_frame_delay(1.0 / fps);
    }

    // Add a certain amount of time, in second, to the `current_time` of the
    // animated sprite, so that it knows when it must go to the next frame.
    pub fn add_time(&mut self, dt: f64) {
        self.current_time += dt;

        if self.current_time < 0.0 {
            self.current_time += self.max_time;
        } else if self.current_time >= self.max_time {
            self.current_time -= self.max_time;
        }
    }

    pub fn load(phi: &mut Phi, descr: ASDescr, fps: f64) -> AnimatedSprite {
        match descr {
            // many frames starting from 0,0
            ASDescr::LoadFromStart {
                image_path,
                total_frames,
                frames_high,
                frames_wide,
                frame_w,
                frame_h
            } => {
                let spritesheet = Sprite::load(&mut phi.renderer, image_path).unwrap();
                let mut frames = Vec::with_capacity(total_frames);

                for yth in 0..frames_high {
                    for xth in 0..frames_wide {
                        if frames_wide * yth + xth >= total_frames {
                            break;
                        }

                        frames.push(
                            spritesheet.region(Rectangle {
                                w: frame_w,
                                h: frame_h,
                                x: frame_w * xth as f64,
                                y: frame_h * yth as f64,
                            }).unwrap());
                    }
                }

                AnimatedSprite::new(frames, 1.0 / fps)
            },

            ASDescr::LoadFromRegion {
                image_path,
                rect,
                total_frames,
                frame_w,
                frame_h,
            } => {
                let spritesheet = Sprite::load(&mut phi.renderer, image_path).unwrap();
                let mut frames = Vec::with_capacity(total_frames);
                let mut region = Rectangle {
                    x: rect.x,
                    y: rect.y,
                    w: frame_w,
                    h: frame_h,
                };

                for _ in 0..total_frames {
                    frames.push(spritesheet.region(region).unwrap());
                    if region.x + region.w + frame_w > rect.x + rect.w {
                        region.y = rect.y;
                        region.x = rect.x;

                        if region.y + region.h > rect.y + rect.w {
                            panic!("region exceeded");
                        }
                    }
                }

                AnimatedSprite::new(frames, 1.0 / fps)
            },

            // one still frame
            ASDescr::SingleFrame {
                image_path,
                frame_x,
                frame_y,
                frame_w,
                frame_h,
            } => {
                let spritesheet = Sprite::load(&mut phi.renderer, image_path).unwrap();
                AnimatedSprite::new(
                    vec![
                        spritesheet.region(Rectangle {
                            x: frame_x,
                            y: frame_y,
                            w: frame_w,
                            h: frame_h,
                        }).unwrap(),
                    ], 0.0)
            },
        }
    }
}

impl Renderable for AnimatedSprite {
    fn render(&self, renderer: &mut Renderer, dest: &SdlRect, fx: RenderFx) {
        let current_frame = (self.current_time  / self.frame_delay) as usize % self.frames();
        let sprite = &self.sprites[current_frame];

        sprite.render(renderer, dest, fx);
    }
}

pub trait CopySprite<T> {
    fn copy_sprite(&mut self, renderable: &T, dest: &SdlRect, fx: RenderFx);
}

impl<'window, T: Renderable> CopySprite<T> for Renderer<'window> {
    fn copy_sprite(&mut self, renderable: &T, dest: &SdlRect, fx: RenderFx) {
        renderable.render(self, dest, fx);
    }
}

pub struct SpriteBuilder {
    spritesheet: Sprite,
    region: Rectangle,
    width: f64,
    height: f64,
    number: usize,
    fps: f64,
}

impl SpriteBuilder {
    pub fn new<'a>(phi: &mut Phi, path: &'a str) -> SpriteBuilder {
        let spritesheet = Sprite::load(&mut phi.renderer, path).unwrap();
        let (width, height) = spritesheet.size();
        let region = Rectangle::with_size(width, height);

        SpriteBuilder {
            spritesheet: spritesheet,
            region: region,
            width: width,
            height: height,
            number: 1,
            fps: 1.0f64,
        }
    }

    pub fn region_from(&mut self, x: f64, y: f64) -> &mut SpriteBuilder {
        self.region.w += self.region.x - x;
        self.region.h += self.region.y - y;
        self.region.x = x;
        self.region.y = y;
        self
    }

    pub fn region_size(&mut self, w: f64, h: f64) -> &mut SpriteBuilder {
        let (width, height) = self.spritesheet.size();
        assert!(self.region.x + w <= width);
        assert!(self.region.y + h <= height);
        self.region.w = w;
        self.region.h = h;
        self
    }

    pub fn size(&mut self, w: f64, h: f64) -> &mut SpriteBuilder {
        assert!(w <= self.region.w);
        assert!(h <= self.region.h);
        self.width = w;
        self.height = h;
        self
    }

    pub fn count(&mut self, number: usize) -> &mut SpriteBuilder {
        self.number = number;
        self
    }

    pub fn fps(&mut self, fps: f64) -> &mut SpriteBuilder {
        self.fps = fps;
        self
    }

    pub fn finalize(&self) -> AnimatedSprite {
        let mut frames = Vec::with_capacity(self.number);

        let mut frame = Rectangle { w: self.width, h: self.height, .. self.region };
        for cnt in 0..self.number {
            frames.push(self.spritesheet.region(frame).unwrap());
            frame.x += self.width;
            if !self.region.contains(frame) {
                frame.x = self.region.x;
                frame.y += self.height;
                if !self.region.contains(frame) {
                    assert!(cnt+1 == self.number);
                    break;
                }
            }
        }

        AnimatedSprite::new(frames, 1.0 / self.fps)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileCollision {
    /// a tile which doesn't hinder player motion at all
    Passable = 0,

    /// a tile which doesn't allow the player to move through
    /// it at all. It's completely solid
    Impassable = 1,

    /// A tile which behaves like a passable tile except when the
    /// player is above it. A player can jump up through a platform
    /// as weel as move past it to the left and right, but cannot
    /// fall down through the top of it.
    Platform = 2,
}

pub struct Tile {
    pub sprite: Option<Sprite>,
    pub collision: TileCollision,
}

impl Tile {
    pub fn new(sprite: Option<Sprite>, collision: TileCollision) -> Tile {
        Tile {
            sprite: sprite,
            collision: collision,
        }
    }

    pub fn load(renderer: &Renderer, path: &str, collision: TileCollision) -> Tile {
        let sprite = match Sprite::load(renderer, path) {
            Some(sprite) => { Some(sprite) },
            None => {
                panic!("Sprite {} not found!", path);
            }
        };

        Tile {
            sprite: sprite,
            collision: collision,
        }
    }
}

impl Renderable for Tile {
    fn render(&self, renderer: &mut Renderer, dest: &SdlRect, fx: RenderFx) {
        if let Some(ref sprite) = self.sprite {
            sprite.render(renderer, dest, fx);
        }
    }
}
