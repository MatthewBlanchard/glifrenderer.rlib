use num_traits::Float;
use skulpin::skia_safe::{
    Canvas, Font, FontMetrics, FontStyle, Paint, Path, Rect, TextBlob, Typeface, M44,
};

use crate::viewport::Viewport;
use std::cell::RefCell;
use std::collections::HashMap;

use super::constants::*;

pub static POINTFONTSIZE: f32 = 14.0;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Copy, Clone)]
pub struct UiString<'a> {
    pub string: &'a str,
    pub color: u32,
    pub bgcolor: Option<u32>,
    pub centered: Alignment,
    pub vcenter: VerticalAlignment,
    pub size: f32,
    pub padding: Option<f32>,
    pub autosized: AutoSizeMode,
    /// in degrees
    pub rotation: Option<f32>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AutoSizeMode {
    Full,
    OnlySmaller,
    None,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Bottom,
}

impl<'a> UiString<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            string: s,
            size: POINTFONTSIZE,
            bgcolor: Some(DEFAULT_STRING_BGCOLOR),
            color: DEFAULT_STRING_COLOR,
            centered: Alignment::Left,
            vcenter: VerticalAlignment::Bottom,
            padding: None,
            autosized: AutoSizeMode::Full,
            rotation: None,
        }
    }

    pub fn centered(s: &'a str) -> Self {
        let mut ret = Self::new(s);
        ret.centered = Alignment::Center;
        ret
    }

    pub fn with_colors(s: &'a str, color: u32, bgcolor: Option<u32>) -> Self {
        let mut ret = Self::new(s);
        ret.color = color;
        ret.bgcolor = bgcolor;
        ret
    }

    pub fn centered_with_colors(s: &'a str, color: u32, bgcolor: Option<u32>) -> Self {
        let mut ret = Self::with_colors(s, color, bgcolor);
        ret.centered = Alignment::Center;
        ret
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.centered = alignment;
        self
    }

    pub fn vcenter(mut self, vcenter: VerticalAlignment) -> Self {
        self.vcenter = vcenter;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn autosized(mut self, autosized: AutoSizeMode) -> Self {
        self.autosized = autosized;
        self
    }

    pub fn rotated(mut self, rotation: f32) -> Self {
        self.rotation = Some(rotation);
        self
    }
}

type MantissaExpSignTriplet = (u64, i16, i8);
trait IntegerEncode {
    fn integer_encode(mest: MantissaExpSignTriplet) -> f64 {
        let (mantissa, exponent, sign) = mest;
        let sign_f = sign as f64;
        let mantissa_f = mantissa as f64;
        let exponent_f = 2.0.powf(exponent as f64);
        sign_f * mantissa_f * exponent_f
    }
}
impl IntegerEncode for MantissaExpSignTriplet {}

// reply from skia::&Font::metrics()
type Metrics = (FontMetrics, f32); // f32 == line_height

// Creating the font is an expensive operation to do every frame. So, we keep a cache of fonts
// based on the current zoom.
thread_local! {
    // Font cache
    pub static POINTFONTS: RefCell<HashMap<MantissaExpSignTriplet, Font>> = {
        let mut h = HashMap::new();
        let font = pointfont_from_size(14.0);
        h.insert(POINTFONTSIZE.integer_decode(), font);
        RefCell::new(h)
    };

    // Textblob cache
    pub static TEXTBLOBS: RefCell<HashMap<(MantissaExpSignTriplet, String), (TextBlob, Rect, Metrics)>> = {
        RefCell::new(HashMap::new())
    }
}

pub fn pointfont_from_size(size: f32) -> Font {
    Font::from_typeface_with_params(
        Typeface::from_name("", FontStyle::bold()).expect("Failed to load bold font"),
        size,
        1.0,
        0.0,
    )
}

pub fn pointfont_from_size_and_factor(size: f32, factor: f32) -> Font {
    pointfont_from_size(size * (1. / factor))
}

pub fn viewport_font_size_scaled(viewport: &Viewport, size: Option<f32>) -> f32 {
    let size = size.unwrap_or(POINTFONTSIZE);
    size * 1. / viewport.factor
}

