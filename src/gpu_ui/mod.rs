//! GPU immediate-mode UI demo (winit + wgpu only):
//!
//! 1. winit window + wgpu colored triangle
//! 2. WGSL shape shader for rectangles and circles
//! 3. mouse hit-testing to recolor buttons on click
//! 4. bitmap text via instanced pixel quads ("Click Me")
//! 5. box-model flex row layout for multiple buttons

mod app;
mod async_utils;
mod layout;
mod renderer;
mod shapes;
mod text;

pub use app::run;
