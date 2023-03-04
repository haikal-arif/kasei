use std::borrow::BorrowMut;

use sdl2::{self, render::Canvas, video::Window, VideoSubsystem};

pub struct WindowManager {
    canvas: Canvas<Window>,
}

impl WindowManager {
    pub fn new(
        video_subsystem: &VideoSubsystem,
        title: &str,
        width: u32,
        height: u32,
        fullscreen: bool,
    ) -> Result<Self, String> {
        let mut binding = video_subsystem.window(title, width, height);
        let window_builder = binding.position_centered();
        let window_builder = match fullscreen {
            true => window_builder.fullscreen(),
            false => window_builder,
        };
        let window = window_builder.build().map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        Ok(WindowManager { canvas })
    }

    pub fn get_canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut Canvas<Window> {
        self.canvas.borrow_mut()
    }
}
