// phi/mod.rs
use self::gfx::Sprite;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::ttf::Sdl2TtfContext;

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
    pub font_context: Sdl2TtfContext,

//    cached_fonts: HashMap<(&'static str, u16), ::sdl2::ttf::Font<'window>>,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        Phi {
            events: events,
            renderer: renderer,
            font_context: ::sdl2::ttf::init().unwrap(),
//            cached_fonts: HashMap::new(),
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

    pub fn ttf_str_sprite(&mut self, text: &str, font_path: &'static str, size: u16, color: Color) -> Option<Sprite> {
        // if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
        //     return font.render(text).blended(color).ok()
        //         .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
        //         .map(Sprite::new)
        // }

        self.font_context.load_font(Path::new(font_path), size).ok()
            .and_then(|font| font
                      .render(text).blended(color).ok()
                      .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
                      .map(Sprite::new))
                // if this worked we cache the font acquired
                // self.cached_fonts.insert((font_path, size), font);

                // then we call the method recursively since
                // we know that now the font is cached
                // self.ttf_str_sprite(text, font_path, size, color)
            // })
    }
}

pub enum ViewAction {
    Render(Box<View>),
    Quit,
}

pub trait View {
    /// Called on every frame to take care of the logic of the program. From
    /// user inputs and the instance's internal state, determine whether to
    /// render itself or another view, close the window, etc.
    ///
    /// `elapsed` is expressed in seconds.
    fn update(self: Box<Self>, context: &mut Phi, elapsed: f64) -> ViewAction;


    /// Called on every frame to take care rendering the current view. It
    /// disallows mutating the object by default, although you may still do it
    /// through a `RefCell` if you need to.
    fn render(&self, context: &mut Phi);
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

    let _image_context = ::sdl2::image::init(::sdl2::image::INIT_PNG).unwrap();

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

        match current_view.update(&mut context, elapsed) {
            ViewAction::Render(view) => {
                current_view = view;
                current_view.render(&mut context);
                context.renderer.present();
            },

            ViewAction::Quit =>
                break,
        }
    }
}
