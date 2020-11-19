use std::marker::PhantomData;
use std::os::raw::c_float;
use std::{f32, mem, ptr};

use super::{ImStr, Ui};

#[must_use]
pub struct PlotHistogram<'ui, 'p> {
    label: &'p ImStr,
    values: &'p [f32],
    values_offset: usize,
    overlay_text: Option<&'p ImStr>,
    scale_min: f32,
    scale_max: f32,
    graph_size: [f32; 2],
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> PlotHistogram<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, values: &'p [f32]) -> Self {
        PlotHistogram {
            label,
            values,
            values_offset: 0usize,
            overlay_text: None,
            scale_min: f32::MAX,
            scale_max: f32::MAX,
            graph_size: [0.0, 0.0],
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
    pub fn graph_size(mut self, graph_size: [f32; 2]) -> Self {
        self.graph_size = graph_size;
        self
    }

    pub fn build(self) {
        unsafe {
            sys::igPlotHistogramFloatPtr(
                self.label.as_ptr(),
                self.values.as_ptr() as *const c_float,
                self.values.len() as i32,
                self.values_offset as i32,
                self.overlay_text.map(|x| x.as_ptr()).unwrap_or(ptr::null()),
                self.scale_min,
                self.scale_max,
                self.graph_size.into(),
                mem::size_of::<f32>() as i32,
            );
        }
    }
}

#[must_use]
pub struct PlotHistogramFn<'ui, 'p, T> {
    label: &'p ImStr,
    values_getter: fn(&mut T, usize) -> f32,
    data: &'p mut T,
    values_count: usize,
    values_offset: usize,
    overlay_text: Option<&'p ImStr>,
    scale_min: f32,
    scale_max: f32,
    graph_size: [f32; 2],
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p, T: 'p> PlotHistogramFn<'ui, 'p, T> {
    pub fn new(
        _: &Ui<'ui>,
        label: &'p ImStr,
        values_getter: fn(&mut T, usize) -> f32,
        data: &'p mut T,
        values_count: usize,
    ) -> Self {
        PlotHistogramFn {
            label,
            values_getter,
            data,
            values_count,
            values_offset: 0usize,
            overlay_text: None,
            scale_min: f32::MAX,
            scale_max: f32::MAX,
            graph_size: [0.0, 0.0],
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
    pub fn graph_size(mut self, graph_size: [f32; 2]) -> Self {
        self.graph_size = graph_size;
        self
    }

    pub fn build(mut self) {
        // see https://stackoverflow.com/questions/34691267/why-would-it-be-necessary-to-perform-two-casts-to-a-mutable-raw-pointer-in-a-row
        let self_ptr = &mut self as *mut _ as *mut _;
        unsafe {
            sys::igPlotHistogramFnFloatPtr(
                self.label.as_ptr(),
                Some(Self::internal_callback),
                self_ptr,
                self.values_count as i32,
                self.values_offset as i32,
                self.overlay_text.map(|x| x.as_ptr()).unwrap_or(ptr::null()),
                self.scale_min,
                self.scale_max,
                self.graph_size.into(),
            );
        }
    }

    extern "C" fn internal_callback(
        data: *mut ::std::os::raw::c_void,
        idx: ::std::os::raw::c_int,
    ) -> f32 {
        // see https://stackoverflow.com/questions/24191249/working-with-c-void-in-an-ffi
        let self_ref: &mut Self = unsafe { &mut *(data as *mut _) };
        (self_ref.values_getter)(self_ref.data, idx as usize)
    }
}
