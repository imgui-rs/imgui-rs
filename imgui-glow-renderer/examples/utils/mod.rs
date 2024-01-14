use std::num::NonZeroU32;

use glow::HasContext;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface},
};
use imgui_winit_support::WinitPlatform;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn create_window(
    title: &str,
    context_api: Option<ContextApi>,
) -> (
    EventLoop<()>,
    Window,
    Surface<WindowSurface>,
    PossiblyCurrentContext,
) {
    let event_loop = EventLoop::new().unwrap();

    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1024, 768));
    let (window, cfg) = glutin_winit::DisplayBuilder::new()
        .with_window_builder(Some(window_builder))
        .build(&event_loop, ConfigTemplateBuilder::new(), |mut configs| {
            configs.next().unwrap()
        })
        .expect("Failed to create OpenGL window");

    let window = window.unwrap();

    let mut context_attribs = ContextAttributesBuilder::new();
    if let Some(context_api) = context_api {
        context_attribs = context_attribs.with_context_api(context_api);
    }
    let context_attribs = context_attribs.build(Some(window.raw_window_handle()));
    let context = unsafe {
        cfg.display()
            .create_context(&cfg, &context_attribs)
            .expect("Failed to create OpenGL context")
    };

    let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new()
        .with_srgb(Some(true))
        .build(
            window.raw_window_handle(),
            NonZeroU32::new(1024).unwrap(),
            NonZeroU32::new(768).unwrap(),
        );
    let surface = unsafe {
        cfg.display()
            .create_window_surface(&cfg, &surface_attribs)
            .expect("Failed to create OpenGL surface")
    };

    let context = context
        .make_current(&surface)
        .expect("Failed to make OpenGL context current");

    surface
        .set_swap_interval(&context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        .expect("Failed to set swap interval");

    (event_loop, window, surface, context)
}

pub fn glow_context(context: &PossiblyCurrentContext) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function_cstr(|s| context.display().get_proc_address(s).cast())
    }
}

pub fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window,
        imgui_winit_support::HiDpiMode::Rounded,
    );

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context)
}

pub struct Triangler {
    pub program: <glow::Context as HasContext>::Program,
    pub vertex_array: <glow::Context as HasContext>::VertexArray,
}

impl Triangler {
    pub fn new(gl: &glow::Context, shader_header: &str) -> Self {
        const VERTEX_SHADER_SOURCE: &str = r#"
const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);

out vec2 vert;
out vec4 color;

vec4 srgb_to_linear(vec4 srgb_color) {
    // Calcuation as documented by OpenGL
    vec3 srgb = srgb_color.rgb;
    vec3 selector = ceil(srgb - 0.04045);
    vec3 less_than_branch = srgb / 12.92;
    vec3 greater_than_branch = pow((srgb + 0.055) / 1.055, vec3(2.4));
    return vec4(
        mix(less_than_branch, greater_than_branch, selector),
        srgb_color.a
    );
}

void main() {
    vert = verts[gl_VertexID];
    color = srgb_to_linear(vec4(vert, 0.5, 1.0));
    gl_Position = vec4(vert - 0.5, 0.0, 1.0);
}
"#;
        const FRAGMENT_SHADER_SOURCE: &str = r#"
in vec2 vert;
in vec4 color;

out vec4 frag_color;

vec4 linear_to_srgb(vec4 linear_color) {
    vec3 linear = linear_color.rgb;
    vec3 selector = ceil(linear - 0.0031308);
    vec3 less_than_branch = linear * 12.92;
    vec3 greater_than_branch = pow(linear, vec3(1.0/2.4)) * 1.055 - 0.055;
    return vec4(
        mix(less_than_branch, greater_than_branch, selector),
        linear_color.a
    );
}

void main() {
    frag_color = linear_to_srgb(color);
}
"#;

        let mut shaders = [
            (glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE, None),
            (glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE, None),
        ];

        unsafe {
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            let program = gl.create_program().expect("Cannot create program");

            for (kind, source, handle) in &mut shaders {
                let shader = gl.create_shader(*kind).expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", shader_header, *source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                *handle = Some(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for &(_, _, shader) in &shaders {
                gl.detach_shader(program, shader.unwrap());
                gl.delete_shader(shader.unwrap());
            }

            Self {
                program,
                vertex_array,
            }
        }
    }

    pub fn render(&self, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.05, 0.05, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }
}
