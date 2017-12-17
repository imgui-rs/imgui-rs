use gfx::format::{Format, Formatted, U8Norm};
use gfx::pso::buffer::{Element, ElemOffset, Structure};
use gfx::traits::Pod;
use std::mem;

use super::{ImDrawVert, ImVec2};

unsafe impl Pod for ImDrawVert {}

impl Structure<Format> for ImDrawVert {
    fn query(name: &str) -> Option<Element<Format>> {
        // array query hack from gfx_impl_struct_meta macro
        let (sub_name, big_offset) = {
            let mut split = name.split(|c| c == '[' || c == ']');
            let _ = split.next().unwrap();
            match split.next() {
                Some(s) => {
                    let array_id: ElemOffset = s.parse().unwrap();
                    let sub_name = match split.next() {
                        Some(s) if s.starts_with('.') => &s[1..],
                        _ => name,
                    };
                    (
                        sub_name,
                        array_id * (mem::size_of::<ImDrawVert>() as ElemOffset),
                    )
                }
                None => (name, 0),
            }
        };
        let dummy: &ImDrawVert = unsafe { mem::transmute(0usize) };
        match sub_name {
            "pos" => {
                Some(Element {
                    format: <ImVec2 as Formatted>::get_format(),
                    offset: unsafe { mem::transmute::<_, usize>(&dummy.pos) } as ElemOffset +
                        big_offset,
                })
            }
            "uv" => {
                Some(Element {
                    format: <ImVec2 as Formatted>::get_format(),
                    offset: unsafe { mem::transmute::<_, usize>(&dummy.uv) } as ElemOffset +
                        big_offset,
                })
            }
            "col" => {
                Some(Element {
                    format: <[U8Norm; 4] as Formatted>::get_format(),
                    offset: unsafe { mem::transmute::<_, usize>(&dummy.col) } as ElemOffset +
                        big_offset,
                })
            }
            _ => None,
        }
    }
}

gfx_format! {
    ImVec2: R32_G32 = Vec2<Float>
}
