use skulpin::skia_safe::Canvas;

use crate::constants::*;
use crate::string::UiString;
use crate::viewport::Viewport;

pub fn draw_point_number(viewport: &Viewport, at: (f32, f32), number: isize, canvas: &mut Canvas) {
    let converted = number.to_string();
    let uis = UiString::new(&converted).padding(POINT_LABEL_PADDING);
    uis.draw(viewport, at, canvas);
}

pub fn draw_point_location(
    viewport: &Viewport,
    at: (f32, f32),
    original: (f32, f32),
    canvas: &mut Canvas,
) {
    let converted = format!("{}, {}", original.0 as i32, original.1 as i32);
    let uis = UiString::new(&converted).padding(POINT_LABEL_PADDING);
    uis.draw(viewport, at, canvas);
}
