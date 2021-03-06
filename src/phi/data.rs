// src/phi/data.rs

use glm::Vector2;
use sdl2::rect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn to_sdl(self) -> rect::Rect {
        assert!(self.w >= 0.0 && self.h >= 0.0);

        rect::Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }

    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        if self.w > parent.w || self.h > parent.h {
            return None
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,
            x: if self.x < parent.x { parent.x }
            else if self.x + self.w >= parent.x + parent.w { parent.x + parent.w - self.w }
            else { self.x },
            y: if self.y < parent.y { parent.y }
            else if self.y + self.h >= parent.y + parent.h { parent.y + parent.h - self.h }
            else { self.y },
        })
    }

    pub fn contains(&self, rect: Rectangle) -> bool {
        let xmin = rect.x;
        let xmax = xmin + rect.w;
        let ymin = rect.y;
        let ymax = ymin + rect.h;

        xmin >= self.x && xmin <= self.x + self.w &&
            xmax >= self.x && xmax <= self.x + self.w &&
            ymin >= self.y && ymin <= self.y + self.h &&
            ymax >= self.y && ymax <= self.y + self.h
    }

    pub fn overlaps(&self, other: Rectangle) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }

    pub fn with_size(w: f64, h: f64) -> Rectangle {
        Rectangle {
            w: w,
            h: h,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn center_at(self, center: Vector2<f64>) -> Rectangle {
        Rectangle {
            x: center.x - self.w / 2.0,
            y: center.y - self.h / 2.0,
            ..self
        }
    }

    pub fn center(self) -> Vector2<f64> {
        Vector2 {
            x: self.x + self.w / 2.0,
            y: self.y + self.h / 2.0,
        }
    }

    /// Signed depth of intersection between two rectangles.
    ///
    /// The function returns the amount of overlap between two rectangles.
    /// The amount can be negative depending on which sides the rectangles
    /// intersect.
    /// The caller can determine the correct direction to push the objects
    /// in order to resolve the collisions.
    /// If the rectangles don't intersect a vector of zero is returned.
    pub fn intersection_depth(&self, other: &Rectangle) -> Option<Vector2<f64>> {

        let center_a = self.center();
        let center_b = other.center();

        // distance between the centers
        let dis = center_a - center_b;

        // minimum non intersecting distance
        let min = Vector2 {
            x: (self.w + other.w) / 2.0,
            y: (self.h + other.h) / 2.0,
        };

        if dis.x.abs() >= min.x || dis.y.abs() >= min.y {
            None
        } else {
            Some(Vector2 {
                x: if dis.x > 0.0 { min.x - dis.x } else { -min.x - dis.x },
                y: if dis.y > 0.0 { min.y - dis.y } else { -min.y - dis.y },
            })
        }
    }
}

pub struct MaybeAlive<T> {
    pub alive: bool,
    pub value: T,
}

impl<T> MaybeAlive<T> {
    // check wether the value is still alive, if this is the case
    // then return `Some(value)`; otherwise return `None`
    pub fn as_option(self) -> Option<T> {
        if self.alive {
            Some(self.value)
        } else {
            None
        }
    }
}
