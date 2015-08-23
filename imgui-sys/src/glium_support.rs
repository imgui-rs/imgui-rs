use glium::vertex::{Attribute, AttributeType};
use libc::c_float;
use std::mem;

unsafe impl Attribute for ImVec2 {
    fn get_type() -> AttributeType { <(c_float, c_float) as Attribute>::get_type() }
}

unsafe impl Attribute for ImVec4 {
    fn get_type() -> AttributeType {
        <(c_float, c_float, c_float, c_float) as Attribute>::get_type()
    }
}

impl Vertex for ImDrawVert {
    fn build_bindings() -> VertexFormat {
        unsafe {
            let dummy: &ImDrawVert = mem::transmute(0usize);
            Cow::Owned(vec![
                       ("pos".into(), mem::transmute(&dummy.pos), <ImVec2 as Attribute>::get_type()),
                       ("uv".into(), mem::transmute(&dummy.uv), <ImVec2 as Attribute>::get_type()),
                       ("col".into(), mem::transmute(&dummy.col), AttributeType::U8U8U8U8)
            ])
        }
    }
}
