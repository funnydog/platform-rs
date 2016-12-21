// src/main.rs

extern crate glm;

#[macro_use]
extern crate lazy_static;

extern crate rand;
extern crate sdl2;
extern crate time;

mod phi;
mod views;

fn main() {
    ::phi::spawn("Platform-RS platformer", |phi| {
        Box::new(::views::menu::MenuView::new(phi))
    });
}