impl UiString<'_> {
    fn make_textblob(&self, size: f32) -> (TextBlob, Rect, Metrics) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);

        let (blob, rect, metrics) = POINTFONTS.with(|f| {
            let mut hm = f.borrow_mut();
            let f = hm.get(&size.integer_decode());
            let font = match f {
                Some(fon) => fon,
                None => {
                    hm.insert(size.integer_decode(), pointfont_from_size(size));
                    hm.get(&size.integer_decode()).unwrap()
                }
            };

            let (line_spacing, skmetrics) = font.metrics();
            let blob = TextBlob::from_str(self.string, font)
                .expect(&format!("Failed to shape {}", self.string));
            // here we ignore the width since we can just call rect.width()
            let (_, rect) = font.measure_str(self.string, Some(&paint));
            (blob, rect, (skmetrics, line_spacing))
        });
        (blob, rect, metrics)
    }
}

impl UiString<'_> {
    pub fn draw(&self, viewport: &Viewport, at: (f32, f32), canvas: &mut Canvas) {
        let matrix = canvas.local_to_device_as_3x3();
        let at = matrix.map_point(at);
        canvas.save();
        canvas.set_matrix(&M44::new_identity());
        let factor = viewport.factor;
        let mut paint = Paint::default();
        paint.set_color(self.color);
        paint.set_anti_alias(true);
        let size = if self.autosized == AutoSizeMode::None {
            self.size * factor
        } else if self.autosized == AutoSizeMode::OnlySmaller {
            if factor > 1. {
                self.size * factor
            } else {
                self.size
            }
        } else {
            // AutoSizeMode::Full
            self.size
        };

        let (blob, rect, metrics) = {
            TEXTBLOBS.with(|b| {
                let mut hm = b.borrow_mut();
                if hm.capacity() > 10_000 {
                    log::warn!("Text blob cache getting worryingly large (> 10k SkTextBlob's), culling it. This might be a bug, but is expected if you're generating many different strings w/Skia and can in that case be ignored.");
                    hm.clear();
                }
                let b = hm.get(&(size.integer_decode(), self.string.to_string()));
                match b {
                    Some((bb, rr, mm)) => (bb.clone(), rr.clone(), mm.clone()),
                    None => {
                        let (bb, rr, mm) = self.make_textblob(size);
                        hm.insert((size.integer_decode(), self.string.to_string()), (bb.clone(), rr.clone(), mm.clone()));
                        (bb, rr, mm)
                    }
                }
            })
        };
        let (metrics, line_spacing) = metrics;

        let padding = if let Some(padding) = self.padding {
            padding
        } else {
            0.
        };

        let mut height = line_spacing;
        let (hpadding, mut vpadding) = (padding, padding);
        vpadding += metrics.leading;
        match self.vcenter {
            VerticalAlignment::Bottom => {
                vpadding = -vpadding;
                vpadding *= 1.5;
                height = metrics.descent;
            }
            VerticalAlignment::Top => {
                height = -height;
            }
        }
        let center = match self.centered {
            Alignment::Left => 0.,
            Alignment::Right => (rect.width()),
            Alignment::Center => (rect.width() / 2.),
        };

        if let Some(angle) = self.rotation {
            canvas.rotate(-angle, Some(at.into()));
        }
        canvas.translate((-center, -height));
        canvas.translate((hpadding, vpadding));
        if self.vcenter == VerticalAlignment::Bottom {
            vpadding = vpadding * (2. / 3.);
        }

        if let Some(bgcolor) = self.bgcolor {
            let mut paint2 = Paint::default();
            paint2.set_color(bgcolor);
            paint2.set_anti_alias(true);
            let mut path = Path::new();
            let at_rect = Rect::from_point_and_size(
                (at.x + center, at.y + metrics.descent),
                (rect.width(), -line_spacing),
            )
            .with_outset((hpadding * 1.5, vpadding));
            path.add_rect(at_rect, None);
            path.close();
            canvas.draw_path(&path, &paint2);
        }

        canvas.draw_text_blob(&blob, at, &paint);
        canvas.restore();
    }
}
