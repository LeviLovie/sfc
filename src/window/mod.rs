extern crate glfw;
extern crate gl;
use glfw::Context;

use super::logger::*;

pub struct VRAM {
    // 256x256 pixels
    // 0xBBGGRR
    pub data: [u8; 256*256],
}
impl VRAM {
    pub fn new() -> VRAM {
        VRAM {
            data: [0x000000; 256*256],
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        self.data[(y) * 256 + (x)] = color;
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.data[(y) * 256 + (x)]
    }
    pub fn clear(&mut self) {
        self.data = [0x000000; 256*256];
    }

    pub fn rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u8) {
        for i in x..x+w {
            for j in y..y+h {
                self.set_pixel(i, j, color);
            }
        }
    }
}

pub struct Window {
    pub vram: VRAM,
    pub width: usize,
    pub height: usize,
    pub scale: usize,
    pub title: String,
    glfw: glfw::Glfw,
    window: glfw::Window,
}
impl Window {
    pub fn new(scale: usize, title: String) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let window = glfw.create_window(
            (256 * scale) as u32,
            (256 * scale) as u32,
            &title,
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");

        Window {
            vram: VRAM::new(),
            width: 256,
            height: 256,
            scale: scale,
            title: title,
            glfw: glfw,
            window: window.0,
        }
    }

    pub fn init(&mut self) {
        log(LEVEL_DEBUG, "Initializing window...");

        self.window.make_current();

        gl::load_with(|s| self.window.get_proc_address(s) as *const _);
        unsafe {
            gl::Viewport(0, 0, (self.height * self.scale) as i32, (self.width * self.scale) as i32);
        }

        while !self.window.should_close() {
            self.glfw.poll_events();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.window.swap_buffers();
        }
    }
}