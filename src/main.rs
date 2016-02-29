// src/main.rs

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate time;

mod phi;
mod views;

fn main() {
    ::phi::spawn("Platform-RS platformer", |phi| {
        Box::new(::views::menu::MenuView::new(phi))
    });
}
