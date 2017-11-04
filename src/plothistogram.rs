use sys;
use std::{f32, mem, ptr};
use std::marker::PhantomData;
use std::os::raw::c_float;

use super::{ImStr, ImVec2, Ui};

#[must_use]
pub struct PlotHistogram<'ui, 'p> {
    label: &'p ImStr,
    values: &'p [f32],
    values_offset: usize,
    overlay_text: Option<&'p ImStr>,
    scale_min: f32,
    scale_max: f32,
    graph_size: ImVec2,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> PlotHistogram<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, values: &'p [f32]) -> Self {
        PlotHistogram {
            label: label,
            values: values,
            values_offset: 0usize,
            overlay_text: None,
            scale_min: f32::MAX,
            scale_max: f32::MAX,
            graph_size: ImVec2::new(0.0f32, 0.0f32),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn values_offset(mut self, values_offset: usize) -> Self {
        self.values_offset = values_offset;
        self
    }

    #[inline]
    pub fn overlay_text(mut self, overlay_text: &'p ImStr) -> Self {
        self.overlay_text = Some(overlay_text);
        self
    }

    #[inline]
    pub fn scale_min(mut self, scale_min: f32) -> Self {
        self.scale_min = scale_min;
        self
    }

    #[inline]
    pub fn scale_max(mut self, scale_max: f32) -> Self {
        self.scale_max = scale_max;
        self
    }

    #[inline]
    pub fn graph_size<S: Into<ImVec2>>(mut self, graph_size: S) -> Self {
        self.graph_size = graph_size.into();
        self
    }

    pub fn build(self) {
        unsafe {
            sys::igPlotHistogram(
                self.label.as_ptr(),
                self.values.as_ptr() as *const c_float,
                self.values.len() as i32,
                self.values_offset as i32,
                self.overlay_text.map(|x| x.as_ptr()).unwrap_or(ptr::null()),
                self.scale_min,
                self.scale_max,
                self.graph_size,
                mem::size_of::<f32>() as i32,
            );
        }
    }
}
