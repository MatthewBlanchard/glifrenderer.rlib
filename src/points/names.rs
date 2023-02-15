use skia_safe::Canvas;

use crate::constants::*;
use crate::string::{UiString, AutoSizeMode};
use crate::toggles::PreviewMode;
use crate::viewport::Viewport;

pub fn draw_point_str(viewport: &Viewport, at: (f32, f32), s: &str, canvas: &mut Canvas) {
    if viewport.preview_mode == PreviewMode::Paper {
        return;
    }
    let uis = UiString::new(s).padding(POINT_LABEL_PADDING);
    uis.draw(viewport, at, canvas);
}

pub fn draw_point_name(viewport: &Viewport, at: (f32, f32), fill: u32, s: &str, canvas: &mut Canvas) {
    if viewport.preview_mode == PreviewMode::Paper {
        return;
    }
    let mut fill = fill.to_be_bytes();
    for b in fill.iter_mut().skip(1) {
        *b /= POINT_NAME_DARKEN;
    }
    let ufill = u32::from_be_bytes(fill);
    let uis = UiString::centered_with_colors(s, ufill, None).padding(POINT_LABEL_PADDING)
            .autosized(AutoSizeMode::OnlySmaller);
    uis.draw(viewport, at, canvas);
}

pub fn draw_point_number(viewport: &Viewport, at: (f32, f32), number: isize, canvas: &mut Canvas) {
    draw_point_str(viewport, at, &number.to_string(), canvas);
}

pub fn draw_point_location(
    viewport: &Viewport,
    at: (f32, f32),
    original: (f32, f32),
    canvas: &mut Canvas,
) {
    let s = format!("{}, {}", original.0 as i32, original.1 as i32);
    draw_point_str(viewport, at, &s, canvas);
}
