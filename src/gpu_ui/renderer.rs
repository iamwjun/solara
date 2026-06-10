use std::num::NonZeroU32;
use std::sync::Arc;

use softbuffer::{Context, Surface};
use winit::event_loop::OwnedDisplayHandle;
use winit::window::Window;

use crate::gpu_ui::draw::{clear, color_to_pixel, draw_triangle_demo, fill_circle};
use crate::gpu_ui::layout::Button;
use crate::gpu_ui::shapes::draw_button;

pub struct DemoCircle {
    pub center_x: f32,
    pub center_y: f32,
    pub diameter: f32,
    pub color: [f32; 4],
}

pub struct Renderer {
    surface: Surface<OwnedDisplayHandle, Arc<Window>>,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(context: &Context<OwnedDisplayHandle>, window: Arc<Window>, width: u32, height: u32) -> Self {
        let mut surface = Surface::new(context, window).expect("failed to create surface");
        if let (Some(width), Some(height)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            surface.resize(width, height).expect("failed to resize surface");
        }

        Self { surface, width, height }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.width = width;
        self.height = height;
        if let (Some(width), Some(height)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            self.surface
                .resize(width, height)
                .expect("failed to resize surface");
        }
    }

    pub fn render(
        &mut self,
        buttons: &[Button],
        demo_circle: &DemoCircle,
    ) -> Result<(), softbuffer::SoftBufferError> {
        let mut buffer = self.surface.buffer_mut()?;
        let width = self.width;
        let height = self.height;

        clear(&mut buffer, color_to_pixel(0.08, 0.09, 0.12));
        draw_triangle_demo(&mut buffer, width, height);

        for button in buttons {
            draw_button(&mut buffer, width, height, button);
        }

        fill_circle(
            &mut buffer,
            width,
            height,
            demo_circle.center_x,
            demo_circle.center_y,
            demo_circle.diameter,
            demo_circle.color,
        );

        buffer.present()
    }
}
