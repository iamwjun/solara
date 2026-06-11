use crate::gpu_ui::layout::Rect;
use crate::gpu_ui::shapes::{ShapeInstance, SHAPE_RECT};

const CHAR_W: f32 = 8.0;
const CHAR_H: f32 = 8.0;

fn glyph(ch: char) -> Option<[u8; 8]> {
    Some(match ch {
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        'C' => [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
        'M' => [0x42, 0x66, 0x7E, 0x5A, 0x42, 0x42, 0x42, 0x00],
        'c' => [0x00, 0x00, 0x3C, 0x60, 0x60, 0x60, 0x3C, 0x00],
        'e' => [0x00, 0x00, 0x3C, 0x60, 0x7E, 0x60, 0x3C, 0x00],
        'i' => [0x00, 0x18, 0x08, 0x08, 0x08, 0x08, 0x1C, 0x00],
        'k' => [0x00, 0x42, 0x44, 0x78, 0x48, 0x44, 0x42, 0x00],
        'l' => [0x00, 0x18, 0x08, 0x08, 0x08, 0x08, 0x1C, 0x00],
        _ => return None,
    })
}

pub fn append_text_instances(instances: &mut Vec<ShapeInstance>, rect: Rect, text: &str) {
    let text_width = text.chars().count() as f32 * CHAR_W;
    let mut x = rect.x + ((rect.width - text_width) * 0.5).max(0.0);
    let y = rect.y + ((rect.height - CHAR_H) * 0.5).max(0.0);
    let color = [1.0, 1.0, 1.0, 1.0];

    for ch in text.chars() {
        if let Some(rows) = glyph(ch) {
            for (row, bits) in rows.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        instances.push(ShapeInstance {
                            pos_size: [
                                x + col as f32,
                                y + row as f32,
                                1.0,
                                1.0,
                            ],
                            color,
                            shape_type: SHAPE_RECT,
                            _pad: 0,
                        });
                    }
                }
            }
        }
        x += CHAR_W;
    }
}
