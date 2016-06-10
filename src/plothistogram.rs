use imgui_sys;
use super::{ImStr};
use imgui_sys::ImVec2;
use std::{f32, mem, ptr};
use libc::c_float;
#[must_use]
pub struct PlotHistogram<'p> {
    label: ImStr<'p>,
    values: &'p [f32],
    values_offset: usize,
    overlay_text: Option<ImStr<'p>>,
    scale_min: f32,
    scale_max: f32,
    graph_size: ImVec2,
}

impl<'p> PlotHistogram<'p> {
    pub fn new(label: ImStr<'p>, values: &'p [f32]) -> Self {
        PlotHistogram {
            label: label,
            values: values,
            values_offset: 0usize,
            overlay_text: None,
            scale_min: f32::MAX,
            scale_max: f32::MAX,
            graph_size: ImVec2::new(0.0f32, 0.0f32),
        }
    }

    #[inline]
    pub fn values_offset(self, values_offset: usize) -> Self {
        PlotHistogram { values_offset: values_offset, ..self }
    }

    #[inline]
    pub fn overlay_text(self, overlay_text: ImStr<'p>) -> Self {
        PlotHistogram { overlay_text: Some(overlay_text), ..self }
    }

    #[inline]
    pub fn scale_min(self, scale_min: f32) -> Self {
        PlotHistogram { scale_min: scale_min, ..self }
    }

    #[inline]
    pub fn scale_max(self, scale_max: f32) -> Self {
        PlotHistogram { scale_max: scale_max, ..self }
    }

    #[inline]
    pub fn graph_size(self, graph_size: ImVec2) -> Self {
        PlotHistogram { graph_size: graph_size, ..self }
    }

    pub fn build(self) {
        unsafe {
            imgui_sys::igPlotHistogram(self.label.as_ptr(),
                                   self.values.as_ptr() as *const c_float,
                                   self.values.len() as i32,
                                   self.values_offset as i32,
                                   self.overlay_text.map(|x| x.as_ptr()).unwrap_or(ptr::null()),
                                   self.scale_min,
                                   self.scale_max,
                                   self.graph_size,
                                   mem::size_of::<f32>() as i32);
        }
    }
}
