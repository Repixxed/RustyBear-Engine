use gilrs::EventType;
use serde::{Serialize, Deserialize};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder}, dpi::{PhysicalSize, LogicalPosition}, event::{WindowEvent, MouseScrollDelta}};
use winit_fullscreen::WindowFullScreen;

use crate::{core::ModuleStack, event, context::{Context}};

#[derive(Serialize, Deserialize)]
pub struct WindowConfig {
    pub size: (u32, u32),
    pub title: String,
    pub position: (f64, f64),
    pub resizeable: bool,
    pub fullscreen: bool,
    pub visible: bool,
    pub border: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig { title: "RustyBear-Sandbox".to_string(), size: (1280, 720), position: (0.0, 0.0), resizeable: true, fullscreen: false, visible: true, border: true }
    }
}


pub struct Window {
    pub native: winit::window::Window,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {

    pub fn new(config_json: String) -> Window
    {
        let json_unchecked = serde_json::from_str(&config_json);
        
        if json_unchecked.is_err() {
            log::error!("Failed to parse window config. Defaulting...");
        }
        
        let window_config: WindowConfig = json_unchecked.unwrap_or(Default::default());

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title(window_config.title)
        .with_inner_size(PhysicalSize{width: window_config.size.0, height: window_config.size.1})
        .with_position(LogicalPosition{x: window_config.position.0, y: window_config.position.1})
        .with_resizable(window_config.resizeable)
        .with_visible(window_config.visible)
        .with_decorations(window_config.border).build(&event_loop).unwrap();

        if window.fullscreen().is_some() ^ window_config.fullscreen {
            window.toggle_fullscreen();
        }

        #[cfg(target_arch = "wasm32")]
        {
    
            window.set_inner_size(PhysicalSize{width: window_config.size.0, height: window_config.size.1});
    
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("rusty-bear-engine")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to the document body.");
        }

        Window { native: window, event_loop }
    }

    pub fn dispatch_gamepad_event(apps: &mut ModuleStack, event: &gilrs::Event, _control_flow: &mut ControlFlow, context: &Context) -> bool
    {
        match event.event {
            EventType::Connected => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::GamepadConnected { id: event.id }, context)
            },
            EventType::Disconnected => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::GamepadDisconnected { id: event.id }, context)
            },
            EventType::ButtonPressed(button, ..) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::GamepadInput { id: event.id, buttoncode: button, state: event::GamepadButtonState::Pressed }, context)
            },
            EventType::ButtonReleased(button, ..) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::GamepadInput { id: event.id, buttoncode: button, state: event::GamepadButtonState::Released }, context)
            },
            EventType::ButtonRepeated(button, ..) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::GamepadInput { id: event.id, buttoncode: button, state: event::GamepadButtonState::Repeated }, context)
            },
            EventType::ButtonChanged(button, value, ..) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::GamepadInputChanged { id: event.id, scancode: button as u32, value }, context)
            },
            EventType::AxisChanged(axis, value, ..) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::GamepadAxis { id: event.id, axiscode: axis, value }, context)
            },
            _ => {false}
        }
    }

    pub fn dispatch_event(apps: &mut ModuleStack, event: &WindowEvent, control_flow: &mut ControlFlow, context: &Context) -> bool
    {
        match event {
            WindowEvent::Resized(size) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::Resized { width: size.width, height: size.height }, context)
            },
            WindowEvent::Moved(pos) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::Moved { x: pos.x, y: pos.y }, context)
            },
            WindowEvent::CloseRequested => {
                let return_value = apps.dispatch_event(event::EventType::Layer, &event::Event::CloseRequested, context);
                control_flow.set_exit();

                return_value
            },
            WindowEvent::Destroyed => {
                let return_value = apps.dispatch_event(event::EventType::Layer, &event::Event::Destroyed, context);
                control_flow.set_exit();

                return_value
            },
            WindowEvent::DroppedFile(path) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::DroppedFile(path.clone()), context)
            },
            WindowEvent::HoveredFile(path) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::HoveredFile(path.clone()), context)
            },
            WindowEvent::HoveredFileCancelled => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::HoveredFileCancelled, context)
            },
            WindowEvent::ReceivedCharacter(ch) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::ReceivedCharacter(*ch), context)
            },
            WindowEvent::Focused( focused) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::Focused(*focused), context)
            },
            WindowEvent::KeyboardInput { input, .. } => {
                if input.virtual_keycode.is_some() {
                    apps.dispatch_event(event::EventType::Layer, &event::Event::KeyboardInput { keycode: input.virtual_keycode.unwrap(), state: input.state }, context)
                } else {
                    false
                }
            },
            WindowEvent::ModifiersChanged( state ) => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::ModifiersChanged(*state), context)
            },
            WindowEvent::CursorMoved { position, .. } => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::CursorMoved { x: position.x, y: position.y }, context)
            },
            WindowEvent::CursorEntered { .. } => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::CursorEntered, context)
            },
            WindowEvent::CursorLeft { .. } => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::CursorLeft, context)
            },
            WindowEvent::MouseWheel { delta, phase, .. } => {
                match delta {
                    MouseScrollDelta::PixelDelta( d) => {
                        apps.dispatch_event(event::EventType::Layer, &event::Event::MouseWheel { delta_x: d.x, delta_y: d.y, state: *phase}, context)
                    },
                    MouseScrollDelta::LineDelta(x, y) => {
                        apps.dispatch_event(event::EventType::Layer, &event::Event::MouseScroll { delta_x: *x, delta_y: *y, state: *phase}, context)
                    }
                }  
            },
            WindowEvent::MouseInput { state, button, .. } => {
                apps.dispatch_event(event::EventType::Layer, &event::Event::MouseInput { mousecode: *button, state: *state }, context)
            }

            _ => {false}
        }
    }
}