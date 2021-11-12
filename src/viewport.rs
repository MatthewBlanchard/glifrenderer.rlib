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

pub fn redraw_viewport(view: &Viewport, canvas: &mut Canvas) {
    let mut matrix = Matrix::new_identity();
    let now_matrix = canvas.local_to_device_as_3x3();
    matrix.set_scale_translate((view.factor, view.factor), view.offset);

    if matrix != now_matrix {
        canvas.set_matrix(&matrix.into());
    }
}
