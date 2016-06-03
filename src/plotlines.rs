use imgui_sys;
use super::{ImStr};
use imgui_sys::ImVec2;
use std::{f32, mem, ptr};
use libc::c_float;
#[must_use]
pub struct PlotLines<'p> {
    label: ImStr<'p>,
    values: &'p [f32],
    values_ofset: i32,
    overlay_text: Option<ImStr<'p>>,
    scale_min: f32,
    scale_max: f32,
    graph_size: ImVec2,
    stride: i32,
}

impl<'p> PlotLines<'p> {
    pub fn new(label: ImStr<'p>, values: &'p [f32]) -> Self {
        PlotLines {
            label: label,
            values: values,
            values_ofset: 0i32,
            overlay_text: None,
            scale_min: f32::MAX,
            scale_max: f32::MAX,
            graph_size: ImVec2::new(0.0f32, 0.0f32),
            stride: mem::size_of::<f32>() as i32,
        }
    }

    #[inline]
    pub fn values_ofset(self, values_ofset: i32) -> Self {
        PlotLines { values_ofset: values_ofset, ..self }
    }

    #[inline]
    pub fn overlay_text(self, overlay_text: ImStr<'p>) -> Self {
        PlotLines { overlay_text: Some(overlay_text), ..self }
    }

    #[inline]
    pub fn scale_min(self, scale_min: f32) -> Self {
        PlotLines { scale_min: scale_min, ..self }
    }

    #[inline]
    pub fn scale_max(self, scale_max: f32) -> Self {
        PlotLines { scale_max: scale_max, ..self }
    }

    #[inline]
    pub fn graph_size(self, graph_size: ImVec2) -> Self {
        PlotLines { graph_size: graph_size, ..self }
    }

    #[inline]
    pub fn stride(self, stride: i32) -> Self {
        PlotLines { stride: stride, ..self }
    }

    pub fn build(self) {
        unsafe {
            imgui_sys::igPlotLines(self.label.as_ptr(),
                                   self.values.as_ptr() as *const c_float,
                                   self.values.len() as i32,
                                   self.values_ofset,
                                   self.overlay_text.map(|x| x.as_ptr()).unwrap_or(ptr::null()),
                                   self.scale_min,
                                   self.scale_max,
                                   self.graph_size,
                                   self.stride);
        }
    }
}
