//! Skia renderer.

pub mod constants;
pub mod viewport;
pub mod guidelines;
pub mod points; // point drawing functions
pub mod toggles;
                // This imports calc_x, etc. which transforms coordinates between .glif and Skia
pub use points::calc::{calc_x, calc_y};
pub mod anchors;
pub mod glyph;
pub mod string;