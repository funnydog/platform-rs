// phi/mod.rs
use self::gfx::Sprite;
use sdl2::render::Renderer;
use sdl2::pixels::Color;

use std::collections::HashMap;
use std::path::Path;

#[macro_use]
mod events;

pub mod data;
pub mod gfx;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space,

        key_1: Num1,
        key_2: Num2,
        key_3: Num3
    },
    else: {
        quit: Quit { .. }
    }
}

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,

    cached_fonts: HashMap<(&'static str, i32), ::sdl2_ttf::Font>,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        Phi {
            events: events,
            renderer: renderer,
            cached_fonts: HashMap::new(),
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

    pub fn ttf_str_sprite(&mut self, text: &str, font_path: &'static str, size: i32, color: Color) -> Option<Sprite> {
        if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
            return font.render(text, ::sdl2_ttf::blended(color)).ok()
                .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
                .map(Sprite::new)
        }

        ::sdl2_ttf::Font::from_file(Path::new(font_path), size).ok()
            .and_then(|font| {
                // if this worked we cache the font acquired
                self.cached_fonts.insert((font_path, size), font);

                // then we call the method recursively since
                // we know that now the font is cached
                self.ttf_str_sprite(text, font_path, size, color)
            })
    }
}

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

/// Create a window with name `title`, initialize the underlying libraries and
/// start the game with the `View` returned by `init()`.
///
/// # Examples
///
/// Here, we simply show a window with color #ffff00 and exit when escape is
/// pressed or when the window is closed
///
/// ```
///
/// struct MyView;
///
/// impl View for MyView {
///     fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
///         if context.events.now.quit {
///             return ViewAction::Quit;
///         }
///
///         context.renderer.set_draw_color(Color::RGB(255, 255, 0));
///         context.renderer.clear();
///         ViewAction::None
///     }
/// }
///
/// spawn("Example", |_| {
///     Box::new(MyView)
/// });
/// ```

pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View> {

    // initialize SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let _image_context = ::sdl2_image::init(::sdl2_image::INIT_PNG).unwrap();
    let _ttf_context = ::sdl2_ttf::init().unwrap();

    // create the window
    let window = video.window(title, 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    // Create the context
    let mut context = Phi::new(
        Events::new(sdl_context.event_pump().unwrap()),
        window.renderer().accelerated().build().unwrap()
    );

    // Create the default view
    let mut current_view = init(&mut context);

    //  Frame timing
    let interval = 1_000_000_u64 / 60_u64;
    let mut before = ::time::precise_time_ns() / 1000u64;
    let mut last_second = before;
    let mut fps = 0u16;

    loop {
        // Frame timing
        let now = ::time::precise_time_ns() / 1000u64;
        let dt = now - before;
        let elapsed = dt as f64 / 1_000_000.0;

        if dt < interval {
            ::std::thread::sleep(::std::time::Duration::new(0, (interval - dt) as u32));
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        // logic and rendering
        context.events.pump(&mut context.renderer);
        match current_view.render(&mut context, elapsed) {
            ViewAction::None =>
                context.renderer.present(),

            ViewAction::Quit =>
                break,

            ViewAction::ChangeView(new_view) => {
                current_view = new_view;
            }
        }
    }
}
