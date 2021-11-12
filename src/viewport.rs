use skulpin::skia_safe::{Canvas, Matrix};
use crate::toggles::{PointLabels, HandleStyle, PreviewMode};
#[derive(Clone, Debug)]
pub struct Viewport {
    pub winsize: (f32, f32),
    pub factor: f32,
    pub offset: (f32, f32),
    pub dpi: f64,
    pub point_labels: PointLabels,
    pub handle_style: HandleStyle,
    pub preview_mode: PreviewMode,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            winsize: (0., 0.),
            factor: 1.,
            offset: (0., 0.),
            dpi: 1.,
            point_labels: PointLabels::None,
            preview_mode: PreviewMode::None,
            handle_style: HandleStyle::Handlebars,
        }
    }
}

impl Viewport {
    pub fn with_winsize(mut self, winsize: (f32, f32)) -> Self {
        self.winsize = winsize;
        self
    }
    pub fn with_factor(mut self, factor: f32) -> Self {
        self.factor = factor;
        self
    }
    pub fn with_offset(mut self, offset: (f32, f32)) -> Self {
        self.offset = offset;
        self
    }
    pub fn with_dpi(mut self, dpi: f64) -> Self {
        self.dpi = dpi;
        self
    }
    pub fn with_point_labels(mut self, point_labels: PointLabels) -> Self {
        self.point_labels = point_labels;
        self
    }
    pub fn with_handle_style(mut self, handle_style: HandleStyle) -> Self {
        self.handle_style = handle_style;
        self
    }
    pub fn with_preview_mode(mut self, preview_mode: PreviewMode) -> Self {
        self.preview_mode = preview_mode;
        self
    }
}

pub fn redraw_viewport(view: &Viewport, canvas: &mut Canvas) {
    let mut matrix = Matrix::new_identity();
    let now_matrix = canvas.local_to_device_as_3x3();
    matrix.set_scale_translate((view.factor, view.factor), view.offset);

    if matrix != now_matrix {
        canvas.set_matrix(&matrix.into());
    }
}
