#![allow(non_snake_case)]

pub mod utils;
#[macro_use] pub mod core;
pub mod event;
pub mod window;
pub mod logging;
pub mod entry;
pub mod input;
pub mod context;
pub mod render;
pub mod sound;
pub mod config;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use rccell::RcCell;
use render::renderer::Renderer;

use crate::{core::{Application}, context::Context, config::load_themes, sound::AudioEngine};

use event::EventSubscriber;
use window::Window;
use winit::{event::{VirtualKeyCode, ElementState, MouseButton}};

use crate::core::ModuleStack;

struct MyHandler{
    audio: AudioEngine
}

impl EventSubscriber for MyHandler {
    fn on_event(&mut self, event: &event::Event, _context: &Context) -> bool
    {
        if let event::Event::KeyboardInput { keycode, state } = event {
            match keycode {
                VirtualKeyCode::D => if state == &ElementState::Pressed { log::info!("Pressed the d") },
                VirtualKeyCode::E => if state == &ElementState::Pressed { log::info!("Pressed the e") },
                _ => {}
            }
        }

        if let event::Event::MouseInput { mousecode, state } = event {
            match mousecode {
                MouseButton::Left => if state == &ElementState::Pressed { self.audio.play_click() },
                MouseButton::Right => if state == &ElementState::Pressed { self.audio.play_click() },
                _ => {}
            }
        }
        false
    }
}

impl MyHandler {
    pub fn new() -> MyHandler
    {
        let theme_conf = load_themes();

        let mut audio = AudioEngine::new(&theme_conf);
        audio.play_background();

        MyHandler { audio }
    }
}

struct MyApp<'a> {
    stack: ModuleStack<'a>,
    renderer: RcCell<Renderer>,
}

impl<'a> Application<'a> for MyApp<'a> { 

    fn render(&mut self, view: wgpu::TextureView, context: &mut Context)
    {
        {
            let mut renderer = self.renderer.borrow_mut();
            renderer.render(context, view);
        }
    } 

    fn get_stack(&mut self) -> &mut ModuleStack<'a>
    {
        &mut self.stack
    }

    fn update(&mut self, _delta: &utils::Timestep) {}

    fn quit(&mut self) {}
}

impl<'a> MyApp<'a> {
    pub fn new(context: &Context) -> MyApp<'a>
    {
        log::info!("Init Application");

        let mut  stack = ModuleStack::new();

        let handler = RcCell::new(MyHandler::new());
        stack.subscribe(event::EventType::Layer, handler);

        let renderer = RcCell::new(Renderer::new(context));
        stack.subscribe(event::EventType::Layer, renderer.clone());

        MyApp { stack, renderer }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn entry_point() {
    logging::init();
    println!();

    //Create the window from the config and create the context.
    let mut window = Window::new("{}".to_string());
    let context = pollster::block_on(Context::new(&mut window));

    //Create and init the application
    let myapp = MyApp::new(&context);

    //Move my app and window into the context. And run the app.
    context.run(myapp, window);
}

