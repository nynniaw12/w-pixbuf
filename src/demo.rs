use crate::{
    draw::{draw_circle, draw_square, draw_triangle},
    init::State,
};
use winit::{
    event::*,
    keyboard::{KeyCode, PhysicalKey},
};

pub enum Shape {
    CIRCLE,
    TRIANGLE,
    SQUARE,
}

pub struct Color(u32);
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub fn get_components(&self) -> (u8, u8, u8) {
        let r = ((self.0 >> 16) & 0xFF) as u8;
        let g = ((self.0 >> 8) & 0xFF) as u8;
        let b = (self.0 & 0xFF) as u8;
        (r, g, b)
    }

    pub fn cycle(&mut self, amount: i32) {
        let (r, g, b) = self.get_components();
        let new_r = ((r as i32 + amount) % 256).abs() as u8;
        let new_g = ((g as i32 + amount / 2) % 256).abs() as u8;
        let new_b = ((b as i32 + amount / 3) % 256).abs() as u8;
        self.0 = ((new_r as u32) << 16) | ((new_g as u32) << 8) | (new_b as u32);
    }
}

pub struct DemoApp {
    pub shape: Shape,
    pub color: Color,
}

impl DemoApp {
    pub fn new() -> Self {
        Self {
            color: Color::new(255, 255, 255),
            shape: Shape::CIRCLE,
        }
    }
    pub fn c(&mut self) {
        self.shape = Shape::CIRCLE;
    }

    pub fn t(&mut self) {
        self.shape = Shape::TRIANGLE;
    }

    pub fn s(&mut self) {
        self.shape = Shape::SQUARE;
    }
    pub fn up(&mut self) {
        self.color.cycle(25);
    }

    pub fn down(&mut self) {
        self.color.cycle(-25);
    }
}

pub fn demo_input_handler(state: &mut State<DemoApp>, event: &WindowEvent) -> bool {
    match event {
        WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: keycode,
                    ..
                },
            ..
        } => {
            match keycode {
                PhysicalKey::Code(KeyCode::KeyC) => {
                    state.game_context.c();
                    true
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    state.game_context.s();
                    true
                }
                PhysicalKey::Code(KeyCode::KeyT) => {
                    state.game_context.t();
                    true
                }
                PhysicalKey::Code(KeyCode::ArrowUp) => {
                    state.game_context.up();
                    true
                }
                PhysicalKey::Code(KeyCode::ArrowDown) => {
                    state.game_context.down();
                    true
                }
                _ => false, // Other key presses
            }
        }
        _ => false,
    }
}

pub fn demo_update_handler(state: &mut State<DemoApp>) {
    match state.game_context.shape {
        Shape::CIRCLE => {
            draw_circle(state);
        }
        Shape::SQUARE => {
            draw_square(state);
        }
        Shape::TRIANGLE => {
            draw_triangle(state);
        }
    }
}
