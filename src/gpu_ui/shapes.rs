use crate::gpu_ui::layout::{Button, Rect};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScreenUniform {
    pub size: [f32; 2],
    pub _pad: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShapeInstance {
    pub pos_size: [f32; 4],
    pub color: [f32; 4],
    pub shape_type: u32,
    pub _pad: u32,
}

pub const SHAPE_RECT: u32 = 0;
pub const SHAPE_CIRCLE: u32 = 1;

impl ShapeInstance {
    pub fn rect(rect: Rect, color: [f32; 4]) -> Self {
        Self {
            pos_size: [rect.x, rect.y, rect.width, rect.height],
            color,
            shape_type: SHAPE_RECT,
            _pad: 0,
        }
    }

    pub fn circle(center_x: f32, center_y: f32, diameter: f32, color: [f32; 4]) -> Self {
        Self {
            pos_size: [
                center_x - diameter * 0.5,
                center_y - diameter * 0.5,
                diameter,
                diameter,
            ],
            color,
            shape_type: SHAPE_CIRCLE,
            _pad: 0,
        }
    }

    pub fn from_button(button: &Button) -> Vec<Self> {
        let style = &button.style;
        let border = button.border_rect;
        let fill = button.fill;

        let border_color = [fill[0] * 0.55, fill[1] * 0.55, fill[2] * 0.55, 1.0];
        let face_color = [fill[0], fill[1], fill[2], 1.0];

        let mut instances = vec![
            Self::rect(border, border_color),
            Self::rect(
                Rect {
                    x: border.x + style.border.left,
                    y: border.y + style.border.top,
                    width: border.width - style.border.horizontal(),
                    height: border.height - style.border.vertical(),
                },
                face_color,
            ),
        ];

        crate::gpu_ui::text::append_text_instances(
            &mut instances,
            button.content_rect,
            &button.label,
        );

        instances
    }
}

pub fn circle_contains(center_x: f32, center_y: f32, diameter: f32, px: f32, py: f32) -> bool {
    let radius = diameter * 0.5;
    let dx = px - center_x;
    let dy = py - center_y;
    dx * dx + dy * dy <= radius * radius
}

pub fn as_bytes<T: Copy>(value: &T) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            (value as *const T).cast::<u8>(),
            std::mem::size_of::<T>(),
        )
    }
}

pub fn cast_slice<T: Copy>(values: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(values.as_ptr().cast::<u8>(), std::mem::size_of_val(values))
    }
}
