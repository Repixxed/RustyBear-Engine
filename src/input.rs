use std::collections::HashMap;

use winit::event::ElementState;

use crate::{event::{EventSubscriber, Event, GamepadButtonState}, context::{Context}};

#[derive(Default)]
pub struct InputState {
    keyboard: HashMap<winit::event::VirtualKeyCode, bool>,
    mouse_button: HashMap<winit::event::MouseButton, bool>,
    gamepad_button: HashMap<gilrs::Button, bool>,
    gamepad_axis: HashMap<gilrs::Axis, f32>,
    mouse_position: (f64, f64),
}

impl InputState {

    pub fn new() -> InputState
    {
        InputState::default()
    }

    pub fn is_key_down(&self, keycode: &winit::event::VirtualKeyCode) -> bool {
        *self.keyboard.get(keycode).unwrap_or(&false)
    }

    pub fn is_mouse_down(&self, keycode: &winit::event::MouseButton) -> bool {
        *self.mouse_button.get(keycode).unwrap_or(&false)
    }

    pub fn is_gamepad_butto_down(&self, keycode: &gilrs::Button) -> bool {
        *self.gamepad_button.get(keycode).unwrap_or(&false)
    }

    pub fn get_gamepad_axis(&self, axiscode: &gilrs::Axis) -> f32 {
        *self.gamepad_axis.get(axiscode).unwrap_or(&0.0)
    }

    pub fn get_mouse_pos(&self) -> (f64, f64) {
        self.mouse_position
    }
}

impl EventSubscriber for InputState {
    fn on_event(&mut self, event: &Event, _context: &Context) -> bool
    {
        match event {
            Event::KeyboardInput { keycode, state } =>
            {
                if *state == ElementState::Pressed
                {
                    self.keyboard.insert(*keycode, true);
                }
                else 
                {
                    self.keyboard.insert(*keycode, false);
                }
            },
            Event::MouseInput { mousecode, state } =>
            {
                if *state == ElementState::Pressed
                {
                    self.mouse_button.insert(*mousecode, true);
                }
                else 
                {
                    self.mouse_button.insert(*mousecode, false);
                }
            },
            Event::CursorMoved { x, y } => {
                self.mouse_position = (*x, *y);
            },
            Event::GamepadInput { buttoncode, state, .. } => {
                if *state == GamepadButtonState::Pressed
                {
                    self.gamepad_button.insert(*buttoncode, true);
                }
                else
                {
                    self.gamepad_button.insert(*buttoncode, false);    
                }
            },
            Event::GamepadAxis { axiscode, value, .. } => {
                self.gamepad_axis.insert(*axiscode, *value);
            }
            _ => {}
        }

        true
    }
}
