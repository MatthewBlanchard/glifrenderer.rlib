use super::constants::*;
use super::string::UiString;
use crate::viewport::Viewport;

use glifparser::{glif::MFEKGlif, Anchor, PointData};
use skia_safe::{Canvas, Paint, PaintStyle, Path as SkPath};

pub fn draw_anchors<PD: PointData>(glyph: &MFEKGlif<PD>, viewport: &Viewport, canvas: &Canvas) {
    for anchor in &glyph.anchors {
        draw_anchor(&anchor, viewport, canvas);
    }
}

fn draw_anchor<PD: PointData>(anchor: &Anchor<PD>, viewport: &Viewport, canvas: &Canvas) {
    let mut path = SkPath::new();
    let (x, y) = (anchor.x, anchor.y);
    let radius = ANCHOR_RADIUS * (1. / viewport.factor);
    path.move_to((x - radius, y));
    path.quad_to((x, y), (x, y + radius));
    path.quad_to((x, y), (x + radius, y));
    path.quad_to((x, y), (x, y - radius));
    path.quad_to((x, y), (x - radius, y));
    path.close();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(ANCHOR_FILL);
    canvas.draw_path(&path, &paint);
    paint.set_style(PaintStyle::Stroke);
    paint.set_color(ANCHOR_STROKE);
    paint.set_stroke_width(ANCHOR_STROKE_THICKNESS * (1. / viewport.factor));
    canvas.draw_path(&path, &paint);
    if let Some(class) = &anchor.class {
        let uis =
            UiString::centered_with_colors(class, ANCHOR_NAME_COLOR, Some(ANCHOR_NAME_BGCOLOR));
        uis.draw(viewport, (x, y - (radius * 1.3)), canvas);
    }
}
